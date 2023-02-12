use legion::*;
use crate::components::*;
use crate::constants::*;

pub fn update(world: &mut World, period: &mut u8){
    
    //frame update
    if *period < PERIODSLIMIT {*period += 1;}
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
        else if velocity.direction.eq(&Direction::Still) {
            continue;
        }
        else if velocity.direction.eq(&Direction::Null){
            position.0 = position.0.offset(0, PLAYER_MOVEMENT_SPEED);
        }
        else if velocity.direction.eq(&Direction::Movetoleft){
            if position.0.x() <= LOWER_NEG_SCREEN_WIDTH{
                position.0.x = SCREEN_WIDTH as i32 * 2;
            }
            position.0 = position.0.offset(-BACKGROUND_WALLPAPER_MOVEMENT_SPEED, 0);
        }
    }

}
