use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn find_sizes(input: impl Iterator<Item = String>) -> HashMap<String, usize> {
    let mut stack = Vec::new();
    let mut dirs = HashMap::new();
    input.for_each(|line| match &line {
        x if x.starts_with("$ cd ..") => { stack.pop(); }
        x if x.starts_with("$ cd ") => { stack.push(x.clone()); }
        x if x.as_bytes()[0].is_ascii_digit() => {
            let size = x.split_once(" ").unwrap().0.parse::<usize>().unwrap();
            for i in 1..=stack.len() {
                dirs.entry(stack[..i].concat())
                    .and_modify(|tot| *tot += size)
                    .or_insert(size);
            }}
        _ => ()
    });
    dirs
}

fn find_smallest_dirs(input: impl Iterator<Item = String>, size: usize) -> usize {
    find_sizes(input)
        .iter()
        .filter(|x| *x.1 <= size)
        .map(|x| *x.1)
        .sum()
}

fn find_dir_to_delete(input: impl Iterator<Item = String>, total: usize, req: usize) -> usize {
    let dirs = find_sizes(input);
    let find = req - (total - dirs["$ cd /"]);
    dirs.iter()
        .filter(|x| *x.1 >= find)
        .min_by_key(|x| *x.1)
        .map(|x| *x.1)
        .unwrap()
}

#[test]
fn test_part_one() {
    let input = getinput("src/day07/input.txt");
    let result = find_smallest_dirs(input, 100000);
    assert_eq!(result, 1667443);
}

#[test]
fn test_part_two() {
    let input = getinput("src/day07/input.txt");
    let result = find_dir_to_delete(input, 70000000, 30000000);
    assert_eq!(result, 8998590);
}

fn getinput(path: &str) -> impl Iterator<Item = String> {
    let file = File::open(path).unwrap();
    return BufReader::new(file).lines().map(|x| x.unwrap());
}
