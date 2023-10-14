use macroquad::{
    prelude::{BLACK, BLUE, GRAY},
    shapes::{draw_line, draw_rectangle_lines},
    text::draw_text,
};

use super::board::Board;

pub struct Drawer<'a> {
    board: &'a Board,
    size: f32,
}

impl<'a> Drawer<'a> {
    pub fn new(board: &'a Board, size: f32) -> Self {
        Self { board, size }
    }

    pub fn draw(&self, start_x: f32, start_y: f32) {
        for (i, row) in self.board.board().iter().enumerate() {
            for (j, cell) in row.iter().enumerate() {
                let x = start_x + (i as f32 * self.size);
                let y = start_y + (j as f32 * self.size);
                let color = {
                    match cell.fixed {
                        true => BLUE,
                        false => BLACK,
                    }
                };

                draw_rectangle_lines(x, y, self.size, self.size, 2.0, GRAY);
                if cell.number > 0 {
                    draw_text(
                        format!("{}", cell.number).as_str(),
                        x + 12.0,
                        y + 40.0,
                        self.size,
                        color,
                    );
                }
            }
        }
        draw_rectangle_lines(
            start_x,
            start_y,
            self.size * 9.0,
            self.size * 9.0,
            8.0,
            BLACK,
        );
        draw_line(
            start_x + 3.0 * self.size,
            start_y,
            start_x + 3.0 * self.size,
            start_y + 9.0 * self.size,
            4.0,
            BLACK,
        );
        draw_line(
            start_x + 6.0 * self.size,
            start_y,
            start_x + 6.0 * self.size,
            start_y + 9.0 * self.size,
            4.0,
            BLACK,
        );
        draw_line(
            start_x,
            start_y + 3.0 * self.size,
            start_x + 9.0 * self.size,
            start_y + 3.0 * self.size,
            4.0,
            BLACK,
        );
        draw_line(
            start_x,
            start_y + 6.0 * self.size,
            start_x + 9.0 * self.size,
            start_y + 6.0 * self.size,
            4.0,
            BLACK,
        );
    }
}
