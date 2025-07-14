use wasm_bindgen::prelude::*;
use hecs::{World};
use console_log;
use console_error_panic_hook;
use log::info;

#[derive(Debug)]
struct Tick {
    tick: u64,
}

#[wasm_bindgen]
pub struct WorldWrapper {
    world: World,
}

#[wasm_bindgen]
impl WorldWrapper {
    #[wasm_bindgen(constructor)]
    pub fn new() -> WorldWrapper {
        console_error_panic_hook::set_once();
        console_log::init().ok();

        let mut world = World::new();
        world.spawn((Tick { tick: 0 },));

        WorldWrapper { world }
    }

    pub fn update(&mut self) {
        for (_, tick) in self.world.query_mut::<&mut Tick>() {
            tick.tick += 1;
            info!("Tick: {}", tick.tick);
        }
    }
}

// #[derive(Debug)]
// struct Local;

// #[derive(Debug)]
// struct Player;

// #[derive(Debug)]
// struct Position {
//     x: i32,
//     y: i32
// }
//world.spawn((Player, Local, Position {x: 256, y: 192}));
