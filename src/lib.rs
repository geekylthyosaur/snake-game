use wasm_bindgen::{prelude::*, JsCast};
use web_sys::{window, HtmlElement};

struct Core {
    snake: Snake,
}

struct Snake {
    cells: Vec<Cell>,
    direction: Direction,
}

#[derive(PartialEq, Clone)]
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
                Cell::new(CellType::Head, Coords::new(2, 0)),
                Cell::new(CellType::Middle, Coords::new(1, 0)),
                Cell::new(CellType::Tail, Coords::new(0, 0)),
            ],
            direction: Direction::Right,
        }
    }

    fn r#move(&mut self, d: Direction) -> () {
        let mut prev_cell_coords = Coords::new(0, 0);
        for mut c in self.cells.iter_mut() {
            match c.r#type {
                CellType::Head => { 
                    prev_cell_coords = c.coords.clone();
                    if self.direction.is_same_or_opposite(&d) { 
                        c.coords += self.direction.value(); 
                    } else {
                        c.coords += d.value();
                    }
                },
                _ => {
                    prev_cell_coords = c.coords.clone();
                    c.coords = prev_cell_coords;
                },
            }
        }
    }
}

impl Cell {
    fn new(r#type: CellType, coords: Coords) -> Self {
        Self { r#type, coords }
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

#[derive(PartialEq)]
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
            Self::Up => Coords::new(0, 1),
            Self::Down => Coords::new(0, -1),
            Self::Right => Coords::new(1, 0),
            Self::Left => Coords::new(-1, 0),
        }
    }
}

#[wasm_bindgen(start)]
pub fn run() {
    let document = window()
        .unwrap()
        .dyn_into::<HtmlElement>()
        .unwrap();
}

