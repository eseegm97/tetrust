use macroquad::prelude::*;
use crate::board::Board;

#[derive(Clone, Copy)]
pub enum TetrominoType {
    I, O, T, S, Z, J, L,
}

#[derive(Clone)]
pub struct Tetramino {
    pub kind: TetrominoType,
    pub x: i32,
    pub y: i32,
    pub rotation: u8,
    pub blocks: [[(i32, i32); 4]; 4], // 4 rotation states of 4 blocks each
    pub color: Color,
}

impl Tetramino {
    pub fn random() -> Self {
        match rand::gen_range(0, 7) {
            0 => Self::new(TetrominoType::I),
            1 => Self::new(TetrominoType::O),
            2 => Self::new(TetrominoType::T),
            3 => Self::new(TetrominoType::S),
            4 => Self::new(TetrominoType::Z),
            5 => Self::new(TetrominoType::J),
            _ => Self::new(TetrominoType::L),
        }
    }

    pub fn new(kind: TetrominoType) -> Self {
        let (blocks, color) = match kind {
            TetrominoType::I => (
                [
                    [(0,1), (1,1), (2,1), (3,1)],
                    [(2,0), (2,1), (2,2), (2,3)],
                    [(0,2), (1,2), (2,2), (3,2)],
                    [(1,0), (1,1), (1,2), (1,3)],
                ],
                SKYBLUE,
            ),
            TetrominoType::O => (
                [
                    [(1,0), (2,0), (1,1), (2,1)],
                    [(1,0), (2,0), (1,1), (2,1)],
                    [(1,0), (2,0), (1,1), (2,1)],
                    [(1,0), (2,0), (1,1), (2,1)],
                ],
                YELLOW,
            ),
            TetrominoType::T => (
                [
                    [(1,0), (0,1), (1,1), (2,1)],
                    [(1,0), (1,1), (2,1), (1,2)],
                    [(0,1), (1,1), (2,1), (1,2)],
                    [(1,0), (0,1), (1,1), (1,2)],
                ],
                PURPLE,
            ),
            TetrominoType::S => (
                [
                    [(1,0), (2,0), (0,1), (1,1)],
                    [(1,0), (1,1), (2,1), (2,2)],
                    [(1,1), (2,1), (0,2), (1,2)],
                    [(0,0), (0,1), (1,1), (1,2)],
                ],
                GREEN,
            ),
            TetrominoType::Z => (
                [
                    [(0,0), (1,0), (1,1), (2,1)],
                    [(2,0), (1,1), (2,1), (1,2)],
                    [(0,1), (1,1), (1,2), (2,2)],
                    [(1,0), (0,1), (1,1), (0,2)],
                ],
                RED,
            ),
            TetrominoType::J => (
                [
                    [(0,0), (0,1), (1,1), (2,1)],
                    [(1,0), (2,0), (1,1), (1,2)],
                    [(0,1), (1,1), (2,1), (2,2)],
                    [(1,0), (1,1), (0,2), (1,2)],
                ],
                BLUE,
            ),
            TetrominoType::L => (
                [
                    [(2,0), (0,1), (1,1), (2,1)],
                    [(1,0), (1,1), (1,2), (2,2)],
                    [(0,1), (1,1), (2,1), (0,2)],
                    [(0,0), (1,0), (1,1), (1,2)],
                ],
                ORANGE,
            ),
        };

        Tetramino {
            kind,
            x: 3,
            y: 0,
            rotation: 0,
            blocks,
            color,
        }
    }

    pub fn move_left(&mut self, board: &Board) {
        self.x -= 1;
        if board.check_collision(self) {
            self.x += 1;
        }
    }

    pub fn move_right(&mut self, board: &Board) {
        self.x += 1;
        if board.check_collision(self) {
            self.x -= 1;
        }
    }

    pub fn move_down(&mut self, board: &Board) -> bool {
        self.y += 1;
        if board.check_collision(self) {
            self.y -= 1;
            return false;
        }
        true
    }

    pub fn rotate(&mut self, board: &Board) {
        let old_rotation = self.rotation;
        self.rotation = (self.rotation + 1) % 4;
        if board.check_collision(self) {
            self.rotation = old_rotation; // revert if collides
        }
    }

    pub fn draw(&self) {
        let block_size = 24.0;
        for &(dx, dy) in &self.blocks[self.rotation as usize] {
            let x = (self.x + dx) as f32 * block_size;
            let y = (self.y + dy) as f32 * block_size;
            draw_rectangle(x, y, block_size - 1.0, block_size - 1.0, self.color);
        }
    }
}