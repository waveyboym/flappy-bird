use legion::*;
use sdl2::rect::{Rect, Point};
use sdl2::pixels::Color;
use sdl2::render::{WindowCanvas, Texture};
use crate::components::*;
use crate::constants::*;

pub fn render(canvas: &mut WindowCanvas, textures: &[Texture], world: &mut World,) -> Result<(), String> {
    canvas.set_draw_color(Color::RGB(255, 255, 255));
    canvas.clear();

    let mut query_one = <(&Position, &Rect, &EntityType)>::query();

    for (position, rect, entity_type) in query_one.iter_mut(world) {
        if entity_type.0.eq(&Entitytype::Wallpaper){
            let screen_position = position.0 + Point::new(SCREEN_WIDTH as i32 / 2, SCREEN_HEIGHT as i32 / 2);
            let screen_rect = Rect::from_center(screen_position, SCREEN_WIDTH, SCREEN_HEIGHT);
            canvas.copy(&textures[0], *rect, screen_rect)?;
        }
    }

    let (width, height) = canvas.output_size()?;

    let mut query = <(&Sprite, &Position, &EntityType)>::query();

    for (sprite, position, entity_type) in query.iter_mut(world) {
        if entity_type.0.eq(&Entitytype::Player){
            let current_frame = sprite.region;
            let screen_position = position.0 + Point::new(width as i32 / 2, height as i32 / 2);
            let screen_rect = Rect::from_center(screen_position, current_frame.width(), current_frame.height());
            canvas.copy(&textures[1], current_frame, screen_rect)?;
        }
    }

    canvas.present();

    Ok(())
}
