use macroquad::prelude::*;
use crate::tetramino::Tetramino;

pub const BOARD_WIDTH: usize = 10;
pub const BOARD_HEIGHT: usize = 20;
const BLOCK_SIZE: f32 = 24.0;

#[derive(Clone, Copy)]
pub struct Cell {
    pub filled: bool,
    pub color: Color,
}

pub struct Board {
    pub grid: [[Cell; BOARD_WIDTH]; BOARD_HEIGHT],
}

impl Board {
    pub fn new() -> Self {
        Self {
            grid: [[Cell { filled: false, color: DARKGRAY }; BOARD_WIDTH]; BOARD_HEIGHT],
        }
    }

    pub fn check_collision(&self, piece: &Tetramino) -> bool {
        for &(dx, dy) in &piece.blocks[piece.rotation as usize] {
            let x = piece.x + dx;
            let y = piece.y + dy;

            if x < 0 || x >= BOARD_WIDTH as i32 || y >= BOARD_HEIGHT as i32 {
                return true; // out of bounds
            }
            if y >= 0 && self.grid[y as usize][x as usize].filled {
                return true; // collides with filled cell
            }
        }
        false
    }

    pub fn place_piece(&mut self, piece: &Tetramino) {
        for &(dx, dy) in &piece.blocks[piece.rotation as usize] {
            let x = piece.x + dx;
            let y = piece.y + dy;

            if y >= 0 && y < BOARD_HEIGHT as i32 && x >= 0 && x < BOARD_WIDTH as i32 {
                self.grid[y as usize][x as usize] = Cell {
                    filled: true,
                    color: piece.color,
                };
            }
        }
    }

    pub fn clear_full_lines(&mut self) {
        let mut new_grid = [[Cell { filled: false, color: DARKGRAY }; BOARD_WIDTH]; BOARD_HEIGHT];
        let mut new_row = BOARD_HEIGHT as i32 - 1;

        for y in (0..BOARD_HEIGHT).rev() {
            if self.grid[y].iter().all(|cell| cell.filled) {
                continue; // skip full rows (cleared)
            }

            new_grid[new_row as usize] = self.grid[y];
            new_row -= 1;
        }

        self.grid = new_grid;
    }

    pub fn draw(&self) {
        // draw filled cells
        for y in 0..BOARD_HEIGHT {
            for x in 0..BOARD_WIDTH {
                let cell = &self.grid[y][x];
                if cell.filled {
                    draw_rectangle(
                        x as f32 * BLOCK_SIZE,
                        y as f32 * BLOCK_SIZE,
                        BLOCK_SIZE - 1.0,
                        BLOCK_SIZE - 1.0,
                        cell.color,
                    );
                } else {
                    draw_rectangle_lines(
                        x as f32 * BLOCK_SIZE,
                        y as f32 * BLOCK_SIZE,
                        BLOCK_SIZE,
                        BLOCK_SIZE,
                        1.0,
                        DARKGRAY,
                    );
                }
            }
        }
    }
}