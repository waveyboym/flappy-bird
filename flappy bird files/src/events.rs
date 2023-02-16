use legion::*;
use sdl2::EventPump;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use crate::components::*;

pub fn process_events(event_pump: &mut EventPump, world: &mut World) -> bool {
    for event in event_pump.poll_iter() {
        match event {
            Event::Quit {..} |
            Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {return false;},
            Event::KeyDown { keycode: Some(Keycode::Space), repeat: false, .. } => {
                let mut query = <(&mut Velocity, &EntityType)>::query();

                //move bird up by five pixels
                for (velocity, entity_type) in query.iter_mut(world) {
                    if entity_type.0.eq(&Entitytype::Player){
                        velocity.direction = Direction::Up;
                    }
                    else if (entity_type.0.eq(&Entitytype::Pipegreen) || entity_type.0.eq(&Entitytype::Pipeyellow))
                    && velocity.direction.eq(&Direction::Still){
                        //this will only fire once when the player moves
                        velocity.direction = Direction::Movepolesleft;
                    }
                }
            },
            Event::KeyUp { keycode: Some(Keycode::Space), repeat: false, .. } => {
                let mut query = <(&mut Velocity, &EntityType)>::query();

                //stop movement
                for (velocity, entity_type) in query.iter_mut(world) {
                    if entity_type.0.eq(&Entitytype::Player){
                        velocity.direction = Direction::Null;
                        break;
                    }
                }
            },
            _ => {}
        }
    }

    return true;
}
