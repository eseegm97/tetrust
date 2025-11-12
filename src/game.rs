use macroquad::prelude::*;
use crate::board::Board;
use crate::tetramino::Tetramino;

pub struct Game {
    board: Board,
    current_piece: Tetramino,
    drop_timer: f32,
    drop_speed: f32,
    game_over: bool,
}

impl Game {
    pub fn new() -> Self {
        Self {
            board: Board::new(),
            current_piece: Tetramino::random(),
            drop_timer: 0.0,
            drop_speed: 0.5, // seconds between drops
            game_over: false,
        }
    }

    pub async fn run(&mut self) {
        loop {
            if self.game_over {
                clear_background(BLACK);
                draw_text("Game Over", 100.0, 200.0, 40.0, RED);
                draw_text("Press R to restart", 100.0, 250.0, 30.0, WHITE);
                if is_key_pressed(KeyCode::R) {
                    *self = Game::new();
                }
                next_frame().await;
                continue;
            }

            self.handle_input();
            self.update();
            self.render();

            next_frame().await;
        }
    }

    fn handle_input(&mut self) {
        if is_key_pressed(KeyCode::Left) {
            self.current_piece.move_left(&self.board);
        }
        if is_key_pressed(KeyCode::Right) {
            self.current_piece.move_right(&self.board);
        }
        if is_key_pressed(KeyCode::Up) {
            self.current_piece.rotate(&self.board);
        }
        if is_key_down(KeyCode::Down) {
            self.drop_speed = 0.05; // faster fall when holding down
        } else {
            self.drop_speed = 0.5;
        }
    }

    fn update(&mut self) {
        self.drop_timer += get_frame_time();

        if self.drop_timer >= self.drop_speed {
            self.drop_timer = 0.0;

            if !self.current_piece.move_down(&self.board) {
                // lock piece into board
                self.board.place_piece(&self.current_piece);

                // clear full lines
                self.board.clear_full_lines();

                // spawn new piece
                self.current_piece = Tetramino::random();

                // check for game over
                if self.board.check_collision(&self.current_piece) {
                    self.game_over = true;
                }
            }
        }
    }

    fn render(&self) {
        clear_background(BLACK);
        self.board.draw();
        self.current_piece.draw();
    }
}