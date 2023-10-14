mod sudoku;
use macroquad::prelude::*;
use sudoku::{board::Board, drawer::Drawer};

#[macroquad::main("BasicShapes")]
async fn main() {
    let board = Board::new();
    let start_x = 50.0;
    let start_y = 50.0;
    let drawer = Drawer::new(&board, 50.0);
    loop {
        clear_background(WHITE);

        drawer.draw(start_x, start_y);
        next_frame().await
    }
}
