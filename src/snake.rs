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
                Cell::new(CellType::Head, Coords::new(0, 0), Direction::Right),
                Cell::new(CellType::Tail, Coords::new(0, 0), Direction::Right),
            ],
            direction: Direction::Right,
            next_direction: Direction::Right,
        }
    }

    pub fn get_head(&self) -> &Cell {
        self.cells.first().unwrap()
    }

    pub fn get_translated_cell_coords(&self, map_size: i32) -> Vec<Coords> {
        self.cells
            .iter()
            .map(|c| c.coords.translate(map_size))
            .collect::<Vec<Coords>>()
    }

    pub fn grow(&mut self) {
        self.cells.last_mut().unwrap().r#type = CellType::Middle;
        let last_cell = self.cells.last().unwrap();
        self.cells.push(Cell::new(
            CellType::Tail,
            last_cell.coords,
            last_cell.direction,
        ));
    }

    pub fn r#move(&mut self) -> () {
        let mut prev_cell_coords = Coords::new(-1, -1);
        self.direction = self.next_direction;
        let mut iter = self.cells.iter_mut().peekable();
        while let Some(c) = iter.next() {
            match c.r#type {
                CellType::Head => {
                    if let Some(next_cell) = iter.peek_mut() {
                        next_cell.direction = self.direction;
                    }
                    c.direction = self.direction;
                    prev_cell_coords = c.coords;
                    c.move_to(c.coords + Coords::from(self.direction));
                }
                CellType::Middle | CellType::Tail => {
                    if let Some(next_cell) = iter.peek_mut() {
                        let d: Option<Direction> = (c.coords - prev_cell_coords).into();
                        next_cell.direction = d.unwrap_or(c.direction);
                    }
                    let tmp = c.coords;
                    c.move_to(prev_cell_coords);
                    prev_cell_coords = tmp;
                }
            }
        }
    }

    pub fn change_direction(&mut self, d: Direction) {
        if !self.direction.is_same_or_opposite(d) {
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
        Self {
            r#type,
            coords,
            direction,
        }
    }

    pub fn move_to(&mut self, c: Coords) -> () {
        self.coords = c;
    }
}

#[derive(PartialEq)]
pub enum CellType {
    Head,
    Middle,
    Tail,
}
