mod game;
mod tetramino;
mod board;

use game::Game;

fn main() {
    let mut game = Game::new();
    game.run();
}