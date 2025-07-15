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
            Position { x: 256.0, y: 192.0 },
            Velocity { x: 1.0, y: 0.0 },
            MoveSpeed { speed: 2.0 },
            Collision { radius: 16.0, offset_x: 0.0, offset_y: -16.0 },
            MoveTarget { x: 256.0, y: 192.0 },
        ));

        Ok(WorldWrapper { world, context })
    }

    pub fn input(&mut self, x: f32, y: f32) {
        info!("Mouse clicked at: ({}, {})", x, y);
        for (_, (_, _, target)) in self.world.query::<(&Local, &Player, &mut MoveTarget)>().iter() {
            target.x = x;
            target.y = y;
        }
    }

    pub fn update(&mut self) -> Result<(), JsValue> {
        update_tick(&mut self.world);
        update_velocity(&mut self.world);
        apply_velocity(&mut self.world);
        render(&self.world, &self.context)
    }
}
