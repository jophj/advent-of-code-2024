use std::fs::File;
use std::io::{self, BufRead};

fn calculate_total_distance(left: Vec<i32>, right: Vec<i32>) -> i32 {
    let mut left_sorted = left.clone();
    let mut right_sorted = right.clone();
    
    left_sorted.sort();
    right_sorted.sort();
    
    left_sorted.iter().zip(right_sorted.iter())
        .map(|(l, r)| (l - r).abs())
        .sum()
}

fn calculate_similarity(left: Vec<i32>, right: Vec<i32>) -> i32 {
    left.iter()
        .map(|l| {
            let occurrences = right
                .iter()
                .map(|r| if l == r { 1 } else { 0 } )
                .sum::<i32>();

                l * occurrences
        })
        .sum()
}

fn main() -> io::Result<()> {
    let path = "input.txt";
    let input = File::open(&path)?;
    let buffered = io::BufReader::new(input);

    let mut left: Vec<i32> = Vec::new();
    let mut right: Vec<i32> = Vec::new();

    for line in buffered.lines() {
        if let Ok(line) = line {
            let mut parts = line.split_whitespace();
            if let (Some(part1), Some(part2)) = (parts.next(), parts.next()) {
                if let (Ok(num1), Ok(num2)) = (part1.parse::<i32>(), part2.parse::<i32>()) {
                    left.push(num1);
                    right.push(num2);
                }
            }
        }
    }

    let total_distance = calculate_total_distance(left.clone(), right.clone());
    println!("Total distance: {}", total_distance);

    let similarity = calculate_similarity(left, right); 
    println!("Similarity: {}", similarity);

    Ok(())
}