pub struct BoardCell {
    pub number: u8,
    pub fixed: bool,
}

impl BoardCell {
    pub fn new(number: u8) -> Self {
        let mut fixed = false;
        if number > 0 {
            fixed = true;
        }
        Self { number, fixed }
    }
}

pub type BoardType = [[BoardCell; 9]; 9];

pub struct Board {
    board: BoardType,
}

impl<'a> Board {
    pub fn board(&'a self) -> &'a BoardType {
        &self.board
    }
    pub fn new() -> Self {
        Self {
            board: [
                [
                    BoardCell::new(0),
                    BoardCell::new(0),
                    BoardCell::new(0),
                    BoardCell::new(2),
                    BoardCell::new(0),
                    BoardCell::new(0),
                    BoardCell::new(2),
                    BoardCell::new(0),
                    BoardCell::new(0),
                ],
                [
                    BoardCell::new(4),
                    BoardCell::new(0),
                    BoardCell::new(9),
                    BoardCell::new(0),
                    BoardCell::new(0),
                    BoardCell::new(0),
                    BoardCell::new(0),
                    BoardCell::new(0),
                    BoardCell::new(0),
                ],
                [
                    BoardCell::new(0),
                    BoardCell::new(0),
                    BoardCell::new(0),
                    BoardCell::new(0),
                    BoardCell::new(0),
                    BoardCell::new(0),
                    BoardCell::new(0),
                    BoardCell::new(0),
                    BoardCell::new(0),
                ],
                [
                    BoardCell::new(0),
                    BoardCell::new(0),
                    BoardCell::new(0),
                    BoardCell::new(0),
                    BoardCell::new(0),
                    BoardCell::new(0),
                    BoardCell::new(0),
                    BoardCell::new(0),
                    BoardCell::new(0),
                ],
                [
                    BoardCell::new(0),
                    BoardCell::new(0),
                    BoardCell::new(0),
                    BoardCell::new(0),
                    BoardCell::new(0),
                    BoardCell::new(0),
                    BoardCell::new(0),
                    BoardCell::new(0),
                    BoardCell::new(0),
                ],
                [
                    BoardCell::new(0),
                    BoardCell::new(0),
                    BoardCell::new(0),
                    BoardCell::new(0),
                    BoardCell::new(0),
                    BoardCell::new(0),
                    BoardCell::new(0),
                    BoardCell::new(0),
                    BoardCell::new(0),
                ],
                [
                    BoardCell::new(0),
                    BoardCell::new(0),
                    BoardCell::new(0),
                    BoardCell::new(0),
                    BoardCell::new(0),
                    BoardCell::new(0),
                    BoardCell::new(0),
                    BoardCell::new(0),
                    BoardCell::new(0),
                ],
                [
                    BoardCell::new(0),
                    BoardCell::new(0),
                    BoardCell::new(0),
                    BoardCell::new(0),
                    BoardCell::new(0),
                    BoardCell::new(0),
                    BoardCell::new(0),
                    BoardCell::new(0),
                    BoardCell::new(0),
                ],
                [
                    BoardCell::new(0),
                    BoardCell::new(0),
                    BoardCell::new(0),
                    BoardCell::new(0),
                    BoardCell::new(0),
                    BoardCell::new(0),
                    BoardCell::new(0),
                    BoardCell::new(0),
                    BoardCell::new(0),
                ],
            ],
        }
    }
}
