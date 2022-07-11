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
    pub direction: Direction,
}

impl Snake {
    pub fn new() -> Self {
        Self {
            cells: vec![
                Cell::new(CellType::Head, Coords::new(4, 0), Direction::Right),
                Cell::new(CellType::Middle, Coords::new(3, 0), Direction::Right),
                Cell::new(CellType::Middle, Coords::new(2, 0), Direction::Right),
                Cell::new(CellType::Middle, Coords::new(1, 0), Direction::Right),
                Cell::new(CellType::Tail, Coords::new(0, 0), Direction::Right),
            ],
            direction: Direction::Right,
            next_direction: Direction::Right,
        }
    }

    pub fn get_head(&self) -> &Cell {
        &self.cells[0]
    }

    pub fn get_cell_coords(&self) -> Vec<Coords> {
        self.cells.iter().map(|c| c.coords).collect::<Vec<Coords>>()
    }

    pub fn grow(&mut self) {
        self.cells.last_mut().unwrap().r#type = CellType::Middle;
        let d = self.cells.last().unwrap().direction;
        self.cells
            .push(Cell::new(CellType::Tail, self.cells.last().unwrap().coords - self.cells.last().unwrap().direction.value(), d));
    }

    pub fn r#move(&mut self) -> () {
        let mut prev_cell_coords = Coords::new(-1, -1);
        self.direction = self.next_direction;
        let mut iter = self.cells.iter_mut().peekable(); 
        while let Some(c) = iter.next() {
            match c.r#type {
                CellType::Head => {
                    if let Some(c) = iter.peek_mut() {
                        c.direction = self.direction;
                    }
                    c.direction = self.direction;
                    prev_cell_coords = c.coords;
                    c.move_to(c.coords + self.direction.value());
                }
                CellType::Middle | CellType::Tail => {
                    let d = (c.coords - prev_cell_coords).direction().unwrap_or(c.direction);
                    if let Some(c) = iter.peek_mut() {
                        c.direction = d;
                    }
                    let tmp = c.coords;
                    c.move_to(prev_cell_coords);
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
    pub fn new(r#type: CellType, coords: Coords, direction: Direction) -> Self {
        Self { r#type, coords, direction }
    }

    pub fn move_to(&mut self, mut c: Coords) -> () {
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

#[derive(PartialEq)]
pub enum CellType {
    Head,
    Middle,
    Tail,
}
