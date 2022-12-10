use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};


// Would be nice to not have to load the entire file into a string, also mapping to a hashset
// is expensive.  Assumes ascii.
fn seek(input: String, size: usize) -> usize {
    input.as_bytes()
        .windows(size)
        .map(|x| x.iter().collect::<HashSet<&u8>>())
        .position(|x| x.len() == size)
        .map(|x| x + size)
        .unwrap()
}

#[test]
fn test_part_one() {
    let input = getinput("src/day06/input.txt");
    let result = seek(input, 4);
    assert_eq!(result, 1640);
}

#[test]
fn test_part_two() {
    let input = getinput("src/day06/input.txt");
    let result = seek(input, 14);
    assert_eq!(result, 3613);
}

fn getinput(path: &str) -> String {
    let file = File::open(path).unwrap();
    BufReader::new(file).lines().next().unwrap().unwrap()
}
