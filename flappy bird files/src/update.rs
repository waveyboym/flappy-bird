use legion::*;
use crate::components::*;
use sdl2::rect::Rect;
use crate::constants::*;

pub fn update(
    world: &mut World, 
    period: &mut u8, 
    current_score: &mut u32, 
    game_over: &mut bool, 
    restart_game: &mut bool,
){
    //frame update for player ONLY
    if *period < PERIODSLIMIT {*period += 1;}
    else{
        update_player_frame(world);
        if *game_over == false{
            *current_score += 5 + *period as u32;
        }
        *period = 0;
    }

    //check collisions
    if *game_over == false{
        fetch_pipe_entities(world, game_over);
    }

    if *game_over == true && *restart_game == true{
        reset_world(world);
        *current_score = 0;
        *game_over = false;
        *restart_game = false;
    }

    //postion update
    if *game_over == false{
        update_entity_positions(world, current_score);
    }
}

fn update_player_frame(world: &mut World) {
    let mut spritequery = <(&mut Sprite, &EntityType)>::query();

    for (sprite, entity_type) in spritequery.iter_mut(world){
        if entity_type.0.eq(&Entitytype::Player){
            if sprite.region.x() == (3 * PLAYER_SPRITE_SIZE_WL as i32) {sprite.region.x = 0;}
            else {sprite.region.x += PLAYER_SPRITE_SIZE_WL as i32;}
            break;
        }
    }
}

fn fetch_pipe_entities(world: &mut World, game_over: &mut bool,){
    let mut x_pos = 0;
    let mut y_pos = 0;

    let mut positionquery = <(&Position, &EntityType)>::query();
    for (position, entity_type) in positionquery.iter_mut(world){
        if entity_type.0.eq(&Entitytype::Player){
            x_pos = position.0.x();
            y_pos = position.0.y();
            break;
        }
    }

    let mut rectquery = <(&Rect, &EntityType, &Position)>::query();
    for (rect, entity_type, position) in rectquery.iter_mut(world) {
        if entity_type.0.eq(&Entitytype::Pipegreen) || entity_type.0.eq(&Entitytype::Pipeyellow){
            if check_collision(&mut x_pos, &mut y_pos, &rect, &position){
                *game_over = true;
                break;
            }
        }
    }

}

fn update_entity_positions(world: &mut World, current_score: &mut u32,){
    let mut positionquery = <(&mut Position, &Velocity)>::query();
    for (position, velocity) in positionquery.iter_mut(world) {
        if velocity.direction.eq(&Direction::Up){
            if !(position.0.y() <= (EDGEOFSCREEN_YAXIS - (SCREEN_HEIGHT as i32 / 2))) {
                position.0 = position.0.offset(0, -velocity.speed);
            }
        }
        else if velocity.direction.eq(&Direction::Null){
            if !(position.0.y() >= (SCREEN_HEIGHT as i32/ 2) - EDGEOFSCREEN_YAXIS) {
                position.0 = position.0.offset(0, velocity.speed);
            }
        }
        else if velocity.direction.eq(&Direction::Movebgleft){
            if position.0.x() <= LOWER_NEG_SCREEN_WIDTH{
                position.0.x = SCREEN_WIDTH as i32 * 2;
            }
            position.0 = position.0.offset(-velocity.speed, 0);
        }
        else if velocity.direction.eq(&Direction::Movepolesleft){
            if position.0.x() <= -((SCREEN_WIDTH as i32 / 2) + (PIPE_WIDTH as i32 / 2)){
                position.0.x = (SCREEN_WIDTH as i32 / 2) + (PIPE_WIDTH as i32 / 2);
            }
            position.0 = position.0.offset(-velocity.speed, 0);
        }
        else{//if velocity.direction.eq(&Direction::Still) 
            *current_score = 0;
            continue;
        }
    }

}

fn reset_world(world: &mut World){
    let mut query = <(&mut Position, &EntityType, &mut Velocity)>::query();

    for (position, entity_type, velocity) in query.iter_mut(world) {
        if entity_type.0.eq(&Entitytype::Player){
            position.0.x = 0;
            position.0.y = 0;
            velocity.direction = Direction::Still;
            break;
        }
    }

    let mut index = 0;
    let mut multiplier = 0;

    for (position, entity_type, velocity) in query.iter_mut(world) {
        if entity_type.0.eq(&Entitytype::Pipegreen) || entity_type.0.eq(&Entitytype::Pipeyellow){
            position.0.x = PIPE_WIDTH as i32 * multiplier + X_SEPARATION as i32 * multiplier;
            if index % 2 == 0 {
                position.0.y = BTM_ADDBASE_VAL as i32;
            }
            else{
                position.0.y = -(SCREEN_HEIGHT as i32 / 2) - TOP_SUBBASE_VAL as i32;
            }
            index += 1;
            velocity.direction = Direction::Still;

            if index % 2 == 0 { multiplier += 1; }
        }
    }
}

///collision detection checker code
fn get_box_a_xval(x_pos: &mut i32) -> (i32, i32){
    let left_a = if *x_pos >= 0{
            (SCREEN_WIDTH as i32 / 2) + *x_pos
        }
        else{
            (SCREEN_WIDTH as i32 / 2) - *x_pos
        };

    let right_a = if *x_pos >= 0{
            *x_pos + PLAYER_SPRITE_SIZE_WL as i32 + (SCREEN_WIDTH as i32 / 2)
        }
        else {
            (PLAYER_SPRITE_SIZE_WL as i32 + (SCREEN_WIDTH as i32 / 2)) - *x_pos
        };

    (left_a, right_a)
}

fn get_box_a_yval(y_pos: &mut i32) -> (i32, i32){
    let top_a = if *y_pos >= 0 {
            (SCREEN_HEIGHT as i32 / 2) + *y_pos
        }
        else{
            (SCREEN_HEIGHT as i32 / 2) - *y_pos
        };


    let bottom_a = if *y_pos >= 0 {
            *y_pos + PLAYER_SPRITE_SIZE_WL as i32 + (SCREEN_HEIGHT as i32 / 2)
        }
        else{
            (PLAYER_SPRITE_SIZE_WL as i32 + (SCREEN_HEIGHT as i32 / 2)) - *y_pos 
        };

    (top_a, bottom_a)
}

fn get_box_b_xval(x_pos: &i32, width: &u32) -> (i32, i32){
    let left_b = *x_pos; 
    let right_b = *x_pos + *width as i32;

    (left_b, right_b)
}

fn get_box_b_yval(y_pos: &i32, height: &u32) -> (i32, i32){
    let top_b = *y_pos; 
    let bottom_b = *y_pos + *height as i32;

    (top_b, bottom_b)
}

fn check_collision(x_pos: &mut i32, y_pos: &mut i32, pole: &Rect, position: &Position) -> bool {
    //The sides of the rectangles

    let (left_a, right_a) = get_box_a_xval(x_pos);
    let (top_a, bottom_a) = get_box_a_yval(y_pos);

    //Calculate the sides of rect A
    let (left_b, right_b) = get_box_b_xval(&position.0.x(), &pole.width());
    let (top_b, bottom_b) = get_box_b_yval(&position.0.y(), &pole.height());

     //If any of the sides from A are outside of B
    if bottom_a <= top_b { return false; }
    if top_a >= bottom_b { return false; }
    if right_a <= left_b { return false; }
    if left_a >= right_b { return false; }

    //If none of the sides from A are outside B
    return true;
}