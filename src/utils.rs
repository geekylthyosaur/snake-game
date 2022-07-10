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
    pub fn is_same_or_opposite(&self, other: &Self) -> bool {
        self == other || self.value() + other.value() == Coords::new(0, 0)
    }

    pub fn value(&self) -> Coords {
        match self {
            Self::Up => Coords::new(0, -1),
            Self::Down => Coords::new(0, 1),
            Self::Right => Coords::new(1, 0),
            Self::Left => Coords::new(-1, 0),
        }
    }
}
