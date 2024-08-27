use super::{Cell, Engine, Rule};

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
        Builder {
            size,
            grid: (0..size.pow(2)).map(|_| rand::random()).collect(),
            rule: self.rule
        }
    }
}

impl Builder<Grid> {
    pub fn build(self) -> Engine {
        Engine {
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
