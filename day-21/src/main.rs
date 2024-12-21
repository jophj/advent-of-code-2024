use std::{collections::HashMap, io};

struct Position(i8, i8);

#[derive(Debug, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
    Forward,
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
        ('0', Position(1, 3)),
        ('A', Position(2, 3)),
    ])
}

fn find_numpad_path(
    numpad: &HashMap<char, Position>,
    start_char: char,
    end_char: char,
) -> Vec<Direction> {
    let start = numpad.get(&start_char).unwrap();
    let end = numpad.get(&end_char).unwrap();
    let mut path = vec![];

    if start.1 < 3 && end.1 < 3 {
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

    if (start.0 < 3 && start.1 < 3) && end.1 == 3 {
        if end_char == '0' {
            let partial = find_numpad_path(numpad, start_char, '2');
            path.extend(partial);
            path.push(Direction::Down);
        } else if end_char == 'A' {
            let partial = find_numpad_path(numpad, start_char, '3');
            path.extend(partial);
            path.push(Direction::Down);
        }
    }

    if start.1 == 3 && end.1 < 3 {
        if start_char == '0' {
            path.push(Direction::Up);
            let partial = find_numpad_path(numpad, '2', end_char);
            path.extend(partial);
        } else if end_char == 'A' {
            path.push(Direction::Up);
            let partial = find_numpad_path(numpad, '3', end_char);
            path.extend(partial);
        }
    }

    if end_char == 'A' && start_char == '0' {
        path.push(Direction::Right);
    }

    if end_char == '0' && start_char == 'A' {
        path.push(Direction::Left);
    }

    path
}

fn calculate_numpad_actions(
    numpad: &HashMap<char, Position>,
    input: Vec<char>,
    start_char: char,
) -> Vec<Direction> {
    let mut directions = vec![];
    let mut current_char = start_char;

    for d in input {
        let path = find_numpad_path(numpad, current_char, d);
        directions.extend(path);
        directions.push(Direction::Forward);
        current_char = d;
    }

    directions
}

fn main() -> io::Result<()> {
    let path = "input.txt";
    let numpad = generate_numpad();
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::{calculate_numpad_actions, find_numpad_path, generate_numpad, Direction};

    #[test]
    fn should_pass() {
        assert_eq!(true, true);
    }

    #[test]
    fn with_029_should_find_path() {
        let numpad = generate_numpad();

        let path = calculate_numpad_actions(&numpad, vec!['0', '2', '9', 'A'], 'A');
        let mapped = path
            .iter()
            .map(|d| match d {
                Direction::Up => '^',
                Direction::Down => 'v',
                Direction::Left => '<',
                Direction::Right => '>',
                Direction::Forward => 'A',
            })
            .collect::<String>();

        assert_eq!(mapped, "<A^A>^^AvvvA");
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

    #[test]
    fn with_1_0_should_find_path() {
        let numpad = generate_numpad();

        let path = find_numpad_path(&numpad, '1', '0');
        assert_eq!(path, vec![Direction::Right, Direction::Down]);
    }

    #[test]
    fn with_9_0_should_find_path() {
        let numpad = generate_numpad();

        let path = find_numpad_path(&numpad, '9', '0');
        assert_eq!(
            path,
            vec![
                Direction::Left,
                Direction::Down,
                Direction::Down,
                Direction::Down
            ]
        );
    }

    #[test]
    fn with_3_0_should_find_path() {
        let numpad = generate_numpad();

        let path = find_numpad_path(&numpad, '3', '0');
        assert_eq!(path, vec![Direction::Left, Direction::Down]);
    }

    #[test]
    fn with_1_A_should_find_path() {
        let numpad = generate_numpad();

        let path = find_numpad_path(&numpad, '1', 'A');
        assert_eq!(
            path,
            vec![Direction::Right, Direction::Right, Direction::Down]
        );
    }

    #[test]
    fn with_0_1_should_find_path() {
        let numpad = generate_numpad();

        let path = find_numpad_path(&numpad, '0', '1');
        assert_eq!(path, vec![Direction::Up, Direction::Left]);
    }

    #[test]
    fn with_A_2_should_find_path() {
        let numpad = generate_numpad();

        let path = find_numpad_path(&numpad, '0', '1');
        assert_eq!(path, vec![Direction::Up, Direction::Left]);
    }

    #[test]
    fn with_A_0_should_find_path() {
        let numpad = generate_numpad();

        let path = find_numpad_path(&numpad, 'A', '0');
        assert_eq!(path, vec![Direction::Left]);
    }

    #[test]
    fn with_0_A_should_find_path() {
        let numpad = generate_numpad();

        let path = find_numpad_path(&numpad, '0', 'A');
        assert_eq!(path, vec![Direction::Right]);
    }
}
