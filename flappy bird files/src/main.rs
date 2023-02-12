mod components;

use legion::*;
use sdl2::EventPump;
use sdl2::image::{self, LoadTexture, InitFlag};
use sdl2::rect::{Rect, Point};
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;
use sdl2::render::{WindowCanvas, Texture};
use crate::components::*;

//defs
const PLAYER_MOVEMENT_SPEED: i32 = 2;
const BACKGROUND_WALLPAPER_MOVEMENT_SPEED: i32 = 1;
const SCREEN_HEIGHT: u32 = 600;
const SCREEN_WIDTH: u32 = 800;
const LOWER_NEG_SCREEN_WIDTH: i32 = -800;
const PERIODSLIMIT: u8 = 5;

fn update(world: &mut World, period: &mut u8){
    
    //frame update
    if period < &mut PERIODSLIMIT {*period += 1;}
    else{
        let mut query = <&mut Sprite>::query();
        for sprite in query.iter_mut(world) {
            if sprite.region.x() == 48 {sprite.region.x = 0;}
            else {sprite.region.x += 16;}
        }
        *period = 0;
    }

    //postion update
    let mut query = <(&mut Position, &Velocity)>::query();
    for (position, velocity) in query.iter_mut(world) {
        if velocity.direction.eq(&Direction::Up){
            position.0 = position.0.offset(0, -velocity.speed);
        }
        else if velocity.direction.eq(&Direction::Null){
            position.0 = position.0.offset(0, 0);//PLAYER_MOVEMENT_SPEED
        }
        else if velocity.direction.eq(&Direction::Movetoleft){
            if position.0.x() <= LOWER_NEG_SCREEN_WIDTH{
                position.0.x = SCREEN_WIDTH as i32 * 2;
            }
            position.0 = position.0.offset(-BACKGROUND_WALLPAPER_MOVEMENT_SPEED, 0);
        }
    }

}

fn process_events(event_pump: &mut EventPump, world: &mut World) -> bool {
    for event in event_pump.poll_iter() {
        match event {
            Event::Quit {..} |
            Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {return false;},
            Event::KeyDown { keycode: Some(Keycode::Space), repeat: false, .. } => {
                let mut query = <&mut Velocity>::query();

                //move bird up by five pixels
                for velocity in query.iter_mut(world) {
                    velocity.direction = Direction::Up;
                    velocity.speed = PLAYER_MOVEMENT_SPEED;
                }
            },
            Event::KeyUp { keycode: Some(Keycode::Space), repeat: false, .. } => {
                let mut query = <&mut Velocity>::query();

                //move bird up by five pixels
                for velocity in query.iter_mut(world) {
                    velocity.direction = Direction::Null;
                    velocity.speed = 0;
                }
            },
            _ => {}
        }
    }

    return true;
}

fn render(canvas: &mut WindowCanvas, textures: &[Texture], world: &mut World,) -> Result<(), String> {
    canvas.set_draw_color(Color::RGB(255, 255, 255));
    canvas.clear();

    let mut query_one = <(&Position, &Rect)>::query();

    for (position, rect) in query_one.iter_mut(world) {
        let screen_position = position.0 + Point::new(SCREEN_WIDTH as i32 / 2, SCREEN_HEIGHT as i32 / 2);
        let screen_rect = Rect::from_center(screen_position, SCREEN_WIDTH, SCREEN_HEIGHT);
        canvas.copy(&textures[0], *rect, screen_rect)?;
    }

    let (width, height) = canvas.output_size()?;

    let mut query = <(&Sprite, &Position)>::query();

    for (sprite, position) in query.iter_mut(world) {
        let current_frame = sprite.region;
        let screen_position = position.0 + Point::new(width as i32 / 2, height as i32 / 2);
        let screen_rect = Rect::from_center(screen_position, current_frame.width(), current_frame.height());
        canvas.copy(&textures[1], current_frame, screen_rect)?;
    }

    canvas.present();

    Ok(())
}

fn main() -> Result<(), String> {
    //inits
    let mut world = World::default();
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let _image_context = image::init(InitFlag::PNG | InitFlag::JPG)?;

    let window = video_subsystem.window("Flappy Bird", SCREEN_WIDTH, SCREEN_HEIGHT)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window
        .into_canvas()
        .software()
        .build()
        .map_err(|e| e.to_string())?;

    let texture_creator = canvas.texture_creator();
    
    let textures = [
        texture_creator.load_texture("assets/Background3.png")?,
        texture_creator.load_texture("assets/bird1.png")?,
    ];
    // First texture in textures array

    let _backgroundwallpaper: &[Entity] = world.extend(vec![
        (
            Position(Point::new(0, 0)), 
            Velocity{speed: PLAYER_MOVEMENT_SPEED, direction: Direction::Movetoleft},
            Rect::new(0, 0, SCREEN_WIDTH, SCREEN_WIDTH),
        ),
        (
            Position(Point::new(SCREEN_WIDTH as i32, 0)), 
            Velocity{speed: PLAYER_MOVEMENT_SPEED, direction: Direction::Movetoleft},
            Rect::new(0, 0, SCREEN_WIDTH, SCREEN_WIDTH),
        ),
        (
            Position(Point::new(SCREEN_WIDTH as i32 * 2, 0)), 
            Velocity{speed: PLAYER_MOVEMENT_SPEED, direction: Direction::Movetoleft},
            Rect::new(0, 0, SCREEN_WIDTH, SCREEN_WIDTH),
        ),
    ]);

    let _player: Entity = world
        .push((
            Position(Point::new(0, 0)), 
            Velocity{speed: 0, direction: Direction::Null},//TODO: change later from 0 to (-1 * PLAYER_MOVEMENT_SPEED)
            Sprite{spritesheet: 0, region: Rect::new(0, 0, 16, 16),}
        ));

    let mut event_pump = sdl_context.event_pump()?;
    let mut period: u8 = 0;

    'running: loop {
        //process inputs
        if process_events(&mut event_pump, &mut world) == false { break 'running; }
        // The rest of the game loop goes here...

        // update
        update( &mut world, &mut period);
        //TODO: Do with specs!

        //render
        render(&mut canvas, &textures, &mut world)?;
        
        // Time management!
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    Ok(())
}