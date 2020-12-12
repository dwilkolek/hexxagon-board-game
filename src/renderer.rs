use piston_window::*;

use crate::board::Board;

pub struct BoardRenderer<'a> {
    field_size: f64,
    field_width: f64,
    field_height: f64,
    board: &'a Board
}


impl BoardRenderer<'_> {

    pub fn new(board: &Board, window_width: f64, window_height: f64, board_size: i8) -> BoardRenderer {
        let field_size = (window_height / board_size as f64).min(window_width / board_size as f64);
        BoardRenderer {
            field_size,
            field_width: field_size * 2_f64,
            field_height: 3_f64.sqrt() * field_size,
            board
        }
    }

    pub fn render<G>(&self, context: Context, graphics: &mut G) where G: Graphics {
        
        
        polygon(
            [1.0, 0.6, 0.4, 1.0],
            &[
                [420.0, 20.0],
                [480.0, 20.0],
                [480.0, 80.0]
            ],
            context.transform,
            graphics
        );
    }
    
}