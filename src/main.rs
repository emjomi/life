mod game;

use game::{Grid, Cell::*};

fn main() {
    let mut glider = Grid::from([
        [Dead, Dead, Dead, Dead, Dead],
        [Dead, Dead, Dead, Dead, Dead],
        [Dead, Dead, Dead, Live, Dead],
        [Dead, Dead, Dead, Dead, Live],
        [Dead, Dead, Live, Live, Live],
    ]);
    
    loop {
        println!("\x1B[2J{}", glider);
        glider.evolve();
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
}
