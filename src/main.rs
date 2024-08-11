mod game;

use game::{Game, Rule};

fn main() {
    let mut game = Game::builder().rule(Rule::new([2].into_iter().collect(), [].into_iter().collect())).random_grid(50).build();

    loop {
        println!("\x1B[2J{}", game);
        game.evolve();
        std::thread::sleep(std::time::Duration::from_millis(250));
    }
}
