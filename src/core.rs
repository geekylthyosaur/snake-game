use wasm_bindgen::{prelude::*, JsCast};
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};

use super::{snake::{Snake, Food}, utils::Coords};

pub struct Core {
    pub snake: Snake,
    food: Option<Food>,
    context: CanvasRenderingContext2d,
}

impl Core {
    pub fn setup(canvas: &HtmlCanvasElement) -> Self {
        let context = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<CanvasRenderingContext2d>()
            .unwrap();
        Self {
            snake: Snake::new(),
            food: None,
            context,
        }
    }

    pub fn next(&mut self) {
        self.snake.move_to();
        self.context.clear_rect(0f64, 0f64, 600 as f64, 400 as f64);
        draw_cells(&self.context);
        draw_snake(&self.context, &self.snake);
        if self.snake.head().coords == self.food.as_ref().unwrap().coords {
            self.gen_food();
            self.snake.grow();
        }
        if let Some(food) = &self.food {
            draw_food(&self.context, food);
        }
    }

    pub fn check_collision(&self) -> bool {
        let cells_coords: Vec<Coords> = self.snake.cells.iter().map(|c| c.coords).collect();
        if let Some((_, cells_except_head_coords)) = cells_coords.split_first() {
            return cells_except_head_coords.contains(&self.snake.head().coords);
        } else {
            false
        }
    }

    pub fn gen_food(&mut self) {
        let cells_coords: Vec<Coords> = self.snake.cells.iter().map(|c| c.coords).collect();
        let mut food = Food::new();
        while cells_coords.contains(&food.coords) {
            food = Food::new();
        }
        self.food = Some(food);
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

fn draw_food(context: &CanvasRenderingContext2d, f: &Food) {
    context.set_fill_style(&JsValue::from_str("rgb(200, 30, 30)"));
    context.fill_rect(
        (f.coords.x * 40) as f64,
        (f.coords.y * 40) as f64,
        40f64,
        40f64,
    );
}

