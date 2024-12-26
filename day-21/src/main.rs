use core::fmt;
use lazy_static::lazy_static;
use std::{collections::HashMap, fmt::Display, vec};

fn main() {
    println!("AoC 21");
}

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

lazy_static! {
    static ref MAP: HashMap<(Direction, Direction), Vec<Direction>> = {
        HashMap::from([
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
        ])
    };
}

fn map_strokes(start: &Direction, end: &Direction) -> Vec<Direction> {
    let mut mapped = MAP.get(&(*start, *end)).unwrap().clone();
    mapped.push(Direction::Forward);
    mapped
}

fn cost(mut strokes: Strokes, depth: usize) -> Strokes {
    for _ in 0..depth {
        let mut mapped = Vec::with_capacity(strokes.0.len() * 2);

        mapped.extend(map_strokes(&Direction::Forward, &strokes.0[0]));

        for window in strokes.0.windows(2) {
            if let [current, next] = window {
                mapped.extend(map_strokes(current, next));
            }
        }

        strokes = Strokes(mapped);
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
    fn ULDR_should_map_to_LADRARARA() {
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
    fn UL_should_map_to_LADLA() {
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
            let strokes = cost(Strokes(map_code(keys)), 14);
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
