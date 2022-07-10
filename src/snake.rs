use rand::Rng;

use crate::utils::{Coords, Direction};

pub struct Snake {
    pub cells: Vec<Cell>,
    pub direction: Direction,
    pub next_direction: Direction,
}

pub struct Food {
    pub coords: Coords,
}

pub struct Cell {
    pub r#type: CellType,
    pub coords: Coords,
}

impl Snake {
    pub fn new() -> Self {
        Self {
            cells: vec![
                Cell::new(CellType::Head, Coords::new(4, 0)),
                Cell::new(CellType::Other, Coords::new(3, 0)),
                Cell::new(CellType::Other, Coords::new(2, 0)),
                Cell::new(CellType::Other, Coords::new(1, 0)),
                Cell::new(CellType::Other, Coords::new(0, 0)),
            ],
            direction: Direction::Right,
            next_direction: Direction::Right,
        }
    }

    pub fn head(&self) -> &Cell {
        &self.cells[0]
    }

    pub fn grow(&mut self) {
        self.cells
            .push(Cell::new(CellType::Other, Coords::new(-1, -1)));
    }

    pub fn move_to(&mut self) -> () {
        let mut prev_cell_coords = Coords::new(-1, -1);
        self.direction = self.next_direction;
        for c in self.cells.iter_mut() {
            match c.r#type {
                CellType::Head => {
                    prev_cell_coords = c.coords;
                    c.move_at(c.coords + self.direction.value());
                }
                CellType::Other => {
                    let tmp = c.coords;
                    c.move_at(prev_cell_coords);
                    prev_cell_coords = tmp;
                }
            }
        }
    }

    pub fn change_direction(&mut self, d: Direction) {
        if !self.direction.is_same_or_opposite(&d) {
            self.next_direction = d;
        }
    }
}

impl Food {
    pub fn new() -> Self {
        let mut rng = rand::thread_rng();
        Self {
            coords: rng.gen::<Coords>(),
        }
    }
}

impl Cell {
    pub fn new(r#type: CellType, coords: Coords) -> Self {
        Self { r#type, coords }
    }

    pub fn move_at(&mut self, mut c: Coords) -> () {
        if c.x < 0 {
            c.x = 9;
        }
        if c.y < 0 {
            c.y = 9;
        }
        if c.x > 9 {
            c.x = 0;
        }
        if c.y > 9 {
            c.y = 0;
        }
        self.coords = c;
    }
}

pub enum CellType {
    Head,
    Other,
}
