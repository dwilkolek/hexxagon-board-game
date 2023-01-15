use std::collections::HashMap;

use piston::GenericEvent;

#[derive(Debug, Copy, Clone)]
pub enum HexFieldState {
    Disabled,
    Empty,
    Player1,
    Player2,
}

#[derive(Hash, Eq, Clone)]
pub struct Coordinate {
    pub x: i8,
    pub y: i8,
    pub z: i8,
}

pub struct Board {
    pub fields: Vec<HexField>,
    pub selected_field: Option<HexField>,
}

pub struct HexField {
    pub coordinate: Coordinate,
    pub state: HexFieldState,
}

impl PartialEq for Coordinate {
    fn eq(&self, other: &Self) -> bool {
        self.distance(other) == 0
    }
}

impl Coordinate {
    pub fn new(x: i8, y: i8) -> Coordinate {
        Coordinate { x, y, z: 0 - x - y }
    }

    pub fn is_valid(&self) -> bool {
        &self.x + self.y + self.z == 0
    }

    pub fn distance(&self, other: &Coordinate) -> i8 {
        ((self.x - other.x).abs() + (self.y - other.y).abs() + (self.z - other.z).abs()) / 2
    }
}

impl Board {
    pub fn new(size: i8, map: HashMap<Coordinate, HexFieldState>) -> Board {
        let mut coordinates = Vec::new();
        coordinates.push(Coordinate::new(0, 0));
        for current_ring in 1..size {
            // -size, 0   ->  0, -size
            let mut start = Coordinate::new(-current_ring, 0);
            let mut end = Coordinate::new(0, -current_ring);
            coordinates.push(start);
            coordinates.append(Self::fields_between(coordinates.last().unwrap(), &end).as_mut());

            //  0, -size   ->  +size, -size
            start = end;
            end = Coordinate::new(current_ring, -current_ring);
            coordinates.push(start);
            coordinates.append(Self::fields_between(coordinates.last().unwrap(), &end).as_mut());

            //  +size, -size   ->  +size, 0
            start = end;
            end = Coordinate::new(current_ring, 0);
            coordinates.push(start);
            coordinates.append(Self::fields_between(coordinates.last().unwrap(), &end).as_mut());

            // +size, 0 -> 0, +size
            start = end;
            end = Coordinate::new(0, current_ring);
            coordinates.push(start);
            coordinates.append(Self::fields_between(coordinates.last().unwrap(), &end).as_mut());

            // 0, +size -> -size, +size
            start = end;
            end = Coordinate::new(-current_ring, current_ring);
            coordinates.push(start);
            coordinates.append(Self::fields_between(coordinates.last().unwrap(), &end).as_mut());

            // -size, +size -> -size, 0
            start = end;
            end = Coordinate::new(-current_ring, 0);
            coordinates.push(start);
            coordinates.append(Self::fields_between(coordinates.last().unwrap(), &end).as_mut());
        }
        let fields: Vec<HexField> = coordinates
            .into_iter()
            .map(|coordinate| {
                let state = match map.get(&coordinate) {
                    Some(state) => *state,
                    None => HexFieldState::Empty,
                };
                HexField { coordinate, state }
            })
            .collect();
        Board {
            fields,
            selected_field: None,
        }
    }

    /// Handles events.
    pub fn event<E: GenericEvent>(&mut self, e: &E) {
        use piston::input::{Button, Key, MouseButton};
        if let Some(cursor_pos) = e.mouse_cursor_args() {
            println!("mouse pos: {:?}", cursor_pos);
            if let Some(Button::Mouse(MouseButton::Left)) = e.press_args() {
                // Find coordinates relative to upper left corner.
                // let x = cursor_pos[0] - pos[0];
                // let y = cursor_pos[1] - pos[1];
                // Check that coordinates are inside board boundaries.
                // if x >= 0.0 && x <= size && y >= 0.0 && y <= size {
                //     // Compute the cell position.
                //     let cell_x = (x / size * 9.0) as usize;
                //     let cell_y = (y / size * 9.0) as usize;
                //     self.selected_cell = Some([cell_x, cell_y]);
                // }
            }
        }

        // if let Some(Button::Keyboard(key)) = e.press_args() {
        //     if let Some(ind) = self.selected_cell {
        //         // Set cell value.
        //         match key {
        //             Key::D1 => self.gameboard.set(ind, 1),
        //             Key::D2 => self.gameboard.set(ind, 2),
        //             Key::D3 => self.gameboard.set(ind, 3),
        //             Key::D4 => self.gameboard.set(ind, 4),
        //             Key::D5 => self.gameboard.set(ind, 5),
        //             Key::D6 => self.gameboard.set(ind, 6),
        //             Key::D7 => self.gameboard.set(ind, 7),
        //             Key::D8 => self.gameboard.set(ind, 8),
        //             Key::D9 => self.gameboard.set(ind, 9),
        //             Key::Backspace => self.gameboard.set(ind, 0),
        //             _ => {}
        //         }
        //     }
        // }
    }

    pub fn possible_move_options(self, from: Coordinate, range: i8) -> Vec<Coordinate> {
        self.fields
            .into_iter()
            .filter(|h| match h.state {
                HexFieldState::Empty => true,
                _ => false,
            })
            .filter(|h| {
                let dist = from.distance(&h.coordinate);
                dist < range && range != 0
            })
            .map(|h| h.coordinate)
            .collect::<Vec<Coordinate>>()
    }

    pub fn field_count(self) -> usize {
        self.fields.len()
    }

    fn step(from: i8, to: i8) -> i8 {
        if from < to {
            return 1;
        }
        if from > to {
            return -1;
        }
        0
    }

    fn fields_between(start: &Coordinate, end: &Coordinate) -> Vec<Coordinate> {
        let step_x = Self::step(start.x, end.x);
        let step_y = Self::step(start.y, end.y);

        let mut new_x = start.x;
        let mut new_y = start.y;

        let mut fields_between = Vec::new();
        if start.distance(end) > 1 {
            loop {
                new_x = new_x + step_x;
                new_y = new_y + step_y;

                let new_field = Coordinate::new(new_x, new_y);
                if end.distance(&new_field) != 0 {
                    fields_between.push(new_field)
                } else {
                    break;
                }
            }
        }
        fields_between
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn should_create_empty_vec_when_field_are_neighbours() {
        let start = Coordinate::new(-1, -1);
        let end = Coordinate::new(-1, 0);
        let fillup = Board::fields_between(&start, &end);

        assert_eq!(fillup.len(), 0)
    }

    #[test]
    fn should_create_1_record_vec_when_field_are_1_field_appart() {
        let start = Coordinate::new(-2, 0);
        let end = Coordinate::new(0, -2);
        let fillup = Board::fields_between(&start, &end);

        assert_eq!(fillup.len(), 1)
    }

    #[test]
    fn field_from_constructor_should_be_valid() {
        assert!(Coordinate::new(1, 2).is_valid())
    }

    #[test]
    fn should_give_distance_from_center() {
        assert!(Coordinate::new(0, 0)
            .distance(&Coordinate::new(-1, -2))
            .eq(&(3 as i8)))
    }

    #[test]
    fn should_give_distance_when_other_field_provided() {
        assert!(Coordinate::new(-2, 0)
            .distance(&Coordinate::new(-1, -2))
            .eq(&(2 as i8)))
    }

    #[test]
    fn should_have_1_fields_when_board_is_size_1() {
        let board = Board::new(1, HashMap::new());
        assert_eq!(board.field_count(), 1)
    }

    #[test]
    fn should_have_7_fields_when_board_is_size_2() {
        let board = Board::new(2, HashMap::new());
        assert_eq!(board.field_count(), 7)
    }

    #[test]
    fn should_have_3_fields_when_board_is_size_19() {
        let board = Board::new(3, HashMap::new());
        assert_eq!(board.field_count(), 19)
    }

    #[test]
    fn should_have_61_fields_when_board_is_size_5() {
        let board = Board::new(5, HashMap::new());
        assert_eq!(board.field_count(), 61)
    }
}
