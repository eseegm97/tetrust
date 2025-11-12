mod game;
mod tetramino;
mod board;

use game::Game;

#[macroquad::main("TetRust")]
async fn main() {
    let mut game = Game::new();
    game.run().await;
}