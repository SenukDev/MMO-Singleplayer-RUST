use wasm_bindgen::prelude::*;
use web_sys::{window, HtmlCanvasElement, CanvasRenderingContext2d};
use wasm_bindgen::JsCast;
use hecs::{World};
use console_log;
use console_error_panic_hook;
use log::info;

#[derive(Debug)]
struct Tick {
    tick: u64,
}

#[derive(Debug)]
struct Player;

#[derive(Debug)]
struct Position {
    x: f32,
    y: f32
}

#[derive(Debug)]
struct Collision {
    radius: f32,
    offset_x: f32,
    offset_y: f32
}

#[wasm_bindgen]
pub struct WorldWrapper {
    world: World,
    context: CanvasRenderingContext2d,
}

#[wasm_bindgen]
impl WorldWrapper {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Result<WorldWrapper, JsValue> {
        console_error_panic_hook::set_once();
        console_log::init().ok();

        //Set up canvas and context
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

        //Set up ECS world
        let mut world = World::new();
        world.spawn((Tick { tick: 0 },));
        world.spawn((Player, Position {x: 256.0, y: 192.0}, Collision {radius: 16.0, offset_x: 0.0, offset_y: -16.0}));

        Ok(WorldWrapper { world, context })
    }

    pub fn update(&mut self) -> Result<(), JsValue> {
        for (_, tick) in self.world.query_mut::<&mut Tick>() {
            tick.tick += 1;
            info!("Tick: {}", tick.tick);
        }

        self.context.set_stroke_style(&wasm_bindgen::JsValue::from_str("#FFFFFF"));
        self.context.set_fill_style(&wasm_bindgen::JsValue::from_str("#FFFFFF"));
        for (_, (_, pos, collision)) in self.world.query::<(&Player, &Position, &Collision)>().iter() {

            //Draw Player Collision Outline
            self.context.begin_path();
            self.context.ellipse(
                (pos.x + collision.offset_x) as f64,
                (pos.y + collision.offset_y) as f64,
                collision.radius as f64,
                collision.radius as f64,
                0.0,
                0.0,
                std::f64::consts::PI * 2.0,
            )?;
            self.context.stroke();

            //Draw Player Position
            self.context.begin_path();
            self.context.ellipse(
                pos.x as f64,
                pos.y as f64,
                4.0,
                4.0,
                0.0,
                0.0,
                std::f64::consts::PI * 2.0,
            )?;
            self.context.fill();
        }

        Ok(())
    }
}
