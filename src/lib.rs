extern crate console_error_panic_hook;

use wasm_bindgen::{prelude::*, JsCast};
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, KeyboardEvent};

use std::cell::RefCell;
use std::panic;
use std::rc::Rc;

struct Core {
    snake: Snake,
    context: CanvasRenderingContext2d,
}

struct Snake {
    cells: Vec<Cell>,
    direction: Direction,
    next_direction: Direction,
}

#[derive(Debug, PartialEq, Clone, Copy)]
struct Coords {
    x: i32,
    y: i32,
}

struct Cell {
    r#type: CellType,
    coords: Coords,
}

impl Core {
    fn setup(canvas: &HtmlCanvasElement) -> Self {
        let context = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<CanvasRenderingContext2d>()
            .unwrap();
        Self {
            snake: Snake::new(),
            context,
        }
    }

    fn next(&mut self) {
        self.snake.move_to();
        self.context.clear_rect(0f64, 0f64, 600 as f64, 400 as f64);
        draw_cells(&self.context);
        draw_snake(&self.context, &self.snake);
    }
}

impl Snake {
    fn new() -> Self {
        Self {
            cells: vec![
                Cell::new(CellType::Head, Coords::new(4, 0)),
                Cell::new(CellType::Middle, Coords::new(3, 0)),
                Cell::new(CellType::Middle, Coords::new(2, 0)),
                Cell::new(CellType::Middle, Coords::new(1, 0)),
                Cell::new(CellType::Tail, Coords::new(0, 0)),
            ],
            direction: Direction::Right,
            next_direction: Direction::Right,
        }
    }

    fn move_to(&mut self) -> () {
        let mut prev_cell_coords = Coords::new(-1, -1);
        self.direction = self.next_direction;
        for c in self.cells.iter_mut() {
            match c.r#type {
                CellType::Head => {
                    prev_cell_coords = c.coords;
                    c.move_at(c.coords + self.direction.value());
                }
                CellType::Middle | CellType::Tail => {
                    let tmp = c.coords;
                    c.move_at(prev_cell_coords);
                    prev_cell_coords = tmp;
                }
            }
        }
    }

    fn change_direction(&mut self, d: Direction) {
        if !self.direction.is_same_or_opposite(&d) {
            self.next_direction = d;
        }
    }
}

impl Cell {
    fn new(r#type: CellType, coords: Coords) -> Self {
        Self { r#type, coords }
    }

    fn move_at(&mut self, c: Coords) -> () {
        self.coords = c;
    }
}

impl Coords {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

impl std::ops::Add<Coords> for Coords {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl std::ops::AddAssign<Coords> for Coords {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

#[derive(PartialEq, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Right,
    Left,
}
enum CellType {
    Head,
    Middle,
    Tail,
}

impl Direction {
    fn is_same_or_opposite(&self, other: &Self) -> bool {
        self == other || self.value() + other.value() == Coords::new(0, 0)
    }

    fn value(&self) -> Coords {
        match self {
            Self::Up => Coords::new(0, -1),
            Self::Down => Coords::new(0, 1),
            Self::Right => Coords::new(1, 0),
            Self::Left => Coords::new(-1, 0),
        }
    }
}

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
        let context = &core.borrow().context;
        let snake = &core.borrow().snake;
        draw_cells(&context);
        draw_snake(&context, &snake);
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

fn draw_cells(context: &CanvasRenderingContext2d) {
    for y in 0..10 {
        for x in 0..10 {
            context.set_stroke_style(&JsValue::from_str("rgb(50, 50, 50)"));
            context.stroke_rect((x * 40) as f64, (y * 40) as f64, 40f64, 40f64);
        }
    }
}

fn draw_snake(context: &CanvasRenderingContext2d, snake: &Snake) {
    for c in snake.cells.iter() {
        context.set_fill_style(&JsValue::from_str("rgb(30, 200, 30)"));
        context.fill_rect(
            (c.coords.x * 40) as f64,
            (c.coords.y * 40) as f64,
            40f64,
            40f64,
        );
    }
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
