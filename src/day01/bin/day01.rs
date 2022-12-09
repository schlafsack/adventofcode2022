use std::fs::File;
use std::io::{BufRead, BufReader};

fn max(lines: impl Iterator<Item = String>, len: usize) -> usize {
    let mut result: Vec<usize> = vec![0; len];
    let mut totcal = 0usize;

    for line in lines {
        let cal = str::parse::<usize>(&line);
        if cal.is_ok() {
            totcal += cal.unwrap();
        } else {
            if result[0] < totcal {
                result[0] = totcal;
                result.sort();
            }
            totcal = 0;
        }
    }
    return result.iter().sum();
}

#[test]
fn test_part_one() {
    let input = getinput("src/day01/bin/input.txt");
    let result = max(input, 1);
    assert_eq!(result, 72240);
}

#[test]
fn test_part_two() {
    let input = getinput("src/day01/bin/input.txt");
    let result = max(input, 3);
    assert_eq!(result, 210957);
}

fn getinput(path: &str) -> impl Iterator<Item = String> {
    let file = File::open(path).unwrap();
    return BufReader::new(file).lines().map(|x| x.unwrap());
}
