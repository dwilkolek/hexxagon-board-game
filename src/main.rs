use std::fmt;
#[derive(Debug)]
struct Field {
    x: i8,
    y: i8,
    z: i8,
}


impl Field {
    pub fn new(x: i8, y: i8) -> Field {
        Field { x, y, z: 0 - x - y }
    }

    pub fn is_valid(&self) -> bool {
        &self.x + self.y + self.z == 0
    }

    pub fn distance(&self, other: &Field) -> i8 {
        ((self.x - other.x).abs() + (self.y - other.y).abs() + (self.z - other.z).abs()) / 2
    }
}

impl fmt::Display for Field {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "(x:{} y:{}, z:{} | valid: {})",
            self.x,
            self.y,
            self.z,
            self.is_valid()
        )
    }
}
#[derive(Debug)]
struct Board {
    fields: Vec<Field>,
}


impl Board {
    pub fn new(size: i8) -> Board {
        let mut fields = Vec::new();
        fields.push(Field::new(0, 0));
        for current_ring in 1..size {
            // -size, 0   ->  0, -size
            let mut start = Field::new(-current_ring, 0);
            let mut end = Field::new(0, -current_ring);
            fields.push(start);
            fields.append(fields_between(fields.last().unwrap(), &end).as_mut());

            //  0, -size   ->  +size, -size
            start = end;
            end = Field::new(current_ring, -current_ring);
            fields.push(start);
            fields.append(fields_between(fields.last().unwrap(), &end).as_mut());

            
            //  +size, -size   ->  +size, 0
            start = end;
            end = Field::new(current_ring, 0);
            fields.push(start);
            fields.append(fields_between(fields.last().unwrap(), &end).as_mut());

            // +size, 0 -> 0, +size
            start = end;
            end = Field::new(0, current_ring);
            fields.push(start);
            fields.append(fields_between(fields.last().unwrap(), &end).as_mut());

            // 0, +size -> -size, +size
            start = end;
            end = Field::new(-current_ring, current_ring);
            fields.push(start);
            fields.append(fields_between(fields.last().unwrap(), &end).as_mut());

            // -size, +size -> -size, 0
            start = end;
            end = Field::new(-current_ring, 0);
            fields.push(start);
            fields.append(fields_between(fields.last().unwrap(), &end).as_mut());

        }
        Board {
            fields
        }
    }

    pub fn field_count(self) -> usize {
        self.fields.len()
    }
    
}


fn fields_between(start: &Field, end: &Field) -> Vec<Field> {
    let mut step_x = 0;
    if start.x < end.x {
        step_x = 1
    }
    if start.x > end.x {
        step_x = -1
    }
    
    
    let mut step_y = 0;
    if start.y < end.y {
        step_y = 1
    }
    if start.y > end.y {
        step_y = -1
    }

    let mut new_x = start.x;
    let mut new_y = start.y;
    
    let mut fields_between = Vec::new();
    if start.distance(end) > 1 {        
        loop {
            new_x = new_x + step_x;
            new_y = new_y + step_y;

            let new_field = Field::new(new_x, new_y);
            if end.distance(&new_field) != 0 {
                fields_between.push(new_field)
            } else {
                break;
            }            
        }
    }
    fields_between
}

fn main() {
 
}


#[test]
fn should_create_empty_vec_when_field_are_neighbours() {
    let start = Field::new(-1, -1);
    let end = Field::new(-1, 0);
    let fillup = fields_between(&start, &end);

    assert_eq!(fillup.len(), 0)
}

#[test]
fn should_create_1_record_vec_when_field_are_1_field_appart() {
    let start = Field::new(-2, 0);
    let end = Field::new(0, -2);
    let fillup = fields_between(&start, &end);

    assert_eq!(fillup.len(), 1)
}

#[test]
fn field_from_constructor_should_be_valid() {
    assert!(Field::new(1, 2).is_valid())
}

#[test]
fn should_give_distance_from_center() {
    assert!(Field::new(0, 0)
        .distance(&Field::new(-1, -2))
        .eq(&(3 as i8)))
}

#[test]
fn should_give_distance_when_other_field_provided() {
    assert!(Field::new(-2, 0)
        .distance(&Field::new(-1, -2))
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