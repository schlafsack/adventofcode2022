use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn score(lines: impl Iterator<Item = String>) -> u32 {
    lines
        .map(|x| split_backpack(&x))
        .map(|x| sum_intersection(&x))
        .sum()
}

fn find_badges(lines: impl Iterator<Item = String>) -> u32 {
    lines
        .map(|x| parse_contents(&x))
        .collect::<Vec<_>>()
        .chunks(3)
        .map(|x| sum_intersection(&x.to_vec()))
        .sum()
}

fn sum_intersection(sets: &Vec<HashSet<u32>>) -> u32 {
    sets.iter()
        .cloned()
        .reduce(|a, b| &a & &b)
        .unwrap()
        .iter()
        .sum()
}

fn split_backpack(backpack: &str) -> Vec<HashSet<u32>> {
    let c = backpack.split_at(backpack.len() / 2);
    vec![parse_contents(c.0), parse_contents(c.1)]
}

fn parse_contents(contents: &str) -> HashSet<u32> {
    contents.chars().map(|x| parse_char(x)).collect()
}

fn parse_char(c: char) -> u32 {
    let x = c as u8;
    (if x >= b'a' {
        1 + x - b'a'
    } else {
        27 + x - b'A'
    }) as u32
}

#[test]
fn test_part_one() {
    let input = getinput("src/day03/bin/input.txt");
    let result = score(input);

    assert_eq!(result, 8202);
}

#[test]
fn test_part_two() {
    let input = getinput("src/day03/bin/input.txt");
    let result = find_badges(input);

    assert_eq!(result, 2864);
}

fn getinput(path: &str) -> impl Iterator<Item = String> {
    let file = File::open(path).unwrap();
    return BufReader::new(file).lines().map(|x| x.unwrap());
}
