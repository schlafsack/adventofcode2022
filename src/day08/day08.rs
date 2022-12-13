use ndarray::{s, Array2};
use std::fs::File;
use std::io::{BufRead, BufReader};
use take_until::TakeUntilExt;

fn load_map(input: impl Iterator<Item=String>) -> Array2<u32> {
    let data: Vec<Vec<u32>> = input
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();
    Array2::<u32>::from_shape_vec((data.len(), data[0].len()), data.iter().flatten().cloned().collect()).unwrap()
}

fn find_visible_trees(input: impl Iterator<Item=String>) -> usize {
    let map = load_map(input);
    map.indexed_iter()
        .filter(|&((row, col), height)|
            map.slice(s![row, 0..col;-1]).iter().all(|n| n < height)
            || map.slice(s![row, col + 1..map.ncols()]).iter().all(|n| n < height)
            || map.slice(s![0..row;-1, col]).iter().all(|n| n < height)
            || map.slice(s![row + 1..map.nrows(), col]).iter().all(|n| n < height)
        )
        .count()
}

fn find_best_view(input: impl Iterator<Item=String>) -> usize {
    let map = load_map(input);
    map.indexed_iter()
        .map(|((row, col), height)|
            map.slice(s![row, 0..col;-1]).iter().take_until(|&n| n >= height).count()
            * map.slice(s![row, col + 1..map.ncols()]).iter().take_until(|&n| n >= height).count()
            * map.slice(s![0..row;-1, col]).iter().take_until(|&n| n >= height).count()
            * map.slice(s![row + 1..map.nrows(), col]).iter().take_until(|&n| n >= height).count()
        )
        .max()
        .unwrap()
}

#[test]
fn test_part_one() {
    let input = getinput("src/day08/input.txt");
    let result = find_visible_trees(input);
    assert_eq!(result, 1805);
}

#[test]
fn test_part_two() {
    let input = getinput("src/day08/input.txt");
    let result = find_best_view(input);
    assert_eq!(result, 444528);
}

fn getinput(path: &str) -> impl Iterator<Item=String> {
    let file = File::open(path).unwrap();
    return BufReader::new(file).lines().map(|x| x.unwrap());
}
