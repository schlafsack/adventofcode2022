use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn find(
    lines: impl Iterator<Item = String>,
    strategy: fn((usize, usize), (usize, usize)) -> bool,
) -> usize {
    lines
        .map(|x| parse_assignments(&x))
        .filter(|x| strategy(x.0, x.1) || strategy(x.1, x.0))
        .count()
}

fn contains(a: (usize, usize), b: (usize, usize)) -> bool {
    (a.0 <= b.0) && (a.1 >= b.1)
}

fn overlaps(a: (usize, usize), b: (usize, usize)) -> bool {
    (a.0 <= b.0) && (b.0 <= a.1)
}

fn parse_assignments(assignments: &str) -> ((usize, usize), (usize, usize)) {
    Regex::new(r"^(\d*)-(\d*),(\d*)-(\d*)$")
        .unwrap()
        .captures(assignments)
        .map(|x| {
            (
                (x[1].parse().unwrap(), x[2].parse().unwrap()),
                (x[3].parse().unwrap(), x[4].parse().unwrap()),
            )
        })
        .unwrap()
}

#[test]
fn test_part_one() {
    let input = getinput("src/day04/input.txt");
    let result = find(input, contains);
    assert_eq!(result, 573);
}

#[test]
fn test_part_two() {
    let input = getinput("src/day04/input.txt");
    let result = find(input, overlaps);
    assert_eq!(result, 867);
}

fn getinput(path: &str) -> impl Iterator<Item = String> {
    let file = File::open(path).unwrap();
    return BufReader::new(file).lines().map(|x| x.unwrap());
}
