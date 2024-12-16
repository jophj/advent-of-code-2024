use pathfinding::prelude::dijkstra;
use std::{
    fs::File,
    io::{self, BufRead},
    slice::Iter,
};

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Position {
    x: usize,
    y: usize,
}

#[derive(PartialEq, Clone, Debug, Eq, Hash, Ord, PartialOrd)]
enum Content {
    Empty,
    Wall,
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Cell {
    position: Position,
    content: Content,
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}
impl Direction {
    pub fn iterator() -> Iter<'static, Direction> {
        static DIRECTIONS: [Direction; 4] = [
            Direction::Right,
            Direction::Up,
            Direction::Left,
            Direction::Down,
        ];
        DIRECTIONS.iter()
    }
}
#[derive(Clone, Debug, Eq, Hash, PartialEq, PartialOrd)]
struct State<'a> {
    cell: &'a Cell,
    direction: &'a Direction,
}

fn same_position(a: &Position, b: &Position) -> bool {
    a.x == b.x && a.y == b.y
}

impl State<'_> {
    fn successors<'a>(&self, matrix: &'a Vec<Vec<Cell>>) -> Vec<(State<'a>, usize)> {
        let &State {
            ref cell,
            ref direction,
        } = self;
        let mut successors = Vec::new();
        // for each Direction, check if the next cell is not a wall
        // if it is not a wall, add it to the successors
        for next_direction in Direction::iterator() {
            let next_cell = match *next_direction {
                Direction::Up => &matrix[cell.position.y - 1][cell.position.x],
                Direction::Down => &matrix[cell.position.y + 1][cell.position.x],
                Direction::Left => &matrix[cell.position.y][cell.position.x - 1],
                Direction::Right => &matrix[cell.position.y][cell.position.x + 1],
            };
            if next_cell.content != Content::Wall {
                successors.push((
                    State {
                        cell: next_cell,
                        direction: next_direction,
                    },
                    if &next_direction == direction {
                        1
                    } else {
                        1001
                    },
                ));
            }
        }
        successors
    }
}

fn main() -> io::Result<()> {
    let path = "input.txt";
    let input_matrix = File::open(&path)?;
    //read the file and put each character in a cell in a matrix
    let mut matrix: Vec<Vec<Cell>> = Vec::new();
    let mut start: Position = Position { x: 0, y: 0 };
    let mut end: Position = Position { x: 0, y: 0 };
    for line in io::BufReader::new(input_matrix).lines() {
        let line = line?;
        let mut row: Vec<Cell> = Vec::new();
        let mut r: usize = 0;
        for c in line.chars() {
            let state = match c {
                '.' => Content::Empty,
                '#' => Content::Wall,
                'S' => {
                    start = Position {
                        x: r,
                        y: matrix.len(),
                    };
                    Content::Empty
                }
                'E' => {
                    end = Position {
                        x: r,
                        y: matrix.len(),
                    };
                    Content::Empty
                }
                _ => panic!("Invalid character in input file"),
            };
            row.push(Cell {
                position: Position {
                    x: row.len(),
                    y: matrix.len(),
                },
                content: state,
            });
            r += 1;
        }
        matrix.push(row);
    }

    // print the matrix
    for row in matrix.iter() {
        for cell in row.iter() {
            let c = match cell.content {
                Content::Empty => '.',
                Content::Wall => '#',
            };
            if same_position(&cell.position, &start) {
                print!("S");
            } else if same_position(&cell.position, &end) {
                print!("E");
            } else {
                print!("{}", c);
            }
        }
        println!();
    }

    println!("Start: ({}, {})", start.x, start.y);
    println!("End: ({}, {})", end.x, end.y);

    let initial_state = State {
        cell: &matrix[start.y][start.x],
        direction: &Direction::Right,
    };
    let mut current_state = initial_state;
    let result = dijkstra(
        &current_state,
        |p| p.successors(&matrix),
        |p| same_position(&p.cell.position, &end),
    );

    let unwrapped_result = result.unwrap();
    println!("Path found: {:?}", unwrapped_result.0);
    println!("Steps: {:?}", unwrapped_result.0.len());
    println!("Cost: {:?}", unwrapped_result.1);

    for row in matrix.iter() {
        for cell in row.iter() {
            let c = match cell.content {
                Content::Empty => '.',
                Content::Wall => '#',
            };
            let is_in_path = unwrapped_result
                .0
                .iter()
                .any(|p| same_position(&p.cell.position, &cell.position));
            if same_position(&cell.position, &start) {
                print!("S");
            } else if same_position(&cell.position, &end) {
                print!("E");
            } else {
                if is_in_path {
                    print!("*");
                } else {
                    print!("{}", c);
                }
            }
        }
        println!();
    }
    Ok(())
}
