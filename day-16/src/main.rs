use std::{fs::File, io::{self, BufRead}};

struct Position {
    x: usize,
    y: usize,
}

enum State {
    Empty,
    Wall,
}

struct Cell {
    position: Position,
    state: State,
}

fn same_position(a: &Position, b: &Position) -> bool {
    a.x == b.x && a.y == b.y
}

fn main() -> io::Result<()> {
    let path = "example.txt";
    let input_matrix = File::open(&path)?;
    //read the file and put each character in a cell in a matrix
    let mut matrix: Vec<Vec<Cell>> = Vec::new();
    let mut start: Position = Position{x: 0, y: 0};
    let mut end: Position = Position{x: 0, y: 0};
    for line in io::BufReader::new(input_matrix).lines() {
        let line = line?;
        let mut row: Vec<Cell> = Vec::new();
        let mut r: usize = 0;
        for c in line.chars() {
            let state = match c {
                '.' => State::Empty,
                '#' => State::Wall,
                'S' => {
                    start = Position{x: r, y: matrix.len()};
                    State::Empty
                },
                'E' => {
                    end = Position{x: r, y: matrix.len()};
                    State::Empty
                },
                _ => panic!("Invalid character in input file"),
            };
            row.push(Cell{position: Position{x: row.len(), y: matrix.len()}, state: state});
            r += 1;
        }
        matrix.push(row);
    }

    println!("Start: ({}, {})", start.x, start.y);
    println!("End: ({}, {})", end.x, end.y);
    // print the matrix
    for row in matrix.iter() {
        for cell in row.iter() {
            let c = match cell.state {
                State::Empty => '.',
                State::Wall => '#',
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

    Ok(())
}