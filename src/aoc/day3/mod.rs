use crate::aoc::utils::Input;
use parse_display::{Display as PDisplay, FromStr as PFromStr};
use std::{error::Error, cmp::min};

pub async fn part1() -> String {
    let input = Input {
        path: "src/aoc/day3/input.txt".to_string()
    };
    let data = input.read_as_string().await;

    get_trees(&data, 1, 3).to_string()
}

pub async fn part2() -> String {
    let input = Input {
        path: "src/aoc/day3/input.txt".to_string()
    };
    let data = input.read_as_string().await;

    let result : usize = get_trees(&data, 1, 1)
    * get_trees(&data, 1, 3)
    * get_trees(&data, 1, 5)
    * get_trees(&data, 1, 7)
    * get_trees(&data, 2, 1);

    result.to_string()
}

fn get_trees(data: &String, down: usize, right: usize) -> usize {
    let mut hpos = 0;
    let mut trees = 0;

    for pat in data.split('\n').step_by(down).skip(1) {
        if pat.len() > 0 {
            hpos = (hpos + right) % 31;
            trees += if pat.chars().nth(hpos).expect("mappos") == '#' { 1 } else { 0 };
        }
    }

    trees
}
