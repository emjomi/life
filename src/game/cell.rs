use rand::{Rng, distributions::{Distribution, Standard}};

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Cell {
    Dead,
    Live,
}

impl Distribution<Cell> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Cell {
        match rng.gen_range(0..=1) {
            0 => Cell::Dead,
            _ => Cell::Live
        }
    }
}