// world.rs
use wasm_bindgen::prelude::*;
use web_sys::{window, HtmlCanvasElement, CanvasRenderingContext2d};
use wasm_bindgen::JsCast;

use hecs::World;
use log::info;

use crate::components::*;
use crate::systems::*;
use crate::render::*;

#[wasm_bindgen]
pub struct WorldWrapper {
    world: World,
    context: CanvasRenderingContext2d,
}

#[wasm_bindgen]
impl WorldWrapper {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Result<WorldWrapper, JsValue> {
        let window = window().unwrap();
        let document = window.document().unwrap();
        let canvas = document
            .get_element_by_id("my_canvas")
            .unwrap()
            .dyn_into::<HtmlCanvasElement>()?;

        let context = canvas
            .get_context("2d")?
            .unwrap()
            .dyn_into::<CanvasRenderingContext2d>()?;

        let mut world = World::new();
        world.spawn((Tick { tick: 0 },));
        world.spawn((
            Local, Player,
            State {state: PlayerState::Idle},
            Position { x: 512.0, y: 384.0 },
            Velocity { x: 0.0, y: 0.0 },
            PlayerCollision { radius: 16.0, offset_x: 0.0, offset_y: 0.0 },
            MoveTarget { x: 512.0, y: 384.0 },
            PlayerMove {move_speed: 2.0, move_input_type: MovementType::Target, timer: 0, timer_threshold: 10, direction_radius: 24.0},
        ));

        world.spawn((
            Collision {
                collision_lines: vec![
                    CollisionLine { x1: 384.0, y1: 256.0, x2: 640.0, y2: 256.0 },
                    CollisionLine { x1: 640.0, y1: 256.0, x2: 640.0, y2: 512.0 },
                    CollisionLine { x1: 640.0, y1: 512.0, x2: 592.0, y2: 416.0 },
                    CollisionLine { x1: 592.0, y1: 416.0, x2: 496.0, y2: 512.0 },
                ]
            },
        ));

        Ok(WorldWrapper { world, context })
    }

    pub fn input_click_pressed(&mut self, x: f32, y: f32) {
        info!("Mouse clicked at: ({}, {})", x, y);
        for (_, (_,
            _,
            state,
            target,
            move_type
        )) in self.world.query::<(
            &Local,
            &Player,
            &State,
            &mut MoveTarget,
            &mut PlayerMove
        )>().iter() {
            match state.state {
                PlayerState::Idle | PlayerState::Move => {
                    move_type.move_input_type = MovementType::Target;
                    move_type.timer = 0;
                    target.x = x;
                    target.y = y;
                }
            }
        }
    }

    pub fn input_click_hold(&mut self, x: f32, y: f32) {
        info!("Mouse held at: ({}, {})", x, y);
        for (_, (
            _,
            _,
            state,
            target,
            move_type
        )) in self.world.query::<(
            &Local,
            &Player,
            &State,
            &mut MoveTarget,
            &mut PlayerMove
        )>().iter() {
            match state.state {
                PlayerState::Idle | PlayerState::Move => {
                    if move_type.timer < move_type.timer_threshold {
                        move_type.timer += 1;
                    }
                    else {
                        move_type.move_input_type = MovementType::Direction;
                        target.x = x;
                        target.y = y;
                    }
                }
            }
        }
    }

    pub fn input_click_released(&mut self) {
        for (_, (
            _,
            _,
            state,
            position,
            target,
            move_type
        )) in self.world.query::<(
            &Local,
            &Player,
            &State,
            &Position,
            &mut MoveTarget,
            &mut PlayerMove
        )>().iter() {
            if state.state == PlayerState::Move && move_type.move_input_type == MovementType::Direction {
                move_type.timer = 0;
                target.x = position.x;
                target.y = position.y;
            }
        }
    }

    pub fn update(&mut self) -> Result<(), JsValue> {
        update_tick(&mut self.world);
        update_state(&mut self.world);
        player_state(&mut self.world);
        apply_velocity(&mut self.world);
        render(&self.world, &self.context)
    }
}
