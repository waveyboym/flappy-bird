use legion::*;
use sdl2::rect::{Rect, Point};
use sdl2::pixels::Color;
use sdl2::render::{WindowCanvas, Texture};
use crate::components::*;
use crate::constants::*;

fn get_index(entity_type: &EntityType) -> usize {
    if entity_type.0.eq(&Entitytype::Wallpaper){return 0;}
    else if entity_type.0.eq(&Entitytype::Player){return 1;}
    else if entity_type.0.eq(&Entitytype::Pipegreen){return 2;}
    else if entity_type.0.eq(&Entitytype::Pipeyellow){return 3;}
    else {return 4;}//else if entity_type.0.eq(&Entitytype::Landscape){return 3;}
}

pub fn render(canvas: &mut WindowCanvas, textures: &[Texture], world: &mut World,) -> Result<(), String> {
    canvas.set_draw_color(Color::RGB(REDVAL, GREENVAL, BLUEVAL));
    canvas.clear();

    let mut query_one = <(&Position, &Rect, &EntityType)>::query();

    //ASSETS DRAW CALL
    for (position, rect, entity_type) in query_one.iter_mut(world) {
        if entity_type.0.eq(&Entitytype::Pipegreen) || entity_type.0.eq(&Entitytype::Pipeyellow) ||
        entity_type.0.eq(&Entitytype::Wallpaper) || entity_type.0.eq(&Entitytype::Landscape){
            let screen_position = position.0 + Point::new(SCREEN_WIDTH as i32 / 2, SCREEN_HEIGHT as i32 / 2);
            let screen_rect = Rect::from_center(screen_position, rect.width(), rect.height());
            canvas.copy(&textures[get_index(&entity_type)], *rect, screen_rect)?;
        }
    }


    //PLAYER DRAW CALL
    let mut query = <(&Sprite, &Position, &EntityType)>::query();

    for (sprite, position, entity_type) in query.iter_mut(world) {
        if entity_type.0.eq(&Entitytype::Player){
            let current_frame = sprite.region;
            let screen_position = position.0 + Point::new(SCREEN_WIDTH as i32 / 2, SCREEN_HEIGHT as i32 / 2);
            let screen_rect = Rect::from_center(screen_position, current_frame.width(), current_frame.height());
            canvas.copy(&textures[1], current_frame, screen_rect)?;
        }
    }

    canvas.present();

    Ok(())
}
