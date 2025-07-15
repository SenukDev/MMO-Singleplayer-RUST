// systems.rs
use crate::components::*;
use hecs::World;

pub fn update_tick(world: &mut World) {
    for (_, tick) in world.query_mut::<&mut Tick>() {
        tick.tick += 1;
    }
}

pub fn update_velocity(world: &mut World) {
    for (_, (position, velocity, target, speed)) in world.query::<(&Position, &mut Velocity, &MoveTarget, &MoveSpeed)>().iter() {
        let dx = target.x - position.x;
        let dy = target.y - position.y;
        let length = (dx * dx + dy * dy).sqrt();
        let speed = speed.speed;

        if length > speed {
            velocity.x = dx / length * speed;
            velocity.y = dy / length * speed;
        } else if length > 0.1 {
            velocity.x = dx;
            velocity.y = dy;
        } else {
            velocity.x = 0.0;
            velocity.y = 0.0;
        }
    }
}

pub fn apply_velocity(world: &mut World) {
    for (_, (_, position, velocity, _)) in world.query::<(&Player, &mut Position, &mut Velocity, &Collision)>().iter() {
        position.x += velocity.x;
        position.y += velocity.y;
    }
}
