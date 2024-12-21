use std::{collections::HashMap, io};

struct Position(i8, i8);

#[derive(Debug, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn generate_numpad() -> HashMap<char, Position> {
    HashMap::from([
        ('7', Position(0, 0)),
        ('8', Position(1, 0)),
        ('9', Position(2, 0)),
        ('4', Position(0, 1)),
        ('5', Position(1, 1)),
        ('6', Position(2, 1)),
        ('1', Position(0, 2)),
        ('2', Position(1, 2)),
        ('3', Position(2, 2)),
        ('0', Position(1, 2)),
        ('A', Position(2, 3)),
    ])
}

fn find_numpad_path(numpad: &HashMap<char, Position>, start: char, end: char) -> Vec<Direction> {
    let start = numpad.get(&start).unwrap();
    let end = numpad.get(&end).unwrap();
    let mut path = vec![];

    if (start.0 < 3 && start.1 < 3) && (end.0 < 3 && end.1 < 3) {
        let mut delta_x = end.0 - start.0;
        let mut delta_y = end.1 - start.1;

        while delta_x != 0 {
            if delta_x > 0 {
                path.push(Direction::Right);
                delta_x -= 1;
            } else {
                path.push(Direction::Left);
                delta_x += 1;
            }
        }

        while delta_y != 0 {
            if delta_y > 0 {
                path.push(Direction::Down);
                delta_y -= 1;
            } else {
                path.push(Direction::Up);
                delta_y += 1;
            }
        }
    }

    path
}

fn main() -> io::Result<()> {
    let path = "input.txt";
    let numpad = generate_numpad();
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::{find_numpad_path, generate_numpad, Direction};

    #[test]
    fn should_pass() {
        assert_eq!(true, true);
    }

    #[test]
    fn with_7_3_should_find_path() {
        let numpad = generate_numpad();

        let path = find_numpad_path(&numpad, '7', '3');
        assert_eq!(
            path,
            vec![
                Direction::Right,
                Direction::Right,
                Direction::Down,
                Direction::Down
            ]
        );
    }
    #[test]
    fn with_2_9_should_find_path() {
        let numpad = generate_numpad();

        let path = find_numpad_path(&numpad, '2', '9');
        assert_eq!(path, vec![Direction::Right, Direction::Up, Direction::Up,]);
    }
}
