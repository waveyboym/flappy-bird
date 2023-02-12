use legion::*;
use sdl2::{EventPump};
use sdl2::image::{self, LoadTexture, InitFlag};
use sdl2::rect::{Rect, Point};
use std::time::Duration;
use sdl2::render::{WindowCanvas, Texture};
use crate::components::*;
use crate::constants::*;
use crate::update::update;
use crate::events::process_events;
use crate::render::render;

pub fn init_and_start_run() -> Result<(), String>{
    //variables exist
    let mut world = World::default();
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let _image_context = image::init(InitFlag::PNG | InitFlag::JPG)?;
    let mut event_pump = sdl_context.event_pump()?;
    let mut period: u8 = 0;
    
    
    //inits
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


    //entities in world
    let _backgroundwallpaper: &[Entity] = world.extend(vec![
        (
            EntityType(Entitytype::Wallpaper),
            Position(Point::new(0, 0)), 
            Velocity{speed: PLAYER_MOVEMENT_SPEED, direction: Direction::Movetoleft},
            Rect::new(0, 0, SCREEN_WIDTH, SCREEN_WIDTH),
        ),
        (
            EntityType(Entitytype::Wallpaper),
            Position(Point::new(SCREEN_WIDTH as i32, 0)), 
            Velocity{speed: PLAYER_MOVEMENT_SPEED, direction: Direction::Movetoleft},
            Rect::new(0, 0, SCREEN_WIDTH, SCREEN_WIDTH),
        ),
        (
            EntityType(Entitytype::Wallpaper),
            Position(Point::new(SCREEN_WIDTH as i32 * 2, 0)), 
            Velocity{speed: PLAYER_MOVEMENT_SPEED, direction: Direction::Movetoleft},
            Rect::new(0, 0, SCREEN_WIDTH, SCREEN_WIDTH),
        ),
    ]);

    let _player: Entity = world
        .push((
            EntityType(Entitytype::Player),
            Position(Point::new(0, 0)), 
            Velocity{speed: 0, direction: Direction::Still},//TODO: change later from 0 to (-1 * PLAYER_MOVEMENT_SPEED)
            Sprite{spritesheet: 0, region: Rect::new(0, 0, 16, 16),}
        ));

    run_application(&mut world, &mut period, &mut event_pump, &mut canvas, &textures)?;
    Ok(())
}

fn run_application(
    world: &mut World, 
    period: &mut u8,
    event_pump: &mut EventPump,
    canvas: &mut WindowCanvas,
    textures: &[Texture],
) -> Result<(), String>{
    'running: loop {
        //process inputs
        if process_events(event_pump, world) == false { break 'running; }
        // The rest of the game loop goes here...

        // update
        update(world, period);
        //TODO: Do with specs!

        //render
        render(canvas, &textures, world)?;
        
        // Time management!
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
    Ok(())
}
