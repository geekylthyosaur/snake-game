extern crate console_error_panic_hook;

use wasm_bindgen::{prelude::*, JsCast};
use web_sys::{HtmlCanvasElement, HtmlElement, KeyboardEvent};

use std::cell::RefCell;
use std::panic;
use std::rc::Rc;

use crate::{
    core::Core,
    utils::{document, window, Direction},
};

mod core;
mod snake;
mod utils;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen(start)]
pub fn run() {
    panic::set_hook(Box::new(console_error_panic_hook::hook));

    let canvas = document()
        .get_element_by_id("canvas")
        .unwrap()
        .dyn_into::<HtmlCanvasElement>()
        .unwrap();
    canvas.set_width(400);
    canvas.set_height(400);

    let score_div = document()
        .get_element_by_id("score")
        .unwrap()
        .dyn_into::<HtmlElement>()
        .unwrap();

    let core = Rc::new(RefCell::new(Core::new(&canvas)));
    let core_for_keyboard_handler = core.clone();

    core.borrow_mut().generate_food();

    let f = Rc::new(RefCell::new(None));
    let g = f.clone();

    let mut i = 0;
    *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
        if i % 20 == 0 {
            core.borrow_mut().move_snake();
            score_div.set_inner_text(format!("Score: {}", core.borrow().score).as_str());
        }
        core.borrow().render(i % 20, (40 / 20) as f32);
        if core.borrow().check_collision() {
            core.borrow().display_lose_msg();
            return;
        }
        i += 1;
        request_animation_frame(f.borrow().as_ref().unwrap());
    }) as Box<dyn FnMut()>));

    request_animation_frame(g.borrow().as_ref().unwrap());

    let keyboard_handler = Closure::wrap(Box::new(move |e: KeyboardEvent| {
        let mut core = core_for_keyboard_handler.borrow_mut();
        let snake = &mut core.snake;
        match e.key().as_str() {
            "ArrowUp" | "w" | "k" => snake.change_direction(Direction::Up),
            "ArrowDown" | "s" | "j" => snake.change_direction(Direction::Down),
            "ArrowRight" | "d" | "l" => snake.change_direction(Direction::Right),
            "ArrowLeft" | "a" | "h" => snake.change_direction(Direction::Left),
            _ => (),
        }
    }) as Box<dyn FnMut(_)>);

    document()
        .add_event_listener_with_event_listener(
            "keydown",
            &keyboard_handler.as_ref().unchecked_ref(),
        )
        .unwrap();

    keyboard_handler.forget();
}

fn request_animation_frame(f: &Closure<dyn FnMut()>) -> i32 {
    window()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .unwrap()
}
