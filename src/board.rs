#[derive(Debug)]
pub struct Board {
    pub width: u8,
    pub height: u8,
    pub board_map: [[u8; 10]; 20],
}


impl Board {
    pub fn new() -> Board {
        Board {
            width: 10,
            height: 20,
            board_map: [[0u8; 10]; 20]
        }
    }

    pub fn get_position(&self, x: u8, y: u8) -> u8 {
        self.board_map[y as usize][x as usize]
    }

    pub fn set_position(&mut self, x: usize, y: usize, value: u8) {
        self.board_map[y][x] = value;
    }
}