use std::{fs::File, io::{self, BufRead}};

fn calculate_safety(levels: &Vec<u8>, dampened: bool) -> bool {
    let order = levels[1] > levels[0];
    for i in 1..levels.len() {
        if (levels[i] > levels[i - 1]) != order {
            if dampened {
                return false
            }
            let mut dampened = levels.clone();
            dampened.remove(i);
            return calculate_safety(&dampened, true);
        }
        if (levels[i].abs_diff(levels[i - 1])) == 0 {
            if dampened {
                return false
            }
            let mut dampened = levels.clone();
            dampened.remove(i);
            return calculate_safety(&dampened, true);        }

        if (levels[i].abs_diff(levels[i - 1])) > 3 {
            if dampened {
                return false
            }
            let mut dampened = levels.clone();
            dampened.remove(i);
            return calculate_safety(&dampened, true);        }
    }
    true
}

fn main() -> io::Result<()> {
    let path = "example.txt";
    let input = File::open(&path)?;
    let buffered = io::BufReader::new(input);

    let mut score = 0;
    for line in buffered.lines() {
        if let Ok(line) = line {
            let parts = line.split_whitespace();
            let levels = parts.map(|x| x.parse::<u8>().unwrap()).collect();
            let safe = calculate_safety(&levels, false);
            println!("Levels: {:?}, Safe: {}", levels, safe);
            if safe {
                score += 1;
            }
        }
    }

    println!("Score: {}", score);

    Ok(())
}