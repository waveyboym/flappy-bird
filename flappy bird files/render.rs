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
    else {return 4;}//else if entity_type.0.eq(&Entitytype::Landscape){return 4;}
}

pub fn render(
    canvas: &mut WindowCanvas, 
    textures: &[Texture], 
    world: &mut World, 
    current_score: &mut u32,
    game_over: &mut bool,
) -> Result<(), String> {
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

    //display score text
    let mut text: String = "score: ".to_owned();
    
    text.push_str(&current_score.to_string());
    render_text(canvas, &textures, &text, 0, 0)?;

    if *game_over == true {
        //display game over text
        let mut gameover_text: String = "Your score is: ".to_owned();
    
        gameover_text.push_str(&current_score.to_string());
        render_text_center_screen(canvas, &textures, "GAME OVER", -(SCREEN_WIDTH as i32 / 3), -48)?;
        render_text_center_screen(canvas, &textures, &gameover_text, -(SCREEN_WIDTH as i32 / 3), 0)?;
        render_text_center_screen(canvas, &textures, "Press R/r to", -(SCREEN_WIDTH as i32 / 3), 48)?;
        render_text_center_screen(canvas, &textures, "restart the game", -(SCREEN_WIDTH as i32 / 3), 96)?;
    }

    canvas.present();

    Ok(())
}

fn render_text(canvas: &mut WindowCanvas, textures: &[Texture], text: &str, x_pos: i32, y_pos: i32) -> Result<(), String>{
    let mut index = 0;
    for character in text.chars() {
        let current_frame: Rect = Rect::new(get_xcoord_of(&character) * 32, get_ycoord_of(&character) * 32, 32, 32);
        let screen_position = Point::new(x_pos + ((index + 1) * 32), y_pos + 32);
        let screen_rect = Rect::from_center(screen_position, 32, 32);
        canvas.copy(&textures[5], current_frame, screen_rect)?;
        index += 1;
    }

    Ok(())
}

fn render_text_center_screen(
    canvas: &mut WindowCanvas, 
    textures: &[Texture], 
    text: &str, 
    x_pos: i32, 
    y_pos: i32
) -> Result<(), String>{
    let mut index = 0;
    for character in text.chars() {
        let current_frame: Rect = Rect::new(get_xcoord_of(&character) * 32, get_ycoord_of(&character) * 32, 32, 32);
        let screen_position = Point::new( (SCREEN_WIDTH as i32 / 2) + x_pos + ((index + 1) * 32), y_pos + SCREEN_HEIGHT as i32 / 2);
        let screen_rect = Rect::from_center(screen_position, 32, 32);
        canvas.copy(&textures[5], current_frame, screen_rect)?;
        index += 1;
    }

    Ok(())
}

fn get_ycoord_of(character_y: &char) -> i32{
    if *character_y == '0' || *character_y == '.' || *character_y == '/'{return 0;}
    else if *character_y == '1' || *character_y == '2' || *character_y == '3' || *character_y == '4'
        || *character_y == '5' || *character_y == '6' || *character_y == '7' || *character_y == '8'
        || *character_y == '9' || *character_y == ':'{
        return 1;
    }
    else if *character_y == 'A' || *character_y == 'B' || *character_y == 'C' || *character_y == 'D'
        || *character_y == 'E' || *character_y == 'F' || *character_y == 'G' || *character_y == 'H'
        || *character_y == 'I' || *character_y == 'J' || *character_y == 'K' || *character_y == 'L' 
        || *character_y == 'M' || *character_y == 'N' || *character_y == 'O' || *character_y == 'P'{
        return 2;
    }
    else if *character_y == 'Q' || *character_y == 'R' || *character_y == 'S' || *character_y == 'T'
        || *character_y == 'U' || *character_y == 'V' || *character_y == 'W' || *character_y == 'X'
        || *character_y == 'Y' || *character_y == 'Z'{
        return 3;
    }
    else if *character_y == 'a' || *character_y == 'b' || *character_y == 'c' || *character_y == 'd'
        || *character_y == 'e' || *character_y == 'f' || *character_y == 'g' || *character_y == 'h'
        || *character_y == 'i' || *character_y == 'j' || *character_y == 'k' || *character_y == 'l' 
        || *character_y == 'm' || *character_y == 'n' || *character_y == 'o' || *character_y == 'p'{
        return 4;
    }
    else if *character_y == 'q' || *character_y == 'r' || *character_y == 's' || *character_y == 't'
        || *character_y == 'u' || *character_y == 'v' || *character_y == 'w' || *character_y == 'x'
        || *character_y == 'y' || *character_y == 'z'{
        return 5;
    }
    else {return 6;}//out of bounds so blank space will display
}

fn get_xcoord_of(character_x: &char) -> i32{//to 16
    if *character_x == '1' || *character_x == 'A' || *character_x == 'Q' || *character_x == 'a'
    || *character_x == 'q' {
        return 0;
    }
    else if *character_x == '2' || *character_x == 'B' || *character_x == 'R' || *character_x == 'b'
    || *character_x == 'r'{
        return 1;
    }
    else if *character_x == '3' || *character_x == 'C' || *character_x == 'S' || *character_x == 'c'
    || *character_x == 's'{
        return 2;
    }
    else if *character_x == '4' || *character_x == 'D' || *character_x == 'T' || *character_x == 'd'
    || *character_x == 't'{
        return 3;
    }
    else if *character_x == '5' || *character_x == 'E' || *character_x == 'U' || *character_x == 'e'
    || *character_x == 'u'{
        return 4;
    }
    else if *character_x == '6' || *character_x == 'F' || *character_x == 'V' || *character_x == 'f'
    || *character_x == 'v'{
        return 5;
    }
    else if *character_x == '7' || *character_x == 'G' || *character_x == 'W' || *character_x == 'g'
    || *character_x == 'w'{
        return 6;
    }
    else if *character_x == '8' || *character_x == 'H' || *character_x == 'X' || *character_x == 'h'
    || *character_x == 'x'{
        return 7;
    }
    else if *character_x == '9' || *character_x == 'I' || *character_x == 'Y' || *character_x == 'i'
    || *character_x == 'y'{
        return 8;
    }
    else if *character_x == 'J' || *character_x == 'Z' || *character_x == 'j' || *character_x == 'z'{
        return 9;
    }
    else if *character_x == 'K' || *character_x == 'k'{
        return 10;
    }
    else if *character_x == 'L' || *character_x == 'l'{
        return 11;
    }
    else if *character_x == 'M' || *character_x == 'm'{
        return 12;
    }
    else if *character_x == 'N' || *character_x == 'n' || *character_x == '.'{
        return 13;
    }
    else if *character_x == 'O' || *character_x == 'o' || *character_x == '/'{
        return 14;
    }
    else if *character_x == '0' || *character_x == 'P' || *character_x == 'p'{
        return 15;
    }
    else {return 16;}//out of bounds so blank space will display
}