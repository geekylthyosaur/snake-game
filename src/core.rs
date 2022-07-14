use wasm_bindgen::{prelude::*, JsCast};
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};

use crate::{
    snake::{Cell, CellType, Food, Snake},
    utils::{Coords, Direction},
};

pub struct Core {
    pub snake: Snake,
    food: Food,
    context: CanvasRenderingContext2d,
    pub score: usize,
}

impl Core {
    pub fn new(canvas: &HtmlCanvasElement) -> Self {
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
            score: 0,
        }
    }

    pub fn move_snake(&mut self) {
        self.snake.r#move();
        if self.check_eating_condition() {
            self.generate_food();
            self.snake.grow();
            self.inc_score();
        }
    }

    pub fn render(&self, i: isize, delta: f32) {
        self.context.clear_rect(0f64, 0f64, 400 as f64, 400 as f64);
        draw_cells(&self.context);
        draw_snake(&self.context, &self.snake, i, delta);
        draw_food(&self.context, &self.food);
    }

    fn check_eating_condition(&self) -> bool {
        self.snake.get_head().coords.translate(10) == self.food.coords 
    }

    pub fn check_collision(&self) -> bool {
        if let Some((_, translated_cell_except_head_coords)) =
            self.snake.get_translated_cell_coords(10).split_first()
        {
            return translated_cell_except_head_coords
                .contains(&self.snake.get_head().coords.translate(10));
        } else {
            false
        }
    }

    pub fn inc_score(&mut self) {
        self.score += 1;
    }

    pub fn generate_food(&mut self) {
        let mut food = Food::new();
        while self
            .snake
            .get_translated_cell_coords(10)
            .contains(&food.coords)
        {
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

fn draw_snake(context: &CanvasRenderingContext2d, s: &Snake, i: isize, delta: f32) {
    for c in s.cells.iter() {
        draw_cell(context, c, i, delta);
    }
}

fn draw_cell(context: &CanvasRenderingContext2d, c: &Cell, i: isize, delta: f32) {
    let i = (i as f32 * delta) as isize;
    let Coords { x, y } = c.coords.translate(10);
    let (x, y) = match c.r#type {
        CellType::Head => (
            match c.direction {
                Direction::Right => (x - 1) * 40 + i,
                Direction::Left => (x + 1) * 40 - i,
                _ => x * 40,
            },
            match c.direction {
                Direction::Up => (y + 1) * 40 - i,
                Direction::Down => (y - 1) * 40 + i,
                _ => y * 40,
            },
        ),
        CellType::Tail => (
            match c.direction {
                Direction::Right => x * 40 + i,
                Direction::Left => x * 40 - i,
                _ => x * 40,
            },
            match c.direction {
                Direction::Up => y * 40 - i,
                Direction::Down => y * 40 + i,
                _ => y * 40,
            },
        ),
        _ => (x * 40, y * 40),
    };
    draw_rect(context, x, y, 40, 40, (30, 200, 30));
}

fn draw_food(context: &CanvasRenderingContext2d, f: &Food) {
    draw_rect(
        context,
        f.coords.x * 40,
        f.coords.y * 40,
        40,
        40,
        (200, 70, 70),
    );
    draw_rect(
        context,
        f.coords.x * 40 + 2,
        f.coords.y * 40 + 2,
        36,
        36,
        (200, 30, 30),
    );
}

fn draw_rect(
    context: &CanvasRenderingContext2d,
    x: isize,
    y: isize,
    width: isize,
    height: isize,
    style: (u8, u8, u8),
) {
    context.set_fill_style(&JsValue::from_str(
        format!("rgb({}, {}, {})", style.0, style.1, style.2).as_str(),
    ));
    context.fill_rect(x as f64, y as f64, width as f64, height as f64);
}
