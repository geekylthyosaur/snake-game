extern crate console_error_panic_hook;

use wasm_bindgen::{prelude::*, JsCast};
use web_sys::{HtmlCanvasElement, KeyboardEvent};

use std::cell::RefCell;
use std::panic;
use std::rc::Rc;

use crate::{
    core::Core,
    utils::Direction,
};

mod snake;
mod core;
mod utils;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen(start)]
pub fn run() {
    panic::set_hook(Box::new(console_error_panic_hook::hook));
    let document = document();

    let canvas = Rc::new(RefCell::new(
        document
            .get_element_by_id("canvas")
            .unwrap()
            .dyn_into::<HtmlCanvasElement>()
            .unwrap(),
    ));

    canvas.borrow().set_width(600);
    canvas.borrow().set_height(400);

    let core = Rc::new(RefCell::new(Core::setup(&canvas.borrow())));
    let core_for_keyboard_handler = core.clone();
    {
        // First frame
        core.borrow_mut().gen_food();
        core.borrow_mut().next();
    }

    let f = Rc::new(RefCell::new(None));
    let g = f.clone();

    {
        let mut i = 0;
        *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
            i += 1;
            if i % 60 == 0 {
                core.borrow_mut().next();
            }
            if core.borrow().check_collision() {
                return;
            }
            request_animation_frame(f.borrow().as_ref().unwrap());
        }) as Box<dyn FnMut()>));
    }

    request_animation_frame(g.borrow().as_ref().unwrap());

    let keyboard_handler = Closure::wrap(Box::new(move |e: KeyboardEvent| {
        let mut core = core_for_keyboard_handler.borrow_mut();
        let snake = &mut core.snake;
        match e.key().as_str() {
            "ArrowUp" => snake.change_direction(Direction::Up),
            "ArrowDown" => snake.change_direction(Direction::Down),
            "ArrowRight" => snake.change_direction(Direction::Right),
            "ArrowLeft" => snake.change_direction(Direction::Left),
            _ => (),
        }
    }) as Box<dyn FnMut(_)>);

    document
        .add_event_listener_with_event_listener(
            "keydown",
            &keyboard_handler.as_ref().unchecked_ref(),
        )
        .unwrap();

    keyboard_handler.forget();
}

fn window() -> web_sys::Window {
    web_sys::window().expect("No global 'window' exist!")
}

fn document() -> web_sys::Document {
    window()
        .document()
        .expect("Should have a document on a window!")
}

fn request_animation_frame(f: &Closure<dyn FnMut()>) -> i32 {
    window()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .unwrap()
}
