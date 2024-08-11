mod builder;
mod cell;
mod rule;

use std::fmt;
use builder::{Builder, NoGrid};
use cell::{Cell, Cell::*};

#[derive(Debug)]
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
                    Dead => ' ',
                    Live => '@',
                });
            }
            result.push('\n');
        }
        write!(f, "{}", result)
    }
}

impl Game {
    pub fn builder() -> Builder<NoGrid> {
        Builder::<NoGrid>::new()
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
                self.grid[neighbor_row * self.size + neighbor_col] == Live
            }).count();

            match (cell, neighbors) {
                (Dead, 3) => Live,
                (Live, 2 | 3) => Live,
                _ => Dead,
            }
        }).collect();
        self.grid = next_cells;
    }
    
    pub fn clear_grid(&mut self) {
        self.grid = (0..self.grid.len()).map(|_| Dead).collect();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lower_right_blinker() {
        let mut blinker = Game::builder().grid([
            [Dead, Dead, Dead, Dead],
            [Dead, Dead, Dead, Dead],
            [Dead, Dead, Dead, Dead],
            [Live, Dead, Live, Live],
        ]).build();
        
        let initial_grid = blinker.grid.clone();

        blinker.evolve();
        blinker.evolve();

        assert_eq!(blinker.grid, initial_grid);
    }

    #[test]
    fn upper_left_blinker() {
        let mut blinker = Game::builder().grid([
            [Live, Live, Dead, Live],
            [Dead, Dead, Dead, Dead],
            [Dead, Dead, Dead, Dead],
            [Dead, Dead, Dead, Dead],
        ]).build();
        
        let initial_grid = blinker.grid.clone();

        blinker.evolve();
        blinker.evolve();

        assert_eq!(blinker.grid, initial_grid);
    }

    #[test]
    fn glider() {
        let mut glider = Game::builder().grid([
            [Dead, Dead, Dead, Dead, Dead],
            [Dead, Dead, Dead, Dead, Dead],
            [Dead, Dead, Dead, Live, Dead],
            [Dead, Dead, Dead, Dead, Live],
            [Dead, Dead, Live, Live, Live],
        ]).build();

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
        let mut blinker = Game::builder().grid([
            [Dead, Dead, Dead, Dead],
            [Dead, Dead, Dead, Dead],
            [Dead, Dead, Dead, Dead],
            [Live, Dead, Live, Live],
        ]).build();

        blinker.clear_grid();

        assert_eq!(blinker.grid, [Dead; 16].into_iter().collect());
    }
}