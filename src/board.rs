struct Player;

enum HexFieldState {
    DISABLED,
    EMPTY,
    OCCUPIED,
}

pub struct Coordinate {
    pub x: i8,
    pub y: i8,
    pub z: i8,
}

pub struct Board {
    pub fields: Vec<HexField>,
}

pub struct HexField {
    pub coordinate: Coordinate,
    state: HexFieldState,
    occupant: Option<Player>,
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
    pub fn new(size: i8) -> Board {
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
            .map(|coordinate| HexField {
                coordinate,
                state: HexFieldState::EMPTY,
                occupant: None,
            })
            .collect();
        Board { fields }
    }

    // pub fn possible_move_options(&self, from: &Coordinate, range: i8) -> Vec<Coordinate> {
    //     self.fields
    //         .into_iter()
    //         .filter(|h| match h.state {
    //             HexFieldState::EMPTY => true,
    //             _ => false,
    //         })
    //         .filter(|h| {
    //             let dist = from.distance(&h.coordinate);
    //             dist < range && range != 0
    //         })
    //         .map(|h| h.coordinate)
    //         .collect::<Vec<Coordinate>>()
    // }

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
        let board = Board::new(1);
        assert_eq!(board.field_count(), 1)
    }

    #[test]
    fn should_have_7_fields_when_board_is_size_2() {
        let board = Board::new(2);
        assert_eq!(board.field_count(), 7)
    }

    #[test]
    fn should_have_3_fields_when_board_is_size_19() {
        let board = Board::new(3);
        assert_eq!(board.field_count(), 19)
    }

    #[test]
    fn should_have_61_fields_when_board_is_size_5() {
        let board = Board::new(5);
        assert_eq!(board.field_count(), 61)
    }
}
