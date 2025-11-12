mod game;
mod tetramino;
mod board;

use game::Game;
use macroquad::prelude::*;

#[macroquad::main("Tetris Clone")]
async fn main() {
    let mut game = Game::new();
    game.run().await;
}