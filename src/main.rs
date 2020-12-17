extern crate piston_window;

use crate::board::{Coordinate, HexFieldState};
use board::Board;
use piston_window::*;
use renderer::BoardRenderer;
use std::collections::HashMap;

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

    let mut starting_params = HashMap::new();
    starting_params.insert(Coordinate::new(0, -1), HexFieldState::DISABLED);
    starting_params.insert(Coordinate::new(-1, 1), HexFieldState::DISABLED);
    starting_params.insert(Coordinate::new(1, 0), HexFieldState::DISABLED);

    starting_params.insert(Coordinate::new(0, -BOARD_SIZE + 1), HexFieldState::PLAYER1);
    starting_params.insert(
        Coordinate::new(-BOARD_SIZE + 1, BOARD_SIZE - 1),
        HexFieldState::PLAYER1,
    );
    starting_params.insert(Coordinate::new(BOARD_SIZE - 1, 0), HexFieldState::PLAYER1);

    starting_params.insert(Coordinate::new(-BOARD_SIZE + 1, 0), HexFieldState::PLAYER2);
    starting_params.insert(
        Coordinate::new(BOARD_SIZE - 1, -BOARD_SIZE + 1),
        HexFieldState::PLAYER2,
    );
    starting_params.insert(Coordinate::new(0, BOARD_SIZE - 1), HexFieldState::PLAYER2);

    let board = Board::new(BOARD_SIZE, starting_params);

    let renderer: BoardRenderer = BoardRenderer::new(&board, WIDTH, HEIGHT, BOARD_SIZE);
    while let Some(event) = window.next() {
        window.draw_2d(&event, |context, graphics, _device| {
            clear([0.1; 4], graphics);
            renderer.render(context, graphics)
        });
    }
}
