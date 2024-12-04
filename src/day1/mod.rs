use advent_of_code_2025::read_file;
use std::collections::HashMap;
use std::num::ParseIntError;

/// AOC - day 1: https://adventofcode.com/2024/day/1
/// Complexity: O(n)
pub fn run_1(filename: &str) -> (i32, i32) {

    let lines = read_file(1, filename).collect::<Vec<_>>();

    let mut left: Vec<i32> = Vec::new();
    let mut right: Vec<i32> = Vec::new();

    let mut right_counter: HashMap<i32, i32> = HashMap::new();

    for i in 0..lines.len(){
        let line = lines[i].as_ref().unwrap();
        let elements: Result<Vec<i32>, ParseIntError> = line.split_ascii_whitespace().map(|l| l.parse::<i32>()).collect();
        left.push(elements.as_ref().unwrap()[0]);
        right.push(elements.as_ref().unwrap()[1]);

        *right_counter.entry(elements.as_ref().unwrap()[1]).or_insert(0) += 1;
    }

    left.sort();
    right.sort();

    let mut distance: i32 = 0;
    let mut similarity: i32 = 0;

    for i in 0..left.len().min(right.len()) {
        distance += (left[i] - right[i]).abs();
        similarity += right_counter.get(&left[i]).unwrap_or(&0) * left[i];
    }

    (distance, similarity)
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_happy_path(){
        assert_eq!((11, 31), run_1("happy_path.txt"));
    }

    #[test]
    fn test_input(){
        assert_eq!((1506483, 23126924), run_1("input.txt"));
    }
}