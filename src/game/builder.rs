use super::{Cell, Cell::*, Game, Rule};
use rand::seq::SliceRandom;

pub struct NoGrid;
pub type Grid = Box<[Cell]>;

pub struct Builder<G> {
    size: usize,
    grid: G,
    rule: Rule
}

impl Builder<NoGrid> {
    pub fn new() -> Self {
        Builder {
            size: 0,
            grid: NoGrid,
            rule: Rule::default()
        }
    }

    pub fn grid<const N: usize>(self, grid: [[Cell; N]; N]) -> Builder<Grid> {
        Builder {
            size: N,
            grid: grid.into_iter().flatten().collect(),
            rule: self.rule
        }
    }

    pub fn random_grid(self, size: usize) -> Builder<Grid> {
        const CELL_VARIANTS: [Cell; 2] = [Dead, Live];

        let mut rng = rand::thread_rng();

        Builder {
            size,
            grid: (0..size * size).map(|_| *CELL_VARIANTS.choose(&mut rng).unwrap()).collect(),
            rule: self.rule
        }
    }
}

impl Builder<Grid> {
    pub fn build(self) -> Game {
        Game {
            size: self.size,
            grid: self.grid,
            rule: self.rule
        }
    }
}

impl<G> Builder<G> {
    pub fn rule(self, rule: Rule) -> Self {
        Self {
            size: self.size,
            grid: self.grid,
            rule
        }
    }
}
