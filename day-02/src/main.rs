use std::{fs::File, io::{self, BufRead}};

fn calculate_safety(levels: &Vec<u8>, dampened: bool) -> bool {
    let mut last_order = levels[0] > levels[1];
    for i in 0..levels.len() - 1 {
        let mut dampened_left = levels.clone();
        dampened_left.remove(if i == 0 { i } else { i -1 });
        let mut dampened_current = levels.clone();
        dampened_current.remove(i);
        let mut dampened_right = levels.clone();
        dampened_right.remove(if i + 1 < levels.len() { i + 1 } else { i });

        

        if (levels[i] > levels[i + 1]) != last_order {
            if dampened {
                return false
            }
            return calculate_safety(&dampened_left, true) || calculate_safety(&dampened_current, true) || calculate_safety(&dampened_right, true);
        }
        last_order = levels[i] > levels[i + 1];

        if (levels[i].abs_diff(levels[i + 1])) == 0 {
            if dampened {
                return false
            }
            return calculate_safety(&dampened_left, true) || calculate_safety(&dampened_current, true) || calculate_safety(&dampened_right, true);
        }

        if (levels[i].abs_diff(levels[i + 1])) > 3 {
            if dampened {
                return false
            }
            return calculate_safety(&dampened_left, true) || calculate_safety(&dampened_current, true) || calculate_safety(&dampened_right, true);
        }
    }
    true
}

fn main() -> io::Result<()> {
    let path = "input.txt";
    let input = File::open(&path)?;
    let buffered = io::BufReader::new(input);

    let test = vec![10, 12, 11, 10];
    let safe = calculate_safety(&test, false);
    println!("Levels: {:?}, Safe: {}", test, safe);

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