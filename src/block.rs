use rand::thread_rng;
use rand::seq::SliceRandom;

use crate::board::Board;

pub const STARTING_X: u8 = 5;

#[derive(Debug)]
pub struct Block {
    pub x: u8,
    pub y: u8,
    pub states: [[[u8; 4]; 4]; 4],
    pub current_state: u8,
    pub shape: [[u8; 4]; 4],
}

impl Block {
    // clockwise
    pub fn rotate_right(&mut self, board: &Board) {
        let mut curr_state = self.current_state;
        if self.current_state + 1 >= self.states.len() as u8 {
            curr_state = 0;
        } else {
            curr_state += 1;
        }

        if self.test_position(board, curr_state, self.x, self.y) == true {
            self.current_state = curr_state;
            self.shape = self.states[self.current_state as usize];
        }
    }

    // counterclockwise
    pub fn rotate_left(&mut self, board: &Board) {
        let mut curr_state = self.current_state;
        if self.current_state == 0 {
            curr_state = (self.states.len() - 1) as u8;
        } else {
            curr_state -= 1;
        }

        if self.test_position(board, curr_state, self.x, self.y) == true {
            self.current_state = curr_state;
            self.shape = self.states[self.current_state as usize];
        }
    }

    pub fn test_position(&self, board: &Board, state: u8, x: u8, y: u8) -> bool {
        for state_x in 0..4 {
            for state_y in 0..4 {
                if self.states[state as usize][state_y][state_x] != 0 {
                    let board_x = x as usize + state_x;
                    let board_y = y as usize + state_y;
    
                    if board_x >= 10 || board_y >= 20 {
                        return false;
                    }
    
                    if board.get_position(board_x as u8, board_y as u8) != 0 {
                        return false;
                    }
                }
    
            }
        }

        true
    }

    pub fn move_position(&mut self, board: &Board, x: u8, y: u8) -> bool {
        if self.test_position(board, self.current_state, x, y) {
            self.x = x;
            self.y = y;

            true
        } else {
            false
        }
    }
}

trait BlockCreator {
    fn new() -> Block;
}

pub struct I; // I block

impl BlockCreator for I {
    fn new() -> Block {
        Block {
            x: STARTING_X,
            y: 0,
            states: [
                [
                    [0, 0, 0, 0],
                    [1, 1, 1, 1],
                    [0, 0, 0, 0],
                    [0, 0, 0, 0],
                ],
                [
                    [0, 0, 1, 0],
                    [0, 0, 1, 0],
                    [0, 0, 1, 0],
                    [0, 0, 1, 0],
                ],
                [
                    [0, 0, 0, 0],
                    [0, 0, 0, 0],
                    [1, 1, 1, 1],
                    [0, 0, 0, 0],
                ],
                [
                    [0, 1, 0, 0],
                    [0, 1, 0, 0],
                    [0, 1, 0, 0],
                    [0, 1, 0, 0],
                ],
            ],
            current_state: 0,
            shape: [
                [0, 0, 0, 0],
                [1, 1, 1, 1],
                [0, 0, 0, 0],
                [0, 0, 0, 0],
            ]
        }
    }
}

pub struct J; // J block

impl BlockCreator for J {
    fn new() -> Block {
        Block {
            x: STARTING_X,
            y: 0,
            states: [
                [
                    [2, 2, 2, 0],
                    [0, 0, 2, 0],
                    [0, 0, 0, 0],
                    [0, 0, 0, 0],
                ],
                [
                    [0, 2, 0, 0],
                    [0, 2, 0, 0],
                    [2, 2, 0, 0],
                    [0, 0, 0, 0],
                ],
                [
                    [0, 0, 0, 0],
                    [2, 0, 0, 0],
                    [2, 2, 2, 0],
                    [0, 0, 0, 0],
                ],
                [
                    [0, 2, 2, 0],
                    [0, 2, 0, 0],
                    [0, 2, 0, 0],
                    [0, 0, 0, 0],
                ],
            ],
            current_state: 0,
            shape: [
                [2, 2, 2, 0],
                [0, 0, 2, 0],
                [0, 0, 0, 0],
                [0, 0, 0, 0],
            ],
        }
    }
}

pub struct L; // L block

impl BlockCreator for L {
    fn new() -> Block {
        Block {
            x: STARTING_X,
            y: 0,
            states: [
                [
                    [0, 0, 0, 0],
                    [3, 3, 3, 0],
                    [3, 0, 0, 0],
                    [0, 0, 0, 0],
                ],
                [
                    [3, 3, 0, 0],
                    [0, 3, 0, 0],
                    [0, 3, 0, 0],
                    [0, 0, 0, 0],
                ],
                [
                    [0, 0, 0, 0],
                    [0, 0, 3, 0],
                    [3, 3, 3, 0],
                    [0, 0, 0, 0],
                ],
                [
                    [0, 3, 0, 0],
                    [0, 3, 0, 0],
                    [0, 3, 3, 0],
                    [0, 0, 0, 0],
                ],
            ],
            current_state: 0,
            shape: [
                [0, 0, 0, 0],
                [3, 3, 3, 0],
                [3, 0, 0, 0],
                [0, 0, 0, 0],
            ],
        }
    }
}


pub struct O; // O block

impl BlockCreator for O {
    fn new() -> Block {
        Block {
            x: STARTING_X,
            y: 0,
            states: [
                [
                    [4, 4, 0, 0],
                    [4, 4, 0, 0],
                    [0, 0, 0, 0],
                    [0, 0, 0, 0],
                ],
                [
                    [4, 4, 0, 0],
                    [4, 4, 0, 0],
                    [0, 0, 0, 0],
                    [0, 0, 0, 0],
                ],
                [
                    [4, 4, 0, 0],
                    [4, 4, 0, 0],
                    [0, 0, 0, 0],
                    [0, 0, 0, 0],
                ],
                [
                    [4, 4, 0, 0],
                    [4, 4, 0, 0],
                    [0, 0, 0, 0],
                    [0, 0, 0, 0],
                ],
            ],
            current_state: 0,
            shape: [
                [4, 4, 0, 0],
                [4, 4, 0, 0],
                [0, 0, 0, 0],
                [0, 0, 0, 0],
            ],
        }
    }
}


pub struct S; // S block

impl BlockCreator for S {
    fn new() -> Block {
        Block {
            x: STARTING_X,
            y: 0,
            states: [
                [
                    [0, 0, 0, 0],
                    [0, 5, 5, 0],
                    [5, 5, 0, 0],
                    [0, 0, 0, 0],
                ],
                [
                    [5, 0, 0, 0],
                    [5, 5, 0, 0],
                    [0, 5, 0, 0],
                    [0, 0, 0, 0],
                ],
                [
                    [0, 0, 0, 0],
                    [0, 5, 5, 0],
                    [5, 5, 0, 0],
                    [0, 0, 0, 0],
                ],
                [
                    [5, 0, 0, 0],
                    [5, 5, 0, 0],
                    [0, 5, 0, 0],
                    [0, 0, 0, 0],
                ],
            ],
            current_state: 0,
            shape: [
                [0, 0, 0, 0],
                [0, 5, 5, 0],
                [5, 5, 0, 0],
                [0, 0, 0, 0],
            ],
        }
    }
}


pub struct Z; // Z block

impl BlockCreator for Z {
    fn new() -> Block {
        Block {
            x: STARTING_X,
            y: 0,
            states: [
                [
                    [0, 0, 0, 0],
                    [6, 6, 0, 0],
                    [0, 6, 6, 0],
                    [0, 0, 0, 0],
                ],
                [
                    [0, 0, 6, 0],
                    [0, 6, 6, 0],
                    [0, 6, 0, 0],
                    [0, 0, 0, 0],
                ],
                [
                    [0, 0, 0, 0],
                    [6, 6, 0, 0],
                    [0, 6, 6, 0],
                    [0, 0, 0, 0],
                ],
                [
                    [0, 6, 0, 0],
                    [6, 6, 0, 0],
                    [6, 0, 0, 0],
                    [0, 0, 0, 0],
                ],
            ],
            current_state: 0,
            shape: [
                [6, 6, 0, 0],
                [0, 6, 6, 0],
                [0, 0, 0, 0],
                [0, 0, 0, 0],
            ],
        }
    }
}

pub struct T; // T block

impl BlockCreator for T {
    fn new() -> Block {
        Block {
            x: STARTING_X,
            y: 0,
            states: [
                [
                    [0, 7, 0, 0],
                    [7, 7, 7, 0],
                    [0, 0, 0, 0],
                    [0, 0, 0, 0],
                ],
                [
                    [0, 7, 0, 0],
                    [0, 7, 7, 0],
                    [0, 7, 0, 0],
                    [0, 0, 0, 0],
                ],
                [
                    [0, 0, 0, 0],
                    [7, 7, 7, 0],
                    [0, 7, 0, 0],
                    [0, 0, 0, 0],
                ],
                [
                    [0, 7, 0, 0],
                    [7, 7, 0, 0],
                    [0, 7, 0, 0],
                    [0, 0, 0, 0],
                ],
            ],
            current_state: 0,
            shape: [
                [0, 7, 0, 0],
                [7, 7, 7, 0],
                [0, 0, 0, 0],
                [0, 0, 0, 0],
            ],
        }
    }
}

// generate 7 block vectors to ensure uniform block distributions.
pub fn new_chunk() -> [Block; 7]{
    let mut rng = thread_rng();
    // initialize chunk of all 7 block types
    let mut chunk: [Block; 7] = [
        I::new(),
        J::new(),
        L::new(),
        O::new(),
        S::new(),
        Z::new(),
        T::new(),
    ];

    chunk.shuffle(&mut rng);

    chunk
}