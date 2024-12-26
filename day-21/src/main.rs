use core::fmt;
use enum_map::{enum_map, Enum, EnumMap};
use lazy_static::lazy_static;
use std::{fmt::Display, vec};

fn main() {
    println!("AoC 21");
}

#[derive(Enum, Clone, Copy, PartialEq, Eq, Hash, Debug)]
enum Direction {
    Right,
    Down,
    Forward,
    Up,
    Left,
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
lazy_static! {
    static ref MAP: EnumMap<Direction, EnumMap<Direction, Vec<Direction>>> = enum_map! {
        Direction::Right => enum_map! {
            Direction::Right => vec![Direction::Forward],
            Direction::Down => vec![Direction::Left, Direction::Forward],
            Direction::Forward => vec![Direction::Up, Direction::Forward],
            Direction::Up => vec![Direction::Up, Direction::Left, Direction::Forward],
            Direction::Left => vec![Direction::Forward],
        },
        Direction::Down => enum_map! {
            Direction::Right => vec![Direction::Right, Direction::Forward],
            Direction::Down => vec![Direction::Forward],
            Direction::Forward => vec![Direction::Right, Direction::Up, Direction::Forward],
            Direction::Up => vec![Direction::Right, Direction::Up, Direction::Forward],
            Direction::Left => vec![Direction::Left, Direction::Forward],
        },
        Direction::Forward => enum_map! {
            Direction::Right => vec![Direction::Down, Direction::Forward],
            Direction::Down => vec![Direction::Down, Direction::Left, Direction::Forward],
            Direction::Forward => vec![Direction::Forward],
            Direction::Up => vec![Direction::Left, Direction::Forward],
            Direction::Left => vec![Direction::Down, Direction::Left, Direction::Left, Direction::Forward],
        },
        Direction::Up => enum_map! {
            Direction::Right => vec![Direction::Right, Direction::Down, Direction::Forward],
            Direction::Down => vec![Direction::Down, Direction::Left, Direction::Forward],
            Direction::Forward => vec![Direction::Right, Direction::Forward],
            Direction::Up => vec![Direction::Forward],
            Direction::Left => vec![Direction::Down, Direction::Left, Direction::Forward],
        },
        Direction::Left => enum_map! {
            Direction::Right => vec![Direction::Right, Direction::Up, Direction::Forward],
            Direction::Down => vec![Direction::Right, Direction::Forward],
            Direction::Forward => vec![Direction::Right, Direction::Right, Direction::Up, Direction::Forward],
            Direction::Up => vec![Direction::Right, Direction::Up, Direction::Forward],
            Direction::Left => vec![Direction::Forward],
        },
    };
}

fn map_strokes(start: &Direction, end: &Direction) -> &'static [Direction] {
    &MAP[*start][*end]
}

fn cost(mut strokes: Strokes, depth: usize) -> Strokes {
    // Pre-allocate two buffers with sufficient capacity
    let mut buffer1 = vec![Direction::Forward; 1 * 1024 * 1024 * 1024];
    let mut buffer2 = vec![Direction::Forward; 1 * 1024 * 1024 * 1024];

    // Pointers to the active and inactive buffers
    let mut active_buffer = &mut buffer1;
    let mut inactive_buffer = &mut buffer2;

    for _ in 0..depth {
        let mut pos = 0;

        // Map the first element with Direction::Forward
        let initial_mapping = map_strokes(&Direction::Forward, &strokes.0[0]);
        active_buffer[pos..pos + initial_mapping.len()].copy_from_slice(initial_mapping);
        pos += initial_mapping.len();

        // Process the pairwise mappings
        for window in strokes.0.windows(2) {
            if let [current, next] = window {
                let mapped = map_strokes(current, next);
                active_buffer[pos..pos + mapped.len()].copy_from_slice(mapped);
                pos += mapped.len();
            }
        }

        // Update the inactive buffer to point to the current active buffer's content
        inactive_buffer[..pos].copy_from_slice(&active_buffer[..pos]);

        // Update strokes to point to the new buffer
        strokes.0.clear();
        strokes.0.extend_from_slice(&active_buffer[..pos]);

        // Swap the active and inactive buffers
        std::mem::swap(&mut active_buffer, &mut inactive_buffer);
    }

    strokes
}

fn score(strokes: &Strokes, code: &str) -> usize {
    let len = strokes.0.len();
    let input_str = code.chars().filter(|c| c.is_numeric()).collect::<String>();
    let numerical = input_str.parse::<usize>().unwrap();
    let score = len * numerical;

    score
}

#[cfg(test)]
mod tests {
    use crate::{cost, map_code, score, Direction, Strokes};

    #[test]
    fn uldr_should_map_to_ladrarara() {
        let keys = vec![
            Direction::Up,
            Direction::Left,
            Direction::Down,
            Direction::Right,
        ];

        let mapped = cost(Strokes(keys), 1);
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
    fn ul_should_map_to_ladla() {
        let keys = vec![Direction::Up, Direction::Left];

        let mapped = cost(Strokes(keys), 1);
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

        let mapped = cost(Strokes(keys.clone()), 1);
        let expected = Strokes(map_code("v<<A>>^A<A>AvA<^AA>A<vAAA>^A"));

        assert_eq!(mapped, expected);
    }

    #[test]
    fn example_l2_directions() {
        let keys = map_code("v<<A>>^A<A>AvA<^AA>A<vAAA>^A");

        let mapped = cost(Strokes(keys.clone()), 1);
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

        let strokes = cost(Strokes(keys.clone()), 2);
        let result = score(&strokes, code);
        assert_eq!(result, 68 * 29);
    }

    #[test]
    fn score_with_second_example_60_x_980() {
        let code = "980A";
        let keys = map_code("^^^A<AvvvA>A");

        let strokes = cost(Strokes(keys.clone()), 2);
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

        let strokes = cost(Strokes(keys.clone()), 2);
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

        let strokes = cost(Strokes(keys.clone()), 2);
        let result = score(&strokes, code);

        println!("{}", strokes);
        assert_eq!(result, 64 * 456);
    }

    #[test]
    fn score_with_fifth_example_64_x_379() {
        let code = "379A";
        let keys = map_code("^A<<^^A>>AvvvA");

        let strokes = cost(Strokes(keys.clone()), 2);
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
            let strokes = cost(Strokes(map_code(keys)), 2);
            let result = score(&strokes, code);
            final_score += result;
        });

        assert_eq!(final_score, 126384);
    }

    #[test]
    fn input_part_1() {
        let codes = vec![
            ("480A", "^^<<A^>AvvvA>A"),
            ("965A", "^^^AvA<Avv>A"),
            ("140A", "^<<A^A>vvA>A"),
            ("341A", "^A<<^AvA>>vA"),
            ("285A", "<^A^^AvAvv>A"),
        ];

        let mut final_score = 0;
        codes.iter().for_each(|(code, keys)| {
            let strokes = cost(Strokes(map_code(keys)), 2);
            let result = score(&strokes, code);
            final_score += result;
        });

        println!("{}", final_score);
        assert_eq!(final_score, 152942);
    }

    #[test]
    fn test_input_depth() {
        let codes = vec![
            ("480A", "^^<<A^>AvvvA>A"),
            ("965A", "^^^AvA<Avv>A"),
            ("140A", "^<<A^A>vvA>A"),
            ("341A", "^A<<^AvA>>vA"),
            ("285A", "<^A^^AvAvv>A"),
        ];

        let mut final_score = 0;
        codes.iter().for_each(|(code, keys)| {
            let strokes = cost(Strokes(map_code(keys)), 16);
            let result = score(&strokes, code);
            final_score += result;
        });

        println!("{}", final_score);
    }

    #[test]
    fn input_part_2() {
        let codes = vec![
            ("480A", "^^<<A^>AvvvA>A"),
            ("965A", "^^^AvA<Avv>A"),
            ("140A", "^<<A^A>vvA>A"),
            ("341A", "^A<<^AvA>>vA"),
            ("285A", "<^A^^AvAvv>A"),
        ];

        let mut final_score = 0;
        codes.iter().for_each(|(code, keys)| {
            let strokes = cost(Strokes(map_code(keys)), 25);
            let result = score(&strokes, code);
            final_score += result;
        });

        println!("{}", final_score);
    }
}
