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
struct Local;

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

#[derive(Debug)]
struct MouseInput {
    x: f32,
    y: f32,
}

#[derive(Debug)]
struct MoveTarget {
    x: f32,
    y: f32
}

#[wasm_bindgen]
pub struct WorldWrapper {
    world: World,
    context: CanvasRenderingContext2d,
    mouse_input: Option<MouseInput>,
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
        world.spawn((Local, Player, Position {x: 256.0, y: 192.0}, Collision {radius: 16.0, offset_x: 0.0, offset_y: -16.0}, MoveTarget {x: 256.0, y: 192.0}));

        Ok(WorldWrapper {
            world,
            context,
            mouse_input: None,
        })
    }

    #[wasm_bindgen]
    pub fn input(&mut self, x: f32, y: f32) {
        self.mouse_input = Some(MouseInput { x, y });
        info!("Mouse clicked at: ({}, {})", x, y);

        for (_, (_, _, target)) in self.world.query::<(&Local, &Player, &mut MoveTarget)>().iter() {
            target.x = x;
            target.y = y;
        }
    }

    pub fn update(&mut self) -> Result<(), JsValue> {
        for (_, tick) in self.world.query_mut::<&mut Tick>() {
            tick.tick += 1;
            //info!("Tick: {}", tick.tick);
        }



        //Rendering

        //Draw Background
        self.context.set_fill_style(&wasm_bindgen::JsValue::from_str("#000000"));
        self.context.fill_rect(0.0, 0.0, 512.0, 384.0);

        //Draw Player's Move Target
        self.context.set_fill_style(&wasm_bindgen::JsValue::from_str("#FF6666"));
        for (_, (_, _, target)) in self.world.query::<(&Local, &Player, &MoveTarget)>().iter() {
            
            self.context.begin_path();
            self.context.ellipse(
                target.x as f64,
                target.y as f64,
                4.0,
                4.0,
                0.0,
                0.0,
                std::f64::consts::PI * 2.0,
            )?;
            self.context.fill();
        }


        self.context.set_stroke_style(&wasm_bindgen::JsValue::from_str("#FFFFFF"));
        self.context.set_fill_style(&wasm_bindgen::JsValue::from_str("#FFFFFF"));
        for (_, (_, pos, collision)) in self.world.query::<(&Player, &Position, &Collision)>().iter() {
            //Draw Player's Collision Outline
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

            //Draw Player's Position
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
