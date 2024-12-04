use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, Lines};

pub fn read_file(day_num: i32, filename: &str) -> Lines<BufReader<File>> {
    let reader = io::BufReader::new(
        File::open(format!("resources/day{day_num}/{filename}")).expect(&format!("Check the file path: resources/{day_num}/{filename}"))
    );

    reader.lines()
}