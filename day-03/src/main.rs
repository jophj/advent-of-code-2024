use std::io;
use std::io::prelude::*;
use std::fs::File;
use regex::Regex;

fn main() -> io::Result<()> {
    let mut f = File::open("example.txt")?;
    let mut buffer = String::new();

    f.read_to_string(&mut buffer)?;

    // regex matching
    let re = Regex::new(r"(mul\((\d{0,3}),(\d{0,3})\))").unwrap();

    let results = re.find_iter( &buffer).map(|m| {
        println!("Found match: {}", &buffer[m.start()..m.end()]);
        m.as_str()
    });

    Ok(())
}