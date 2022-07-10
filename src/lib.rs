extern crate console_error_panic_hook;

use wasm_bindgen::{prelude::*, JsCast};
use web_sys::KeyboardEvent;

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

    let core = Rc::new(RefCell::new(Core::new()));
    let core_for_keyboard_handler = core.clone();

    let f = Rc::new(RefCell::new(None));
    let g = f.clone();

    let mut i = 0;
    *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
        if i % 60 == 0 {
            core.borrow_mut().move_snake();
            core.borrow().render();
        }
        if core.borrow().check_collision() {
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
