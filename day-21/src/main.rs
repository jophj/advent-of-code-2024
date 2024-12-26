use core::fmt;
use std::{collections::HashMap, fmt::Display, io, path, vec};

struct Position(i8, i8);

#[derive(Debug, PartialEq, Eq, Clone, Hash, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
    Forward,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
struct Strokes(Vec<Direction>);

impl Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Direction::Up => write!(f, "^"),
            Direction::Down => write!(f, "v"),
            Direction::Left => write!(f, "<"),
            Direction::Right => write!(f, ">"),
            Direction::Forward => write!(f, "A"),
        }
    }
}

impl Display for Strokes {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut s = String::new();
        for d in self.0.iter() {
            s.push_str(&format!("{}", d));
        }

        write!(f, "{}", s)
    }
}

fn map_code(input: &str) -> Vec<Direction> {
    input
        .chars()
        .map(|c| match c {
            '^' => Direction::Up,
            'v' => Direction::Down,
            '<' => Direction::Left,
            '>' => Direction::Right,
            'A' => Direction::Forward,
            _ => panic!("Invalid character"),
        })
        .collect()
}

// TODO singleton
// up has priority over left
// right has priority over down
fn generate_map() -> HashMap<(Direction, Direction), Vec<Direction>> {
    let map: HashMap<(Direction, Direction), Vec<Direction>> = HashMap::from([
        ((Direction::Right, Direction::Right), vec![]),
        ((Direction::Right, Direction::Down), vec![Direction::Left]),
        ((Direction::Right, Direction::Forward), vec![Direction::Up]),
        (
            (Direction::Forward, Direction::Left),
            vec![Direction::Down, Direction::Left, Direction::Left],
        ),
        ((Direction::Forward, Direction::Up), vec![Direction::Left]),
        ((Direction::Forward, Direction::Forward), vec![]),
        (
            (Direction::Forward, Direction::Down),
            vec![Direction::Down, Direction::Left],
        ),
        (
            (Direction::Forward, Direction::Right),
            vec![Direction::Down],
        ),
        (
            (Direction::Up, Direction::Left),
            vec![Direction::Down, Direction::Left],
        ),
        (
            (Direction::Up, Direction::Right),
            vec![Direction::Right, Direction::Down],
        ),
        ((Direction::Up, Direction::Forward), vec![Direction::Right]),
        ((Direction::Up, Direction::Up), vec![]),
        (
            (Direction::Left, Direction::Up),
            vec![Direction::Right, Direction::Up],
        ),
        ((Direction::Left, Direction::Down), vec![Direction::Right]),
        ((Direction::Left, Direction::Left), vec![]),
        (
            (Direction::Left, Direction::Forward),
            vec![Direction::Right, Direction::Right, Direction::Up],
        ),
        ((Direction::Down, Direction::Right), vec![Direction::Right]),
        ((Direction::Down, Direction::Left), vec![Direction::Left]),
        (
            (Direction::Down, Direction::Forward),
            vec![Direction::Right, Direction::Up],
        ),
        ((Direction::Down, Direction::Down), vec![]),
        (
            (Direction::Right, Direction::Up),
            vec![Direction::Up, Direction::Left],
        ),
    ]);

    map
}

fn map_strokes(start: Direction, end: Direction) -> Vec<Direction> {
    // println!("Mapping {} to {}", start, end);
    let map = generate_map();
    let mut mapped = map.get(&(start, end)).unwrap().clone();
    mapped.push(Direction::Forward);

    mapped.to_vec()
}

fn cost(strokes: &Strokes, depth: usize) -> Strokes {
    if depth == 0 {
        return strokes.clone();
    }

    let mut mapped = vec![];
    mapped.extend(map_strokes(Direction::Forward, strokes.0[0]));
    // println!("{}", Strokes(mapped.clone()));
    for i in 0..strokes.0.len() - 1 {
        // get next without advancing the iterator
        let current = &strokes.0[i];
        let next = &strokes.0[i + 1];
        // TODO slices
        let partial = map_strokes(current.clone(), next.clone());
        mapped.extend(partial);
    }

    cost(&Strokes(mapped), depth - 1)
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

    if end_char == 'A' && start_char == '0' {
        path.push(Direction::Right);
        return path;
    }

    if end_char == '0' && start_char == 'A' {
        path.push(Direction::Left);
        return path;
    }

    if start.1 < 3 && end.1 < 3 {
        let mut delta_x = end.0 - start.0;
        let mut delta_y = end.1 - start.1;

        if delta_x > 0 && delta_y > 0 {
            while delta_x != 0 {
                path.push(Direction::Right);
                delta_x -= 1;
            }

            while delta_y != 0 {
                path.push(Direction::Down);
                delta_y -= 1;
            }

            return path;
        }

        if delta_y < 0 && delta_x < 0 {
            while delta_y != 0 {
                path.push(Direction::Up);
                delta_y += 1;
            }
            while delta_x != 0 {
                path.push(Direction::Left);
                delta_x += 1;
            }

            return path;
        }

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

        return path;
    }

    if start.1 < 3 && end.1 == 3 {
        if end_char == '0' {
            let partial = find_numpad_path(numpad, start_char, '2');
            path.extend(partial);
            path.push(Direction::Down);
        } else if end_char == 'A' {
            let partial = find_numpad_path(numpad, start_char, '3');
            path.extend(partial);
            path.push(Direction::Down);
        }

        return path;
    }

    if start.1 == 3 && end.1 < 3 {
        if start_char == '0' {
            path.push(Direction::Up);
            let partial = find_numpad_path(numpad, '2', end_char);
            path.extend(partial);
        } else if start_char == 'A' {
            path.push(Direction::Up);
            let partial = find_numpad_path(numpad, '3', end_char);
            path.extend(partial);
        }

        return path;
    }

    return path;
}

fn calculate_keypad_actions(input: Vec<Direction>, start: Direction) -> Vec<Direction> {
    let mut directions = vec![];
    let mut current_direction = start;

    for d in input {
        let path = find_keypad_path(current_direction, d.clone());
        directions.extend(path);
        directions.push(Direction::Forward);
        current_direction = d;
    }

    directions
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

fn find_keypad_path(start: Direction, end: Direction) -> Vec<Direction> {
    let mut path = vec![];

    if start == end {
        return path;
    }

    match start {
        Direction::Left => match end {
            Direction::Down => {
                path.push(Direction::Right);
            }
            Direction::Forward => {
                path.push(Direction::Right);
                path.push(Direction::Right);
                path.push(Direction::Up);
            }
            _ => {
                path.push(Direction::Right);
                let partial = find_keypad_path(Direction::Down, end);
                path.extend(partial);
            }
        },
        Direction::Down => match end {
            Direction::Forward => {
                path.push(Direction::Right);
                path.push(Direction::Up);
            }
            _ => {
                path.push(end);
            }
        },
        Direction::Right => match end {
            Direction::Forward => {
                path.push(Direction::Up);
            }
            Direction::Down => {
                path.push(Direction::Left);
            }
            _ => {
                path.push(Direction::Left);
                path.push(end);
            }
        },
        Direction::Up => match end {
            Direction::Forward => {
                path.push(Direction::Right);
            }
            Direction::Down => {
                path.push(Direction::Down);
            }
            _ => {
                path.push(Direction::Down);
                path.push(end);
            }
        },
        Direction::Forward => match end {
            Direction::Right => {
                path.push(Direction::Down);
            }
            Direction::Up => {
                path.push(Direction::Left);
            }
            // added to comply with the example
            Direction::Left => {
                path.push(Direction::Down);
                path.push(Direction::Left);
                path.push(Direction::Left);
            }
            _ => {
                path.push(Direction::Left);
                let partial = find_keypad_path(Direction::Up, end);
                path.extend(partial);
            }
        },
    }

    path
}

fn score(strokes: &Strokes, code: &str) -> usize {
    let len = strokes.0.len();
    let input_str = code.chars().filter(|c| c.is_numeric()).collect::<String>();
    let numerical = input_str.parse::<usize>().unwrap();
    let score = len * numerical;

    score
}

fn main() -> io::Result<()> {
    let path = "example.txt";
    let text = std::fs::read_to_string(path)?;
    let inputs = text
        .lines()
        .map(|l| l.chars().collect())
        .collect::<Vec<Vec<char>>>();

    let numpad = generate_numpad();

    let mut result = 0;
    for input in inputs {
        let numpad_actions = calculate_numpad_actions(&numpad, input.clone(), 'A');
        let keypad_1 = calculate_keypad_actions(numpad_actions, Direction::Forward);
        let keypad_2 = calculate_keypad_actions(keypad_1, Direction::Forward);

        // reduce input chars to a single string filtering only numeric chars
        let len = keypad_2.len();
        let input_str = input.iter().filter(|c| c.is_numeric()).collect::<String>();
        let numerical = input_str.parse::<usize>().unwrap();
        let score = len * numerical;
        println!("The score is: {} * {} = {}", len, numerical, score);
        result += score;
    }

    let test = "379A";
    let numpad_actions = calculate_numpad_actions(&numpad, test.chars().collect(), 'A');
    let keypad_1 = calculate_keypad_actions(numpad_actions.clone(), Direction::Forward);
    let keypad_2 = calculate_keypad_actions(keypad_1.clone(), Direction::Forward);

    let numpad_mapped = numpad_actions
        .iter()
        .map(|d| match d {
            Direction::Up => "^",
            Direction::Down => "v",
            Direction::Left => "<",
            Direction::Right => ">",
            Direction::Forward => "A",
        })
        .collect::<String>();
    println!("{:?}", numpad_mapped);

    let mapped_1 = keypad_1
        .iter()
        .map(|d| match d {
            Direction::Up => "^",
            Direction::Down => "v",
            Direction::Left => "<",
            Direction::Right => ">",
            Direction::Forward => "A",
        })
        .collect::<String>();

    println!("{:?}", mapped_1);

    // 379A
    // ^A<<^^A>>AvvvA x prima
    // ^A^^<<A>>AvvvA y prima
    // <A>Av<<AA>^AA>AvAA^A<vAAA>^A   x prima
    // <A>A<AAv<AA>>^AvAA^A<vAAA>^A   y prima
    // v<<A >>^A vA ^A v<<A >>^AA<vA<A>>^AAvAA<^A>A<vA>^AA<A>Av<<A>A>^AAAvA<^A>A
    // <v<A >>^A vA ^A <vA <AA>>^AAvA<^A>AAvA^A<vA>^AA<A>A<v<A>A>^AAAvA<^A>A
    // <v<A>>^AvA^A<vA<AA>>^AAvA<^A>AAvA^A<vA>^AA<A>A<v<A>A>^AAAvA<^A>A
    // v<<A>>^AvA^Av<<A>>^AA<vA<A>>^AAvAA<^A>A<vA>^AA<A>Av<<A>A>^AAAvA<^A>A

    // 456A
    // ^<<^A>A>AvvA
    // <Av<AA>^A>AvA^AvA^A<vAA>^A
    // v<<A >>^A <vA<A>>^AAvA<^A>AvA^A<vA>^A<A>A<vA>^A<A>Av<<A>A>^AAvA<^A>A
    // <v<A >>^A A<vA<A>>^AAvAA<^A>A<vA>^A<A>A<vA>^A<A>A<v<A>A>^AAvA<^A>A
    // 980A
    // A<vA<AA>>^AvAA<^A>Av<<A>A>^AAAvA<^A>A<vA>^A<A>A
    // <v<A>>^AAAvA^A<vA<AA>>^AvAA<^A>A<v<A>A>^AAAvA<^A>A<vA>^A<A>A

    // 029A
    // <vA<AA>>^AvAA<^A>Av<<A>>^AvA^A<vA>^Av<<A>^A>AAvA^Av<<A>A>^AAAvA<^A>A
    // <vA<AA>>^AvAA<^A>A<v<A>>^AvA^A<vA>^A<v<A>^A>AAvA^A<v<A>A>^AAAvA<^A>A

    let mapped = keypad_2
        .iter()
        .map(|d| match d {
            Direction::Up => "^",
            Direction::Down => "v",
            Direction::Left => "<",
            Direction::Right => ">",
            Direction::Forward => "A",
        })
        .collect::<String>();

    println!("{:?}", mapped);

    // let result = keypad_2.len() * 29;
    println!("The result is: {}", result);

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::{
        calculate_keypad_actions, calculate_numpad_actions, cost, find_keypad_path,
        find_numpad_path, generate_numpad, map_code, map_strokes, score, Direction, Strokes,
    };

    #[test]
    fn ULDR_should_map_to_LADRARARA() {
        let keys = vec![
            Direction::Up,
            Direction::Left,
            Direction::Down,
            Direction::Right,
        ];

        let mapped = cost(&Strokes(keys), 1);
        let expected = Strokes(vec![
            Direction::Left,
            Direction::Forward,
            Direction::Down,
            Direction::Left,
            Direction::Forward,
            Direction::Right,
            Direction::Forward,
            Direction::Right,
            Direction::Forward,
        ]);
        assert_eq!(mapped, expected);
    }

    #[test]
    fn UL_should_map_to_LADLA() {
        let keys = vec![Direction::Up, Direction::Left];

        let mapped = cost(&Strokes(keys), 1);
        let expected = Strokes(vec![
            Direction::Left,
            Direction::Forward,
            Direction::Down,
            Direction::Left,
            Direction::Forward,
        ]);
        assert_eq!(mapped, expected);
    }

    #[test]
    fn example_l1_directions() {
        let keys = map_code("<A^A>^^AvvvA");

        let mapped = cost(&Strokes(keys.clone()), 1);
        let expected = Strokes(map_code("v<<A>>^A<A>AvA<^AA>A<vAAA>^A"));

        assert_eq!(mapped, expected);
    }

    #[test]
    fn example_l2_directions() {
        let keys = map_code("v<<A>>^A<A>AvA<^AA>A<vAAA>^A");

        let mapped = cost(&Strokes(keys.clone()), 1);
        let expected = Strokes(vec![Direction::Down, Direction::Left, Direction::Left]);

        println!("{}", Strokes(keys));
        println!("{}", mapped);

        //v<A<AA>>^AvAA^<A>Av<<A>>^AvA^Av<A>^Av<<A>^A>AAvA^Av<<A>A>^AAAvA^<A>A
        //<vA<AA>>^AvAA<^A>A<v<A>>^AvA^A<vA>^A<v<A>^A>AAvA^A<v<A>A>^AAAvA<^A>A

        assert_eq!(mapped, expected);
    }

    #[test]
    fn score_with_first_example_68_x_29() {
        let code = "029A";
        let keys = map_code("<A^A>^^AvvvA");

        let strokes = cost(&Strokes(keys.clone()), 2);
        let result = score(&strokes, code);
        assert_eq!(result, 68 * 29);
    }

    #[test]
    fn score_with_second_example_60_x_980() {
        let code = "980A";
        let keys = map_code("^^^A<AvvvA>A");

        let strokes = cost(&Strokes(keys.clone()), 2);
        let result = score(&strokes, code);

        //v<<A>>^AAAvA^Av<A<AA>>^AvAA^<A>Av<A<A>>^AAAvA^<A>Av<A>^A<A>A
        //<v<A>>^AAAvA^A<vA<AA>>^AvAA<^A>A<v<A>A>^AAAvA<^A>A<vA>^A<A>A

        println!("{}", strokes);
        assert_eq!(result, 60 * 980);
    }

    #[test]
    fn score_with_third_example_68_x_179() {
        let code = "179A";
        let keys = map_code("^<<A^^A>>AvvvA");

        let strokes = cost(&Strokes(keys.clone()), 2);
        let result = score(&strokes, code);

        //v<<A>>^Av<A<A>>^AvAA^<A>Av<<A>>^AAvA^Av<A>^AA<A>Av<A<A>>^AAvA^<A>A
        //<v<A>>^A<vA<A>>^AAvAA<^A>A<v<A>>^AAvA^A<vA>^AA<A>A<v<A>A>^AAAvA<^A>A

        println!("{}", strokes);
        assert_eq!(result, 68 * 179);
    }

    #[test]
    fn score_with_fourth_example_64_x_456() {
        let code = "456A";
        let keys = map_code("^^<<A>A>AvvA");

        let strokes = cost(&Strokes(keys.clone()), 2);
        let result = score(&strokes, code);

        println!("{}", strokes);
        assert_eq!(result, 64 * 456);
    }

    #[test]
    fn score_with_fifth_example_64_x_379() {
        let code = "379A";
        let keys = map_code("^A<<^^A>>AvvvA");

        let strokes = cost(&Strokes(keys.clone()), 2);
        let result = score(&strokes, code);

        //v<<A>>^AvA^Av<<A>>^AAv<A<A>>^AAvAA^<A>Av<A>^AA<A>Av<A<A>>^AAAvA^<A>A
        //<v<A>>^AvA^A<vA<AA>>^AAvA<^A>AAvA^A<vA>^AA<A>A<v<A>A>^AAAvA<^A>A

        println!("{}", strokes);
        assert_eq!(result, 64 * 379);
    }

    #[test]
    fn score_example_126384() {
        let codes = vec![
            ("029A", "<A^A>^^AvvvA"),
            ("980A", "^^^A<AvvvA>A"),
            ("179A", "^<<A^^A>>AvvvA"),
            ("456A", "^^<<A>A>AvvA"),
            ("379A", "^A<<^^A>>AvvvA"),
        ];

        let mut final_score = 0;
        codes.iter().for_each(|(code, keys)| {
            let strokes = cost(&Strokes(map_code(keys)), 2);
            let result = score(&strokes, code);
            final_score += result;
        });

        assert_eq!(final_score, 126384);
    }

    #[test]
    fn input_part_1() {
        // 480A
        // 965A
        // 140A
        // 341A
        // 285A
        let codes = vec![
            ("480A", "^^<<A^>AvvvA>A"),
            ("965A", "^^^AvA<Avv>A"),
            ("140A", "^<<A^A>vvA>A"),
            ("341A", "^A<<^AvA>>vA"),
            ("285A", "<^A^^AvAvv>A"),
        ];

        let mut final_score = 0;
        codes.iter().for_each(|(code, keys)| {
            let strokes = cost(&Strokes(map_code(keys)), 2);
            let result = score(&strokes, code);
            final_score += result;
        });

        println!("{}", final_score);
        assert_eq!(final_score, 152942);
    }

    #[test]
    fn input_part_2() {
        // 480A
        // 965A
        // 140A
        // 341A
        // 285A
        let codes = vec![
            ("480A", "^^<<A^>AvvvA>A"),
            ("965A", "^^^AvA<Avv>A"),
            ("140A", "^<<A^A>vvA>A"),
            ("341A", "^A<<^AvA>>vA"),
            ("285A", "<^A^^AvAvv>A"),
        ];

        let mut final_score = 0;
        codes.iter().for_each(|(code, keys)| {
            let strokes = cost(&Strokes(map_code(keys)), 25);
            let result = score(&strokes, code);
            final_score += result;
        });

        println!("{}", final_score);
    }
}
