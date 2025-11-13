use macroquad::prelude::*;
use crate::board::Board;
use crate::tetramino::{Tetramino, TetrominoType};

// Scoring constants
const SCORE_SINGLE: u32 = 100;
const SCORE_DOUBLE: u32 = 300;
const SCORE_TRIPLE: u32 = 500;
const SCORE_TETRIS: u32 = 800;
const LINES_PER_LEVEL: u32 = 10;

pub struct Game {
    board: Board,
    current_piece: Tetramino,
    drop_timer: f32,
    drop_speed: f32,
    game_over: bool,
    paused: bool,
    score: u32,
    lines_cleared: u32,
    level: u32,
    base_drop_speed: f32,
    left_move_timer: f32,
    right_move_timer: f32,
    move_repeat_delay: f32,
    move_repeat_rate: f32,
    last_tetramino_type: Option<TetrominoType>,
}

impl Game {
    pub fn new() -> Self {
        let base_speed = 0.8;
        let first_piece = Tetramino::random();
        let first_type = first_piece.get_type();
        
        Self {
            board: Board::new(),
            current_piece: first_piece,
            drop_timer: 0.0,
            drop_speed: base_speed,
            game_over: false,
            paused: false,
            score: 0,
            lines_cleared: 0,
            level: 1,
            base_drop_speed: base_speed,
            left_move_timer: 0.0,
            right_move_timer: 0.0,
            move_repeat_delay: 0.15, // Initial delay before repeating (150ms)
            move_repeat_rate: 0.05,  // Time between repeats (50ms)
            last_tetramino_type: Some(first_type),
        }
    }

    fn calculate_score(&self, lines_cleared: u32) -> u32 {
        let base_score = match lines_cleared {
            1 => SCORE_SINGLE,
            2 => SCORE_DOUBLE,
            3 => SCORE_TRIPLE,
            4 => SCORE_TETRIS,
            _ => 0,
        };
        base_score * self.level
    }

    fn update_level(&mut self) {
        let new_level = (self.lines_cleared / LINES_PER_LEVEL) + 1;
        if new_level != self.level {
            self.level = new_level;
            // Increase speed with each level (faster drop)
            self.drop_speed = self.base_drop_speed * (0.9_f32.powf((self.level - 1) as f32));
        }
    }

    pub async fn run(&mut self) {
        loop {
            if self.game_over {
                clear_background(BLACK);
                draw_text("Game Over", 100.0, 200.0, 40.0, RED);
                draw_text(&format!("Final Score: {}", self.score), 100.0, 250.0, 30.0, WHITE);
                draw_text(&format!("Lines Cleared: {}", self.lines_cleared), 100.0, 280.0, 30.0, WHITE);
                draw_text(&format!("Level Reached: {}", self.level), 100.0, 310.0, 30.0, WHITE);
                draw_text("Press R to restart", 100.0, 360.0, 30.0, YELLOW);
                draw_text("Press Q to quit", 100.0, 390.0, 30.0, YELLOW);
                
                if is_key_pressed(KeyCode::R) {
                    *self = Game::new();
                }
                if is_key_pressed(KeyCode::Q) {
                    std::process::exit(0);
                }
                next_frame().await;
                continue;
            }

            // Always handle pause input, even when paused
            if is_key_pressed(KeyCode::Space) {
                self.paused = !self.paused;
            }

            if !self.paused {
                self.handle_input();
                self.update();
            }
            
            self.render();

            next_frame().await;
        }
    }

    fn handle_input(&mut self) {
        let frame_time = get_frame_time();

        // Handle left movement
        if is_key_down(KeyCode::Left) {
            if is_key_pressed(KeyCode::Left) {
                // First press - immediate movement
                self.current_piece.move_left(&self.board);
                self.left_move_timer = 0.0;
            } else {
                // Key is held down - check for repeat
                self.left_move_timer += frame_time;
                if self.left_move_timer >= self.move_repeat_delay {
                    // Calculate how many moves should have happened
                    let time_since_delay = self.left_move_timer - self.move_repeat_delay;
                    let moves_count = (time_since_delay / self.move_repeat_rate) as i32 + 1;
                    
                    // Move and reset timer appropriately
                    self.current_piece.move_left(&self.board);
                    self.left_move_timer = self.move_repeat_delay + ((moves_count as f32) * self.move_repeat_rate);
                }
            }
        } else {
            self.left_move_timer = 0.0;
        }

        // Handle right movement
        if is_key_down(KeyCode::Right) {
            if is_key_pressed(KeyCode::Right) {
                // First press - immediate movement
                self.current_piece.move_right(&self.board);
                self.right_move_timer = 0.0;
            } else {
                // Key is held down - check for repeat
                self.right_move_timer += frame_time;
                if self.right_move_timer >= self.move_repeat_delay {
                    // Calculate how many moves should have happened
                    let time_since_delay = self.right_move_timer - self.move_repeat_delay;
                    let moves_count = (time_since_delay / self.move_repeat_rate) as i32 + 1;
                    
                    // Move and reset timer appropriately
                    self.current_piece.move_right(&self.board);
                    self.right_move_timer = self.move_repeat_delay + ((moves_count as f32) * self.move_repeat_rate);
                }
            }
        } else {
            self.right_move_timer = 0.0;
        }

        // Handle rotation (single press only)
        if is_key_pressed(KeyCode::Z) {
            self.current_piece.rotate_counter_clockwise(&self.board);
        }
        if is_key_pressed(KeyCode::X) {
            self.current_piece.rotate_clockwise(&self.board);
        }

        // Handle fast drop
        if is_key_down(KeyCode::Down) {
            self.drop_speed = 0.05; // faster fall when holding down
        } else {
            // Use level-adjusted speed when not pressing down
            self.drop_speed = self.base_drop_speed * (0.9_f32.powf((self.level - 1) as f32));
        }
    }

    fn update(&mut self) {
        self.drop_timer += get_frame_time();

        if self.drop_timer >= self.drop_speed {
            self.drop_timer = 0.0;

            if !self.current_piece.move_down(&self.board) {
                // lock piece into board
                self.board.place_piece(&self.current_piece);

                // clear full lines and calculate score
                let cleared_lines = self.board.clear_full_lines();
                if cleared_lines > 0 {
                    let points = self.calculate_score(cleared_lines);
                    self.score += points;
                    self.lines_cleared += cleared_lines;
                    self.update_level();
                }

                // spawn new piece (avoiding same type as previous)
                if let Some(last_type) = self.last_tetramino_type {
                    self.current_piece = Tetramino::random_excluding(last_type);
                } else {
                    self.current_piece = Tetramino::random();
                }
                self.last_tetramino_type = Some(self.current_piece.get_type());

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

        // Draw scoring information
        let text_color = WHITE;
        let font_size = 20.0;
        let info_x = 260.0; // Position to the right of the game board

        draw_text(
            &format!("Score: {}", self.score),
            info_x, 40.0, font_size, text_color
        );

        draw_text(
            &format!("Lines: {}", self.lines_cleared),
            info_x, 70.0, font_size, text_color
        );

        draw_text(
            &format!("Level: {}", self.level),
            info_x, 100.0, font_size, text_color
        );

        // Draw scoring guide
        draw_text("Scoring:", info_x, 150.0, font_size, YELLOW);
        draw_text("Single: 100 × Level", info_x, 180.0, 16.0, LIGHTGRAY);
        draw_text("Double: 300 × Level", info_x, 200.0, 16.0, LIGHTGRAY);
        draw_text("Triple: 500 × Level", info_x, 220.0, 16.0, LIGHTGRAY);
        draw_text("Tetris: 800 × Level", info_x, 240.0, 16.0, LIGHTGRAY);

        // Controls
        draw_text("Controls:", info_x, 280.0, font_size, YELLOW);
        draw_text("Left/Right: Move Left/Right", info_x, 310.0, 16.0, LIGHTGRAY);
        draw_text("Z: Rotate Left", info_x, 330.0, 16.0, LIGHTGRAY);
        draw_text("X: Rotate Right", info_x, 350.0, 16.0, LIGHTGRAY);
        draw_text("Down Arrow: Fast Drop", info_x, 370.0, 16.0, LIGHTGRAY);
        draw_text("Space: Pause/Unpause", info_x, 390.0, 16.0, LIGHTGRAY);

        // Pause indicator
        if self.paused {
            // Draw semi-transparent overlay
            draw_rectangle(0.0, 0.0, screen_width(), screen_height(), Color::new(0.0, 0.0, 0.0, 0.5));
            
            // Draw pause text in center of screen
            let pause_text = "PAUSED";
            let pause_font_size = 48.0;
            let text_width = measure_text(pause_text, None, pause_font_size as u16, 1.0).width;
            let center_x = (screen_width() - text_width) / 2.0;
            let center_y = screen_height() / 2.0;
            
            draw_text(pause_text, center_x, center_y, pause_font_size, WHITE);
            draw_text("Press SPACE to continue", center_x - 50.0, center_y + 50.0, 20.0, YELLOW);
        }
    }
}