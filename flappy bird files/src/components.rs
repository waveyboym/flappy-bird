use sdl2::rect::{Rect, Point};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Entitytype {
    Wallpaper,
    Player,
    Pipegreen,
    Pipeyellow,
    Landscape,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Still,
    Up,
    Movebgleft,
    Movepolesleft,
    Null,
}

pub struct EntityType(pub Entitytype);

/// The current position of a given entity
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Position(pub Point);

/// The current speed of a given entity
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Velocity {
    pub speed: i32,
    pub direction: Direction,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Sprite {
    /// The specific spritesheet to render from
    pub spritesheet: usize,
    /// The current region of the spritesheet to be rendered
    pub region: Rect,
}