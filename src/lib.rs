use wasm_bindgen::{prelude::*, JsCast};
use web_sys::{window, HtmlCanvasElement, KeyboardEvent, CanvasRenderingContext2d};

use std::rc::Rc;
use std::cell::RefCell;

struct Core {
    snake: Snake,
}

struct Snake {
    cells: Vec<Cell>,
    direction: Direction,
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
    fn setup() -> Self {
        Self { snake: Snake::new() }
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
        }
    }

    fn move_to(&mut self, d: Direction) -> () {
        let mut prev_cell_coords = Coords::new(-1, -1);
        if !self.direction.is_same_or_opposite(&d) { self.direction = d.clone(); }
        for c in self.cells.iter_mut() {
            match c.r#type {
                CellType::Head => { 
                    prev_cell_coords = c.coords;
                    if self.direction.is_same_or_opposite(&d) { 
                        c.move_at(c.coords + self.direction.value()); 
                    } else {
                        c.move_at(c.coords + d.value());
                    }
                },
                CellType::Middle
                | CellType::Tail => {
                    let tmp = c.coords;
                    c.move_at(prev_cell_coords);
                    prev_cell_coords = tmp;
                },
            }
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

#[derive(PartialEq, Clone)]
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
        self == other
        || self.value() + other.value() == Coords::new(0, 0)
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

#[wasm_bindgen(start)]
pub fn run() {
    let core = Rc::new(RefCell::new(Core::setup()));

    let document = window()
        .unwrap()
        .document()
        .unwrap();

    let canvas = Rc::new(RefCell::new(document                  
        .get_element_by_id("root")
        .unwrap()
        .dyn_into::<HtmlCanvasElement>()
        .unwrap()));

    canvas.borrow().set_width(600);
    canvas.borrow().set_height(400);
    let ctx = Rc::new(RefCell::new(canvas.borrow()
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<CanvasRenderingContext2d>()
        .unwrap()));

    draw_cells(&ctx.borrow());
    draw_snake(&ctx.borrow(), &core.borrow());

    let keyboard_handler = Closure::wrap(Box::new(move |e: KeyboardEvent| {
        match e.key().as_str() {
            "ArrowUp" => core.borrow_mut().snake.move_to(Direction::Up),
            "ArrowDown" => core.borrow_mut().snake.move_to(Direction::Down),
            "ArrowRight" => core.borrow_mut().snake.move_to(Direction::Right),
            "ArrowLeft" => core.borrow_mut().snake.move_to(Direction::Left),
            _ => (),
        }
        ctx.borrow().clear_rect(0f64, 0f64, 600 as f64, 400 as f64);
        draw_cells(&ctx.borrow());
        draw_snake(&ctx.borrow(), &core.borrow());
    }) as Box<dyn FnMut(_)>);

    document
        .add_event_listener_with_event_listener(
            "keydown",
            &keyboard_handler.as_ref().unchecked_ref(),
        )
        .unwrap();

    keyboard_handler.forget();
}

fn draw_cells(ctx: &CanvasRenderingContext2d) -> () {
    for y in 0..10 {
        for x in 0..10 {
            ctx.set_stroke_style(&JsValue::from_str("rgb(50, 50, 50)"));
            ctx.stroke_rect((x*40) as f64, (y*40) as f64, 40f64, 40f64);
        }
    }
}

fn draw_snake(ctx: &CanvasRenderingContext2d, core: &Core) -> () {
    for c in core.snake.cells.iter() {
        ctx.set_fill_style(&JsValue::from_str("rgb(30, 200, 30)"));
        ctx.fill_rect((c.coords.x*40) as f64, (c.coords.y*40) as f64, 40f64, 40f64);
    }
}

