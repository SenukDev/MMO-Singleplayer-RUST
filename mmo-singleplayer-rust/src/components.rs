#[derive(Debug)]
pub struct Tick {
    pub tick: u64,
}

#[derive(Debug)]
pub struct Local;

#[derive(Debug)]
pub struct Player;

#[derive(Debug)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

#[derive(Debug)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
}

#[derive(Debug)]
pub struct Collision {
    pub radius: f32,
    pub offset_x: f32,
    pub offset_y: f32,
}

#[derive(Debug)]
pub struct MoveSpeed {
    pub speed: f32,
}

#[derive(Debug)]
pub struct MoveTarget {
    pub x: f32,
    pub y: f32,
}
