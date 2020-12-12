extern crate piston_window;

use board::Board;
use piston_window::*;
use renderer::BoardRenderer;

mod board;
mod renderer;

const WIDTH: f64 = 640.0;
const HEIGHT: f64 = 480.0;
const BOARD_SIZE: i8 = 5;

fn main() {
    

    let mut window: PistonWindow = WindowSettings::new("Hexxagon", [WIDTH, HEIGHT])
        .exit_on_esc(true)
        .build()
        .unwrap();

    let board = Board::new(BOARD_SIZE);

    let renderer: BoardRenderer = BoardRenderer::new(&board, WIDTH, HEIGHT, BOARD_SIZE);

    let fields_ref = &board.fields;
    while let Some(event) = window.next() {
        window.draw_2d(&event, |context, graphics, _device| {
            clear([1.0; 4], graphics);
            renderer.render(context, graphics)
        });
    }
}