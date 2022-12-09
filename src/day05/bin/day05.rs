use regex::Regex;
use std::collections::VecDeque;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn organise(mut lines: impl Iterator<Item = String>, is_9000: bool) -> String {
    // parse and build stacks
    let mut stacks = lines
        .by_ref()
        .take_while(|x| x.len() > 0)
        .flat_map(|line| {
            line.chars()
                .collect::<Vec<char>>()
                .chunks(4)
                .map(|chunk| chunk[1])
                .enumerate()
                .filter(|slot| slot.1.is_alphabetic())
                .collect::<Vec<(usize, char)>>()
        })
        .fold(Vec::new(), |mut stacks: Vec<VecDeque<char>>, item| {
            stacks.resize(usize::max(stacks.len(), item.0 + 1), VecDeque::new());
            stacks[item.0].push_front(item.1);
            stacks
        });

    // follow moving instructions
    let re = Regex::new(r"^\D*(\d*)\D*(\d*)\D*(\d*)$").unwrap();
    lines
        .map(|line| {
            re.captures(&line)
                .map(|cap| {
                    (
                        cap[1].parse::<usize>().unwrap(),
                        cap[2].parse::<usize>().unwrap() - 1,
                        cap[3].parse::<usize>().unwrap() - 1,
                    )
                })
                .unwrap()
        })
        .for_each(|instruction| {
            let len = stacks[instruction.1].len() - instruction.0;
            let mut load = stacks[instruction.1].split_off(len);
            if is_9000 {
                load.make_contiguous().reverse();
            }
            stacks[instruction.2].append(&mut load);
        });

    // concat the top items in each stack.
    stacks
        .iter()
        .map(|stack| stack.back().unwrap().to_string())
        .reduce(|items, item| items + &item)
        .unwrap()
}

#[test]
fn test_part_one() {
    let input = getinput("src/day05/bin/input.txt");
    let result = organise(input, true);
    assert_eq!(result, "MQTPGLLDN");
}

#[test]
fn test_part_two() {
    let input = getinput("src/day05/bin/input.txt");
    let result = organise(input, false);
    assert_eq!(result, "LVZPSTTCZ");
}

fn getinput(path: &str) -> impl Iterator<Item = String> {
    let file = File::open(path).unwrap();
    return BufReader::new(file).lines().map(|x| x.unwrap());
}
