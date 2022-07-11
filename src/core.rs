use wasm_bindgen::{prelude::*, JsCast};
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};

use crate::{
    snake::{Cell, CellType, Food, Snake},
    utils::{Direction, document},
};

pub struct Core {
    pub snake: Snake,
    food: Food,
    context: CanvasRenderingContext2d,
}

impl Core {
    pub fn new() -> Self {
        let canvas = document()
            .get_element_by_id("canvas")
            .unwrap()
            .dyn_into::<HtmlCanvasElement>()
            .unwrap();
        canvas.set_width(400);
        canvas.set_height(400);
        let context = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<CanvasRenderingContext2d>()
            .unwrap();
        Self {
            snake: Snake::new(),
            food: Food::new(),
            context,
        }
    }

    pub fn move_snake(&mut self) {
        self.snake.r#move();
        if self.snake.get_head().coords == self.food.coords {
            self.generate_food();
            self.snake.grow();
        }
    }

    pub fn render(&self, i: i32) {
        self.context.clear_rect(0f64, 0f64, 600 as f64, 400 as f64);
        draw_cells(&self.context);
        draw_snake(&self.context, &self.snake, i);
        draw_food(&self.context, &self.food);
    }

    pub fn check_collision(&self) -> bool {
        if let Some((_, cell_except_head_coords)) = self.snake.get_cell_coords().split_first() {
            return cell_except_head_coords.contains(&self.snake.get_head().coords);
        } else {
            false
        }
    }

    pub fn generate_food(&mut self) {
        let mut food = Food::new();
        while self.snake.get_cell_coords().contains(&food.coords) {
            food = Food::new();
        }
        self.food = food;
    }
}

fn draw_cells(context: &CanvasRenderingContext2d) {
    for y in 0..10 {
        for x in 0..10 {
            context.set_stroke_style(&JsValue::from_str("rgb(50, 50, 50)"));
            context.stroke_rect((x * 40) as f64, (y * 40) as f64, 40f64, 40f64);
        }
    }
}

fn draw_snake(context: &CanvasRenderingContext2d, snake: &Snake, i: i32) {
    for c in snake.cells.iter() {
        draw_cell(context, c, i);
    }
}

fn draw_cell(context: &CanvasRenderingContext2d, c: &Cell, i: i32) {
    context.set_fill_style(&JsValue::from_str("rgb(30, 200, 30)"));
    let x;
    let y;
    if c.r#type == CellType::Head {
        x = match c.direction {
            Direction::Up => (c.coords.x * 40) as f64,
            Direction::Down => (c.coords.x * 40) as f64,
            Direction::Right => ((c.coords.x - 1) * 40 + i) as f64,
            Direction::Left => ((c.coords.x + 1) * 40 - i) as f64,
        };
        y = match c.direction {
            Direction::Up => ((c.coords.y + 1) * 40 - i) as f64,
            Direction::Down => ((c.coords.y - 1) * 40 + i) as f64,
            Direction::Right => (c.coords.y * 40) as f64,
            Direction::Left => (c.coords.y * 40) as f64,
        };
    } else if c.r#type == CellType::Tail {
        x = match c.direction {
            Direction::Up => (c.coords.x * 40) as f64,
            Direction::Down => (c.coords.x * 40) as f64,
            Direction::Right => (c.coords.x * 40 + i) as f64,
            Direction::Left => (c.coords.x * 40 - i) as f64,
        };
        y = match c.direction {
            Direction::Up => (c.coords.y * 40 - i) as f64,
            Direction::Down => (c.coords.y * 40 + i) as f64,
            Direction::Right => (c.coords.y * 40) as f64,
            Direction::Left => (c.coords.y * 40) as f64,
        };
    } else {
        x = (c.coords.x * 40) as f64;
        y = (c.coords.y * 40) as f64;
    }
    context.fill_rect(
        x, 
        y,
        40f64,
        40f64,
    );
    /*
    context.fill_rect(
        (c.coords.x * 40) as f64, 
        (c.coords.y * 40) as f64,
        40f64,
        40f64,
    );
    context.set_fill_style(&JsValue::from_str("rgb(100, 200, 100)"));
    context.fill_rect(
        (c.coords.x * 40 + 2) as f64,
        (c.coords.y * 40 + 2) as f64,
        36f64,
        36f64,
    );
    match c.direction {
        Direction::Up | Direction::Down => {
            context.set_fill_style(&JsValue::from_str("rgb(0, 0, 0)"));
            context.fill_rect(
                (c.coords.x * 40 + 2) as f64,
                (c.coords.y * 40 + 2) as f64,
                36f64,
                36f64,
            );
        },
        Direction::Right | Direction::Left => {
            context.set_fill_style(&JsValue::from_str("rgb(200, 200, 200)"));
            context.fill_rect(
                (c.coords.x * 40 + 2) as f64,
                (c.coords.y * 40 + 2) as f64,
                36f64,
                36f64,
            );
        },
    }*/
}

fn draw_food(context: &CanvasRenderingContext2d, f: &Food) {
    context.set_fill_style(&JsValue::from_str("rgb(200, 70, 70)"));
    context.fill_rect(
        (f.coords.x * 40) as f64,
        (f.coords.y * 40) as f64,
        40f64,
        40f64,
    );
    context.set_fill_style(&JsValue::from_str("rgb(200, 30, 30)"));
    context.fill_rect(
        (f.coords.x * 40 + 2) as f64,
        (f.coords.y * 40 + 2) as f64,
        36f64,
        36f64,
    );
}
