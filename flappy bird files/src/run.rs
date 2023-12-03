use legion::*;
use sdl2::EventPump;
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

    //game status data
    let mut period: u8 = 0;
    let mut current_score: u32 = 0;
    let mut game_over: bool = false;
    let mut restart_game: bool = false;
    
    
    //inits
    let window = video_subsystem.window("Flappy Bird", SCREEN_WIDTH, SCREEN_HEIGHT)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;
    
    //let icon: AsRef<SurfaceRef>;

    //window.set_icon(icon);
    
    let mut canvas = window
        .into_canvas()
        .software()
        .build()
        .map_err(|e| e.to_string())?;
    
    let texture_creator = canvas.texture_creator();
    let textures = [
        texture_creator.load_texture("assets/Background5.png")?,//background
        texture_creator.load_texture("assets/bird1.png")?,//bird/player
        texture_creator.load_texture("assets/pipegreen.png")?,//green pipe
        texture_creator.load_texture("assets/pipeyellow.png")?,//yellow pipe
        texture_creator.load_texture("assets/landscape.png")?,//landscape
        texture_creator.load_texture("assets/Forbin font.png")?,//landscape
    ];

    add_static_entities_to_world(&mut world);

    run_application(
        &mut world, 
        &mut period, 
        &mut event_pump, 
        &mut canvas, 
        &textures, 
        &mut current_score,
        &mut game_over,
        &mut restart_game,
    )?;
    Ok(())
}

fn add_static_entities_to_world(world: &mut World, ){
    //entities in world

    //player entity
    world.push((
            EntityType(Entitytype::Player),
            Position(Point::new(0, 0)), 
            Velocity{speed: PLAYER_MOVEMENT_SPEED, direction: Direction::Still},
            Sprite{spritesheet: 0, region: Rect::new(0, 0, PLAYER_SPRITE_SIZE_WL, PLAYER_SPRITE_SIZE_WL),}
        ));

    //wallpaper entity
    world.extend(vec![
        (
            EntityType(Entitytype::Wallpaper),
            Position(Point::new(0, 0)), 
            Velocity{speed: BACKGROUND_WALLPAPER_MOVEMENT_SPEED, direction: Direction::Movebgleft},
            Rect::new(0, 0, SCREEN_WIDTH, SCREEN_HEIGHT),
        ),
        (
            EntityType(Entitytype::Wallpaper),
            Position(Point::new(SCREEN_WIDTH as i32, 0)), 
            Velocity{speed: BACKGROUND_WALLPAPER_MOVEMENT_SPEED, direction: Direction::Movebgleft},
            Rect::new(0, 0, SCREEN_WIDTH, SCREEN_HEIGHT),
        ),
        (
            EntityType(Entitytype::Wallpaper),
            Position(Point::new(SCREEN_WIDTH as i32 * 2, 0)), 
            Velocity{speed: BACKGROUND_WALLPAPER_MOVEMENT_SPEED, direction: Direction::Movebgleft},
            Rect::new(0, 0, SCREEN_WIDTH, SCREEN_HEIGHT),
        ),
    ]);
    
    //pipe entity
    world.extend(vec![
        (
            EntityType(Entitytype::Pipegreen),
            Position(Point::new(0, BTM_ADDBASE_VAL as i32)), 
            Velocity{speed: PLAYER_MOVEMENT_SPEED, direction: Direction::Still},
            Rect::new(0, 0, PIPE_WIDTH, PIPE_HEIGHT),
        ),
        (
            EntityType(Entitytype::Pipegreen),
            Position(Point::new(0, -(SCREEN_HEIGHT as i32 / 2) - TOP_SUBBASE_VAL as i32)), 
            Velocity{speed: PLAYER_MOVEMENT_SPEED, direction: Direction::Still},
            Rect::new(0, 0, PIPE_WIDTH, PIPE_HEIGHT),
        ),
        (
            EntityType(Entitytype::Pipeyellow),
            Position(Point::new(PIPE_WIDTH as i32 + X_SEPARATION as i32, BTM_ADDBASE_VAL as i32)), 
            Velocity{speed: PLAYER_MOVEMENT_SPEED, direction: Direction::Still},
            Rect::new(0, 0, PIPE_WIDTH, PIPE_HEIGHT),
        ),
        (
            EntityType(Entitytype::Pipeyellow),
            Position(Point::new(PIPE_WIDTH as i32 + X_SEPARATION as i32, -(SCREEN_HEIGHT as i32 / 2) - TOP_SUBBASE_VAL as i32)), 
            Velocity{speed: PLAYER_MOVEMENT_SPEED, direction: Direction::Still},
            Rect::new(0, 0, PIPE_WIDTH, PIPE_HEIGHT),
        ),
        (
            EntityType(Entitytype::Pipegreen),
            Position(Point::new(PIPE_WIDTH as i32 * 2 + X_SEPARATION as i32 * 2, BTM_ADDBASE_VAL as i32)), 
            Velocity{speed: PLAYER_MOVEMENT_SPEED, direction: Direction::Still},
            Rect::new(0, 0, PIPE_WIDTH, PIPE_HEIGHT),
        ),
        (
            EntityType(Entitytype::Pipegreen),
            Position(Point::new(PIPE_WIDTH as i32 * 2 + X_SEPARATION as i32 * 2, -(SCREEN_HEIGHT as i32 / 2) - TOP_SUBBASE_VAL as i32)), 
            Velocity{speed: PLAYER_MOVEMENT_SPEED, direction: Direction::Still},
            Rect::new(0, 0, PIPE_WIDTH, PIPE_HEIGHT),
        ),
    ]);

    //landscape entity
    world.extend(vec![
        (
            EntityType(Entitytype::Landscape),
            Position(Point::new(-(SCREEN_WIDTH as i32 / 2) + (LANDSCAPE_WIDTH as i32 / 2), BTM_LANDSCAPE_POS)), 
            //Velocity{speed: PLAYER_MOVEMENT_SPEED, direction: Direction::Movepolesleft},
            Rect::new(0, 0, LANDSCAPE_WIDTH, (LANDSCAPE_WIDTH / 4) - 1),
        ),
        (
            EntityType(Entitytype::Landscape),
            Position(Point::new((-(SCREEN_WIDTH as i32 / 2) + (LANDSCAPE_WIDTH as i32 / 2)) + LANDSCAPE_WIDTH as i32, BTM_LANDSCAPE_POS)), 
            //Velocity{speed: PLAYER_MOVEMENT_SPEED, direction: Direction::Movepolesleft},
            Rect::new(0, 0, LANDSCAPE_WIDTH, (LANDSCAPE_WIDTH / 4) - 1),
        ),
        (
            EntityType(Entitytype::Landscape),
            Position(Point::new((-(SCREEN_WIDTH as i32 / 2) + (LANDSCAPE_WIDTH as i32 / 2)) + (LANDSCAPE_WIDTH as i32 * 2), BTM_LANDSCAPE_POS)), 
            //Velocity{speed: PLAYER_MOVEMENT_SPEED, direction: Direction::Movepolesleft},
            Rect::new(0, 0, LANDSCAPE_WIDTH, (LANDSCAPE_WIDTH / 4) - 1),
        ),
        (
            EntityType(Entitytype::Landscape),
            Position(Point::new((-(SCREEN_WIDTH as i32 / 2) + (LANDSCAPE_WIDTH as i32 / 2)) + (LANDSCAPE_WIDTH as i32 * 3), BTM_LANDSCAPE_POS)), 
            //Velocity{speed: PLAYER_MOVEMENT_SPEED, direction: Direction::Movepolesleft},
            Rect::new(0, 0, LANDSCAPE_WIDTH, (LANDSCAPE_WIDTH / 4) - 1),
        ),
        (
            EntityType(Entitytype::Landscape),
            Position(Point::new((-(SCREEN_WIDTH as i32 / 2) + (LANDSCAPE_WIDTH as i32 / 2)) + (LANDSCAPE_WIDTH as i32 * 4), BTM_LANDSCAPE_POS)), 
            //Velocity{speed: PLAYER_MOVEMENT_SPEED, direction: Direction::Movepolesleft},
            Rect::new(0, 0, LANDSCAPE_WIDTH, (LANDSCAPE_WIDTH / 4) - 1),
        ),
        (
            EntityType(Entitytype::Landscape),
            Position(Point::new((-(SCREEN_WIDTH as i32 / 2) + (LANDSCAPE_WIDTH as i32 / 2)) + (LANDSCAPE_WIDTH as i32 * 5), BTM_LANDSCAPE_POS)), 
            //Velocity{speed: PLAYER_MOVEMENT_SPEED, direction: Direction::Movepolesleft},
            Rect::new(0, 0, LANDSCAPE_WIDTH, (LANDSCAPE_WIDTH / 4) - 1),
        ),
        (
            EntityType(Entitytype::Landscape),
            Position(Point::new((-(SCREEN_WIDTH as i32 / 2) + (LANDSCAPE_WIDTH as i32 / 2)) + (LANDSCAPE_WIDTH as i32 * 6), BTM_LANDSCAPE_POS)), 
            //Velocity{speed: PLAYER_MOVEMENT_SPEED, direction: Direction::Movepolesleft},
            Rect::new(0, 0, LANDSCAPE_WIDTH, (LANDSCAPE_WIDTH / 4) - 1),
        )
    ]);


}

fn run_application(
    world: &mut World, 
    period: &mut u8,
    event_pump: &mut EventPump,
    canvas: &mut WindowCanvas,
    textures: &[Texture],
    current_score: &mut u32,
    game_over: &mut bool,
    restart_game: &mut bool,
) -> Result<(), String>{
    'running: loop {
        //process inputs
        if process_events(event_pump, world, restart_game) == false { break 'running; }
        // The rest of the game loop goes here...

        // update
        update(world, period, current_score, game_over, restart_game);

        //render
        render(canvas, &textures, world, current_score, game_over)?;
        
        // Time management!
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
    Ok(())
}
