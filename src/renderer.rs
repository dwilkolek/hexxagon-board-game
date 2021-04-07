use piston_window::*;

use crate::board::{Board, HexField, HexFieldState};

const FIELD_COLOR: [f32; 4] = [0.8, 0.2, 0.5, 1.0];
const FIELD_DISABLED_COLOR: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
const FIELD_PLAYER_1_COLOR: [f32; 4] = [1.0, 0.0, 0.0, 1.0];
const FIELD_PLAYER_2_COLOR: [f32; 4] = [0.0, 0.0, 1.0, 1.0];
const BORDER_COLOR: [f32; 4] = [0.5, 0.1, 0.3, 1.0];
pub struct BoardRenderer<'a> {
    field_size: f64,
    field_width: f64,
    field_height: f64,
    board: &'a Board,
    window_center_x: f64,
    window_center_y: f64,
}

impl BoardRenderer<'_> {
    pub fn new(
        board: &Board,
        window_width: f64,
        window_height: f64,
        board_size: i8,
    ) -> BoardRenderer {
        let field_size =
            (window_height).min(window_width) / ((board_size as f64 + 1.0) * 2.0) * 0.65;
        BoardRenderer {
            field_size,
            field_width: field_size * 2_f64,
            field_height: 3_f64.sqrt() * field_size,
            board,
            window_center_x: window_width / 2.0,
            window_center_y: window_height / 2.0,
        }
    }

    pub fn render<G>(&self, context: Context, graphics: &mut G)
    where
        G: Graphics,
    {
        let fields = &self.board.fields;
        fields.into_iter().for_each(|field| {
            let vertexes = self.vertexes(field);
            let background = match field.state {
                HexFieldState::Disabled => FIELD_DISABLED_COLOR,
                HexFieldState::Player1 => FIELD_PLAYER_1_COLOR,
                HexFieldState::Player2 => FIELD_PLAYER_2_COLOR,
                _ => FIELD_COLOR,
            };
            polygon(background, &vertexes, context.transform, graphics);
        });

        fields.into_iter().for_each(|field| {
            let vertexes = self.vertexes(field);

            for i in 0..6 {
                line(
                    BORDER_COLOR,
                    2.0,
                    [
                        vertexes[i % vertexes.len()][0],
                        vertexes[i % vertexes.len()][1],
                        vertexes[(i + 1) % vertexes.len()][0],
                        vertexes[(i + 1) % vertexes.len()][1],
                    ],
                    context.transform,
                    graphics,
                );
            }
        });
    }

    fn vertexes(&self, field: &HexField) -> [[f64; 2]; 6] {
        let x = field.coordinate.x as f64;
        let y = field.coordinate.y as f64;
        let z = field.coordinate.z as f64;
        let x_offset = x * self.field_width * 3.0 / 4.0;
        let y_offset = ((y - z) / 2.0) * self.field_height;
        let center_x = self.window_center_x + x_offset;
        let center_y = self.window_center_y + y_offset;

        [
            self.get_point_of_hex(center_x, center_y, self.field_size, 0),
            self.get_point_of_hex(center_x, center_y, self.field_size, 1),
            self.get_point_of_hex(center_x, center_y, self.field_size, 2),
            self.get_point_of_hex(center_x, center_y, self.field_size, 3),
            self.get_point_of_hex(center_x, center_y, self.field_size, 4),
            self.get_point_of_hex(center_x, center_y, self.field_size, 5),
        ]
    }

    fn get_point_of_hex(&self, center_x: f64, center_y: f64, size: f64, i: i8) -> [f64; 2] {
        let angle_deg = 60_f64 * i as f64;
        let angle_rad = std::f64::consts::PI / 180.0 * angle_deg;
        [
            center_x + size * angle_rad.cos(),
            center_y + size * angle_rad.sin(),
        ]
    }
}

// function flat_hex_corner(center, size, i):
//     var angle_deg = 60 * i
//     var angle_rad = PI / 180 * angle_deg
//     return Point(center.x + size * cos(angle_rad),
//                  center.y + size * sin(angle_rad))
