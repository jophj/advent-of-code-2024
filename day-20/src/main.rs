use pathfinding::prelude::{astar, dfs};
use std::{
    collections::HashMap,
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
struct State {
    cell: Cell,
    direction: Direction,
}

fn same_position(a: &Position, b: &Position) -> bool {
    a == b
}

fn successors(state: State, matrix: &Vec<Vec<Cell>>) -> Vec<(State, usize)> {
    let mut successors = Vec::new();

    for next_direction in Direction::iterator() {
        let next_cell = match *next_direction {
            Direction::Up => &matrix[state.cell.position.y - 1][state.cell.position.x],
            Direction::Down => &matrix[state.cell.position.y + 1][state.cell.position.x],
            Direction::Left => &matrix[state.cell.position.y][state.cell.position.x - 1],
            Direction::Right => &matrix[state.cell.position.y][state.cell.position.x + 1],
        };
        if next_cell.content != Content::Wall {
            let state = State {
                cell: next_cell.clone(),
                direction: next_direction.clone(),
            };

            successors.push((state, 1));
        }
    }
    successors
}

fn print_matrix(matrix: &Vec<Vec<Cell>>) {
    for row in matrix {
        for cell in row {
            match cell.content {
                Content::Empty => print!("."),
                Content::Wall => print!("#"),
            }
        }
        println!();
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
    print_matrix(&matrix);
    println!("Start: ({}, {})", start.x, start.y);
    println!("End: ({}, {})", end.x, end.y);

    let initial_state = State {
        cell: matrix[start.y][start.x].clone(),
        direction: Direction::Right,
    };
    let result = astar(
        &initial_state,
        |p| successors(p.clone(), &&matrix),
        |s| 1,
        |p| same_position(&p.cell.position, &end),
    );

    let steps = result.unwrap().1;
    println!("Steps {}", steps);

    let mut cheats = HashMap::new();
    cheats.insert(0, (1, steps));
    // println!("{:?}", cheats);

    for i in 1..matrix.len() - 1 {
        for j in 1..matrix[i].len() - 1 {
            if &matrix[i][j].content == &Content::Wall {
                let cheat_cell = Cell {
                    content: Content::Empty,
                    position: Position { x: j, y: i },
                };

                matrix[i][j] = cheat_cell;
                // print_matrix(&matrix);
                let result = astar(
                    &initial_state,
                    |p| successors(p.clone(), &&matrix),
                    |s| 1,
                    |p| same_position(&p.cell.position, &end),
                );
                let cheated_steps = result.unwrap().1;
                let saved_steps = steps - cheated_steps;
                println!("Cheated Steps {}, saves {}", cheated_steps, saved_steps);
                if !cheats.contains_key(&saved_steps) {
                    cheats.insert(saved_steps, (1, cheated_steps));
                } else {
                    let count = cheats.get(&saved_steps).unwrap();
                    cheats.insert(saved_steps, (count.0 + 1, cheated_steps));
                }

                let restored_cell = Cell {
                    content: Content::Wall,
                    position: Position { x: j, y: i },
                };
                matrix[i][j] = restored_cell;
            }
        }
    }

    let mut saves_100 = 0;
    let mut sorted_keys: Vec<_> = cheats.into_iter().collect();
    sorted_keys.sort_by(|x, y| x.0.cmp(&y.0));
    for (key, value) in sorted_keys {
        println!(
            "Saved steps: {}, count: {}, cheated steps: {}",
            key, value.0, value.1
        );
        if key >= 100 {
            saves_100 += value.0;
        }
    }
    println!("Saves 100: {}", saves_100);
    Ok(())
}
