use regex::Regex;
use std::collections::VecDeque;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn organise(mut lines: impl Iterator<Item = String>, is_9000: bool) -> String {
    // parse and build stacks
    let mut stacks = lines
        .by_ref()
        .take_while(|x| x.len() > 0)
        .flat_map(|x| {
            x.chars()
                .collect::<Vec<char>>()
                .chunks(4)
                .map(|x| x[1])
                .enumerate()
                .filter(|x| x.1.is_alphabetic())
                .collect::<Vec<(usize, char)>>()
        })
        .fold(Vec::new(), |mut stacks: Vec<VecDeque<char>>, item| {
            stacks.resize(usize::max(stacks.len(), item.0 + 1), VecDeque::new());
            stacks[item.0].push_front(item.1);
            stacks
        });

    // follow moving instructions
    let re = Regex::new(r"^\D*(\d*)\D*(\d*)\D*(\d*)$").unwrap();
    lines.map(|x| {
        re.captures(&x)
            .map(|cap| {
                (
                    cap[1].parse::<usize>().unwrap(),
                    cap[2].parse::<usize>().unwrap() - 1,
                    cap[3].parse::<usize>().unwrap() - 1,
                )
            })
            .unwrap()
    })
        .for_each(|x| {
            let len = stacks[x.1].len() - x.0;
            let mut a = stacks[x.1].split_off(len);
            if is_9000 {
                a.make_contiguous().reverse();
            }
            stacks[x.2].append(&mut a);
        });

    stacks
        .iter()
        .map(|x| x.back().unwrap().to_string())
        .reduce(|x, y| x + &y)
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
