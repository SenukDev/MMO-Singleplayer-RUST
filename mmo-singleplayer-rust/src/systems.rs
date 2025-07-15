// systems.rs
use crate::components::*;
use crate::scripts::*;
use hecs::World;

pub fn update_tick(world: &mut World) {
    for (_, tick) in world.query_mut::<&mut Tick>() {
        tick.tick += 1;
    }
}

pub fn update_state(world: &mut World) {
    for (_,(
        _,
        _,
        state,
        position,
        target
    )) in world.query::<(
        &Local,
        &Player,
        &mut State,
        &Position,
        &MoveTarget,
    )>().iter() {
        let dx = target.x - position.x;
        let dy = target.y - position.y;
        let distance = (dx * dx + dy * dy).sqrt();

        if distance > 0.0 {
            state.state = PlayerState::Move;
        }
        else {
            state.state = PlayerState::Idle;
        }
    }
}

pub fn player_state(world: &mut World) {
    for (_,(
        _,
        _,
        state,
        position,
        velocity,
        target,
        player_collision,
        player_move,
    )) in world.query::<(
        &Local,
        &Player,
        &mut State,
        &Position,
        &mut Velocity,
        &mut MoveTarget,
        &PlayerCollision,
        &PlayerMove
    )>().iter() {
        match state.state {
            PlayerState::Idle => {
                target.x = position.x;
                target.y = position.y;
                velocity.x = 0.0;
                velocity.y = 0.0;
            },
            PlayerState::Move => {
                //Velocity towards Move Target
                let dx = target.x - position.x;
                let dy = target.y - position.y;
                let length = (dx * dx + dy * dy).sqrt();

                if length > player_move.move_speed {
                    velocity.x = dx / length * player_move.move_speed;
                    velocity.y = dy / length * player_move.move_speed;
                } else if length > 0.1 {
                    velocity.x = dx;
                    velocity.y = dy;
                } else {
                    velocity.x = 0.0;
                    velocity.y = 0.0;
                }
                
                for (_, collision) in world.query::<&Collision>().iter() {
                    let (vx, vy) = collision_slide_velocity(&position, &velocity, &player_collision, &collision, 3);
                    velocity.x = vx;
                    velocity.y = vy;
                }
            }
        }
    }
}

pub fn apply_velocity(world: &mut World) {
    for (_,(
        _,
        position,
        velocity,
        _
    )) in world.query::<(
        &Player,
        &mut Position,
        &Velocity,
        &PlayerCollision
    )>().iter() {
        position.x += velocity.x;
        position.y += velocity.y;
    }
}