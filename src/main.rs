mod game;

use game::Grid;

fn main() {
    let mut grid = Grid::random(15);
    
    loop {
        println!("\x1B[2J{}", grid);
        grid.evolve();
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
}