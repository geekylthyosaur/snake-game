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
        self.context.clear_rect(0f64, 0f64, 400f64, 400f64);
        self.draw_cells();
        self.draw_snake(i, delta);
        self.draw_food();
    }

    pub fn display_lose_msg(&self) {
        self.context.set_font("64px Geneva");
        self.context.set_text_align("center");
        self.context.fill_text("WASTED", 200f64, 200f64).unwrap();
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

    fn draw_cells(&self) {
        for y in 0..10 {
            for x in 0..10 {
                self.context
                    .set_stroke_style(&JsValue::from_str("rgb(50, 50, 50)"));
                self.context
                    .stroke_rect((x * 40) as f64, (y * 40) as f64, 40f64, 40f64);
            }
        }
    }

    fn draw_snake(&self, i: isize, delta: f32) {
        for c in self.snake.cells.iter() {
            self.draw_cell(c, i, delta);
        }
        self.draw_line(i, delta);
    }

    fn draw_cell(&self, c: &Cell, i: isize, delta: f32) {
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
        self.draw_rect(x, y, 40, 40, (30, 200, 30));
    }

    fn draw_line(&self, i: isize, delta: f32) {
        self.context
            .set_stroke_style(&JsValue::from_str("rgb(0, 0, 0)"));
        let mut iter = self.snake.cells.iter().peekable();
        self.context.begin_path();
        while let Some(c) = iter.next() {
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
            if let Some(n) = iter.peek() {
                if isize::abs(n.coords.translate(10).x - c.coords.translate(10).x) > 1
                    || isize::abs(n.coords.translate(10).y - c.coords.translate(10).y) > 1
                {
                    self.context.move_to(
                        (n.coords.translate(10).x * 40 + 18) as f64,
                        (n.coords.translate(10).y * 40 + 18) as f64,
                    );
                } else {
                    self.context.line_to((x + 18) as f64, (y + 18) as f64);
                }
            } else {
                self.context.line_to((x + 18) as f64, (y + 18) as f64);
            }
            self.context.stroke();
        }
        self.context.close_path();
    }

    fn draw_food(&self) {
        self.draw_rect(
            self.food.coords.x * 40,
            self.food.coords.y * 40,
            40,
            40,
            (200, 70, 70),
        );
        self.draw_rect(
            self.food.coords.x * 40 + 2,
            self.food.coords.y * 40 + 2,
            36,
            36,
            (200, 30, 30),
        );
    }

    fn draw_rect(&self, x: isize, y: isize, width: isize, height: isize, style: (u8, u8, u8)) {
        self.context.set_fill_style(&JsValue::from_str(
            format!("rgb({}, {}, {})", style.0, style.1, style.2).as_str(),
        ));
        self.context
            .fill_rect(x as f64, y as f64, width as f64, height as f64);
    }
}
