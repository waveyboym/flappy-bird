use legion::*;
use crate::components::*;
use crate::constants::*;

pub fn update(world: &mut World, period: &mut u8){
    
    //frame update for player ONLY
    if *period < PERIODSLIMIT {*period += 1;}
    else{
        let mut query = <&mut Sprite>::query();
        for sprite in query.iter_mut(world) {
            if sprite.region.x() == (3 * PLAYER_SPRITE_SIZE_WL as i32) {sprite.region.x = 0;}
            else {sprite.region.x += PLAYER_SPRITE_SIZE_WL as i32;}
        }
        *period = 0;
    }

    //postion update
    let mut query = <(&mut Position, &Velocity)>::query();
    for (position, velocity) in query.iter_mut(world) {
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
            continue;
        }
    }

}
