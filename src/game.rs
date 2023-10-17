use crate::block::{new_chunk, Block, STARTING_X};
use crate::board::Board;
use crate::colors::get_color;
use crate::input::Input;

use crossterm::cursor::MoveTo;
use crossterm::execute;
use crossterm::style::{Print, SetForegroundColor, ResetColor};
use std::io::Stdout;

const MARGIN: u8 = 2; // side margin for cursor calculations

#[derive(Debug)]
pub struct Game {
    pub chunk: [Block; 7],
    pub current_block: u8, // current block in chunk
    lines: u8, // cleared lines
    score: u16,
    board: Board,
    game_over: bool,
    counter: u8
}

impl Game {
    pub fn new() -> Game {
        Game {
            chunk: new_chunk(),
            current_block: 0,
            lines: 0,
            score: 0,
            board: Board::new(),
            game_over: false,
            counter: 0,
        }
    }

    pub fn handle_falling(&mut self, stdout: &Stdout) {
        self.counter += 1;
        if self.counter == 2 {
            self.clear(stdout);
            if self.chunk[self.current_block as usize].y < self.board.height - 3 {
                self.chunk[self.current_block as usize].y += 1;
            } else {
                // solidify block
                self.place_current_block();
                
                self.current_block += 1;

                if self.current_block >= 7 {
                    self.current_block = 0;
                    self.chunk = new_chunk();
                }
            }
            self.counter = 0;
            self.render(stdout)
        }
    }

    pub fn update(&mut self, stdout: &Stdout) {
        if self.game_over {
            return;
        } else {
            if self.current_block > 7 {
                self.current_block = 0;
                self.chunk = new_chunk();
            }

            self.render(stdout);
        }
    }

    pub fn render(&self, mut stdout: &Stdout) {
        for y in 0..self.board.board_map.len() {
            for x in 0..self.board.board_map[y].len() {
                if self.board.board_map[y][x] != 0 {
                    execute!(stdout, MoveTo(x as u16 * 2, y as u16), SetForegroundColor(get_color(self.board.board_map[y][x])), Print("██"), ResetColor).unwrap();
                } 
            }
            execute!(stdout, MoveTo(0, y as u16)).unwrap();
        }

        // print current active block
        for x in 0..4 {
            for y in 0..4 {
                if self.chunk[self.current_block as usize].shape[y][x] != 0 && self.board.board_map[y + self.chunk[self.current_block as usize].y as usize][x + self.chunk[self.current_block as usize].x as usize] == 0 {
                    execute!(stdout, MoveTo((x + self.chunk[self.current_block as usize].x as usize) as u16 * 2 + MARGIN as u16, (y + self.chunk[self.current_block as usize].y as usize) as u16)).unwrap();
                    execute!(stdout, SetForegroundColor(get_color(self.chunk[self.current_block as usize].shape[y][x])), Print("██"), ResetColor).unwrap();
                }
            }
        }
    }

    pub fn init(&self, mut stdout: &Stdout) {
        for y in 0..self.board.height {
            if y + 1 < self.board.height {
                execute!(stdout, MoveTo(0, y as u16), Print("<>")).unwrap();
                execute!(stdout, MoveTo(self.board.width as u16 * 2 + MARGIN as u16, y as u16), Print("<>")).unwrap();
            } else {
                for x in 0..(self.board.width + MARGIN) {
                    execute!(stdout, MoveTo(x as u16 * 2, y as u16), Print("<>")).unwrap();
                }
            }
        } 
    }

    fn clear(&self, mut stdout: &Stdout) {
        for x in 0..4 {
            for y in 0..4 {
                if self.chunk[self.current_block as usize].shape[y][x] != 0 && self.board.board_map[y + self.chunk[self.current_block as usize].y as usize][x + self.chunk[self.current_block as usize].x as usize] == 0 {
                    execute!(stdout, MoveTo((x + self.chunk[self.current_block as usize].x as usize) as u16 * 2 + MARGIN as u16, (y + self.chunk[self.current_block as usize].y as usize) as u16)).unwrap();
                    execute!(stdout, Print("  ")).unwrap();
                }
            }
        }
    }

    pub fn place_current_block(&mut self) {
        let current_block = &mut self.chunk[self.current_block as usize];
        let x = current_block.x as usize;
        let y = current_block.y as usize;

        for (block_y, row) in current_block.shape.iter().enumerate() {
            for (block_x, &value) in row.iter().enumerate() {
                if value != 0 {
                    let board_x = x + block_x;
                    let board_y = y + block_y;
                    self.board.set_position(board_x, board_y, value);
                }
            }
        }

        // self.board.clear_completed_rows();

        current_block.x = STARTING_X;
        current_block.y = 0;
        current_block.current_state = 0;
        current_block.shape = current_block.states[current_block.current_state as usize];
    }

    pub fn handle_input(&mut self, input: Input, stdout: &Stdout) {
        match input {
            Input::Left => {
                self.clear(stdout);
                if self.chunk[self.current_block as usize].x > 0 {
                    self.chunk[self.current_block as usize].move_position(&self.board, self.chunk[self.current_block as usize].x - 1, self.chunk[self.current_block as usize].y);
                } 
            },
            Input::Right => {
                self.clear(stdout);
                self.chunk[self.current_block as usize].move_position(&self.board, self.chunk[self.current_block as usize].x + 1, self.chunk[self.current_block as usize].y);
            },
            Input::Clockwise => {
                self.clear(stdout);
                self.chunk[self.current_block as usize].rotate_right(&self.board);
            },
            Input::Counterclockwise => {
                self.clear(stdout);
                self.chunk[self.current_block as usize].rotate_left(&self.board);
            },
            Input::Drop => {
                todo!();
            }
        }
    }
}