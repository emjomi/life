mod game;

use game::Game;

fn main() {
    let mut game = Game::builder().random_grid(15).build();

    loop {
        println!("\x1B[2J{}", game);
        game.evolve();
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
}
