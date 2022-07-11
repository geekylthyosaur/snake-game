use rand::distributions::{Distribution, Standard};
use rand::Rng;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Coords {
    pub x: i32,
    pub y: i32,
}

impl Coords {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn translate(&self, max: i32) -> Self {
        Self {
            x: if self.x % max < 0 {
                max - self.x.abs() % max
            } else {
                self.x % max
            },
            y: if self.y % max < 0 {
                max - self.y.abs() % max
            } else {
                self.y % max
            },
        }
    }
}

impl From<Direction> for Coords {
    fn from(d: Direction) -> Self {
        match d {
            Direction::Up => Coords::new(0, -1),
            Direction::Down => Coords::new(0, 1),
            Direction::Right => Coords::new(1, 0),
            Direction::Left => Coords::new(-1, 0),
        }
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

impl std::ops::Sub<Coords> for Coords {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl Distribution<Coords> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Coords {
        let rand_x = rng.gen_range(0..10);
        let rand_y = rng.gen_range(0..10);
        Coords {
            x: rand_x,
            y: rand_y,
        }
    }
}

#[derive(PartialEq, Clone, Copy)]
pub enum Direction {
    Up,
    Down,
    Right,
    Left,
}

impl Direction {
    pub fn is_same_or_opposite(self, other: Self) -> bool {
        let (self_c, other_c): (Coords, Coords) = (self.into(), other.into());
        self == other || self_c + other_c == Coords::new(0, 0)
    }
}

impl From<Coords> for Option<Direction> {
    fn from(c: Coords) -> Self {
        match (c.x, c.y) {
            (x, y) if x == 0 && y == 1 => Some(Direction::Up),
            (x, y) if x == 0 && y == -1 => Some(Direction::Down),
            (x, y) if x == -1 && y == 0 => Some(Direction::Right),
            (x, y) if x == 1 && y == 0 => Some(Direction::Left),
            (_, _) => return None,
        }
    }
}

pub fn window() -> web_sys::Window {
    web_sys::window().expect("No global 'window' exist!")
}

pub fn document() -> web_sys::Document {
    window()
        .document()
        .expect("Should have a document on a window!")
}
