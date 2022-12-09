use std::fs::File;
use std::io::{BufRead, BufReader};

fn score(lines: impl Iterator<Item=String>, strategy: fn(i32, i32) -> i32) -> i32 {
    lines
        .map(|x| x.to_ascii_lowercase())
        .map(|x| parse_game(x))
        .map(|x| strategy(x.0, x.1))
        .sum()
}

fn parse_game(game: String) -> (i32, i32) {
    // normalise plays to 1 (rock), 2 (paper), 3 (scissors)
    game.split_once(" ")
        .map(|p| {
            (p.0.chars().nth(0).unwrap() as i32 - 96,
             p.1.chars().nth(0).unwrap() as i32 - 119)
        })
        .unwrap()
}

fn play_strategy(them: i32, us: i32) -> i32 { us + ((((us - them).rem_euclid(3) + 1) % 3) * 3) }

fn rig_strategy(them: i32, result: i32) -> i32 { (((them + result).rem_euclid(3)) + 1) + ((result - 1) * 3) }

#[test]
fn test_part_one() {
    let input = getinput("src/day02/bin/input.txt");
    let result = score(input, play_strategy);

    assert_eq!(result, 9651);
}

#[test]
fn test_part_two() {
    let input = getinput("src/day02/bin/input.txt");
    let result = score(input, rig_strategy);
    assert_eq!(result, 10560);
}

fn getinput(path: &str) -> impl Iterator<Item=String> {
    let file = File::open(path).unwrap();
    return BufReader::new(file).lines().map(|x| x.unwrap());
}
