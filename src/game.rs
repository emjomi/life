mod builder;
mod cell;
mod rule;

use std::fmt;
use builder::{Builder, NoGrid};
pub use cell::{Cell, Cell::*};
pub use rule::Rule;

#[derive(Debug)]
pub struct Game {
    size: usize,
    grid: Box<[Cell]>,
    rule: Rule
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
            }).count() as u8;

            match (cell, neighbors) {
                (Dead, n) if self.rule.is_born(n) => Live,
                (Live, n) if self.rule.is_survivor(n) => Live,
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
    fn radar_seeds_automaton() {
        let mut radar = Game::builder().rule(Rule::new([2].into_iter().collect(), [].into_iter().collect())).grid([
            [Dead, Dead, Dead, Dead, Dead, Dead],
            [Dead, Live, Dead, Dead, Dead, Dead],
            [Dead, Dead, Live, Live, Dead, Dead],
            [Dead, Dead, Live, Live, Dead, Dead],
            [Dead, Dead, Dead, Dead, Live, Dead],
            [Dead, Dead, Dead, Dead, Dead, Dead],
        ]).build();
        
        let initial_grid = radar.grid.clone();

        radar.evolve();
        radar.evolve();
        radar.evolve();
        radar.evolve();

        assert_eq!(radar.grid, initial_grid);
    }
    
    #[test]
    fn flock_predecessor_flock_automaton() {
        let mut flock_predecessor = Game::builder().rule(Rule::new([3].into_iter().collect(), [1, 2].into_iter().collect())).grid([
            [Dead, Dead, Dead, Dead, Dead, Dead, Dead],
            [Dead, Dead, Dead, Dead, Dead, Dead, Dead],
            [Dead, Dead, Live, Live, Live, Dead, Dead],
            [Dead, Dead, Live, Dead, Live, Dead, Dead],
            [Dead, Dead, Live, Live, Live, Dead, Dead],
            [Dead, Dead, Dead, Dead, Dead, Dead, Dead],
            [Dead, Dead, Dead, Dead, Dead, Dead, Dead],
        ]).build();
        
        flock_predecessor.evolve();
        flock_predecessor.evolve();
        flock_predecessor.evolve();
        
        let flock_grid = flock_predecessor.grid.clone();

        flock_predecessor.evolve();

        assert_eq!(flock_predecessor.grid, flock_grid);
    }
    
    #[test]
    fn moon_iceballs_automaton() {
        let mut moon = Game::builder().rule(Rule::new([2, 5, 6, 7, 8].into_iter().collect(), (5..=8).into_iter().collect())).grid([
            [Dead, Dead, Dead, Dead, Dead, Dead],
            [Dead, Dead, Dead, Dead, Dead, Dead],
            [Dead, Dead, Dead, Dead, Dead, Dead],
            [Dead, Dead, Dead, Dead, Dead, Dead],
            [Dead, Dead, Live, Live, Dead, Dead],
            [Dead, Live, Dead, Dead, Live, Dead],
        ]).build();

        let initial_grid = moon.grid.clone();
        
        moon.evolve();
        moon.evolve();
        moon.evolve();
        moon.evolve();
        moon.evolve();
        moon.evolve();

        assert_eq!(moon.grid, initial_grid);
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