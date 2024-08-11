use rand::seq::SliceRandom;
use std::fmt;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Cell {
    Dead,
    Live,
}

#[derive(Debug, Clone)]
pub struct Game {
    size: usize,
    grid: Box<[Cell]>,
}

impl fmt::Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut result = String::with_capacity(self.size * (self.size + 1));
        for row in self.grid.chunks(self.size) {
            for cell in row {
                result.push(match cell {
                    Cell::Dead => ' ',
                    Cell::Live => '@',
                });
            }
            result.push('\n');
        }
        write!(f, "{}", result)
    }
}

impl<const N: usize> From<[[Cell; N]; N]> for Game {
    fn from(value: [[Cell; N]; N]) -> Self {
        Game {
            size: N,
            grid: value.into_iter().flatten().collect(),
        }
    }
}

impl Game {
    pub fn random(size: usize) -> Self {
        const CELL_VARIANTS: [Cell; 2] = [Cell::Dead, Cell::Live];

        let mut rng = rand::thread_rng();

        Game {
            size,
            grid: (0..size * size).map(|_| *CELL_VARIANTS.choose(&mut rng).unwrap()).collect()
        }
    }
    
    pub fn evolve(&mut self) {
        const NEIGHBOR_OFFSETS: [(isize, isize); 8] = [
            (0, 1), (-1, 1), (-1, 0), (-1, -1),
            (0, -1), (1, -1), (1, 0), (1, 1),
        ];

        let next_cells = self.grid.iter().enumerate().map(|(i, cell)| {
            let row = i / self.size;
            let col = i % self.size;

            let neighbors = NEIGHBOR_OFFSETS.iter().filter(|(dx, dy)| {
                    let neighbor_row = (row as isize + dx).rem_euclid(self.size as isize) as usize;
                    let neighbor_col = (col as isize + dy).rem_euclid(self.size as isize) as usize;
                    self.grid[neighbor_row * self.size + neighbor_col] == Cell::Live
                }).count();

            match (cell, neighbors) {
                (Cell::Dead, 3) => Cell::Live,
                (Cell::Live, 2 | 3) => Cell::Live,
                _ => Cell::Dead,
            }
        }).collect();
        self.grid = next_cells;
    }
    
    pub fn clear_grid(&mut self) {
        self.grid = (0..self.grid.len()).map(|_| Cell::Dead).collect();
    }
}

#[cfg(test)]
mod tests {
    use super::{Cell::*, Game};

    #[test]
    fn lower_right_blinker() {
        let blinker = Game::from([
            [Dead, Dead, Dead, Dead],
            [Dead, Dead, Dead, Dead],
            [Dead, Dead, Dead, Dead],
            [Live, Dead, Live, Live],
        ]);
        let mut evolved_blinker = blinker.clone();

        evolved_blinker.evolve();
        evolved_blinker.evolve();

        assert_eq!(blinker.grid, evolved_blinker.grid);
    }

    #[test]
    fn upper_left_blinker() {
        let blinker = Game::from([
            [Live, Live, Dead, Live],
            [Dead, Dead, Dead, Dead],
            [Dead, Dead, Dead, Dead],
            [Dead, Dead, Dead, Dead],
        ]);
        let mut evolved_blinker = blinker.clone();

        evolved_blinker.evolve();
        evolved_blinker.evolve();

        assert_eq!(blinker.grid, evolved_blinker.grid);
    }

    #[test]
    fn glider() {
        let mut glider = Game::from([
            [Dead, Dead, Dead, Dead, Dead],
            [Dead, Dead, Dead, Dead, Dead],
            [Dead, Dead, Dead, Live, Dead],
            [Dead, Dead, Dead, Dead, Live],
            [Dead, Dead, Live, Live, Live],
        ]);

        glider.evolve();
        assert_eq!(glider.grid, 
            [
                Dead, Dead, Dead, Live, Dead,
                Dead, Dead, Dead, Dead, Dead,
                Dead, Dead, Dead, Dead, Dead,
                Dead, Dead, Live, Dead, Live,
                Dead, Dead, Dead, Live, Live
            ].into_iter().collect()
        );

        glider.evolve();
        assert_eq!(glider.grid,
            [
                Dead, Dead, Dead, Live, Live,
                Dead, Dead, Dead, Dead, Dead,
                Dead, Dead, Dead, Dead, Dead,
                Dead, Dead, Dead, Dead, Live,
                Dead, Dead, Live, Dead, Live
            ].into_iter().collect()
        );

        glider.evolve();
        assert_eq!(glider.grid,
            [
                Dead, Dead, Dead, Live, Live,
                Dead, Dead, Dead, Dead, Dead,
                Dead, Dead, Dead, Dead, Dead,
                Dead, Dead, Dead, Live, Dead,
                Live, Dead, Dead, Dead, Live
            ].into_iter().collect()
        );

        glider.evolve();
        assert_eq!(glider.grid,
            [
                Live, Dead, Dead, Live, Live,
                Dead, Dead, Dead, Dead, Dead,
                Dead, Dead, Dead, Dead, Dead,
                Dead, Dead, Dead, Dead, Live,
                Live, Dead, Dead, Dead, Dead
            ].into_iter().collect()
        );
    }
    
    #[test]
    fn clear_grid() {
        let mut blinker = Game::from([
            [Dead, Dead, Dead, Dead],
            [Dead, Dead, Dead, Dead],
            [Dead, Dead, Dead, Dead],
            [Live, Dead, Live, Live],
        ]);

        blinker.clear_grid();

        assert_eq!(blinker.grid, [Dead; 16].into_iter().collect());
    }
}
