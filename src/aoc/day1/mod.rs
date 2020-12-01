use std::collections::HashSet;

use crate::aoc::utils::Input;

pub async fn part1() -> String {
    let input = Input {
        path: "src/aoc/day1/input.txt".to_string()
    };
    
    let lines = input.read().await;
    let vec = lines.iter().map(|f| f.parse::<i32>().unwrap()).collect::<Vec<i32>>();
    let mut set: HashSet<i32> = HashSet::new();

    for num in vec.iter() {
        let target = 2020_i32 - num;
        if set.contains(&target) {
            return (target * num as &i32).to_string();
        }
        set.insert(*num);
    }

    "".to_string()
}

pub async fn part2() -> String {
    let input = Input {
        path: "src/aoc/day1/input.txt".to_string()
    };
    
    let lines = input.read().await;
    let vec = lines.iter().map(|f| f.parse::<i32>().unwrap()).collect::<HashSet<i32>>();

    for i in vec.iter() {
        for j in vec.iter() {
            let diff = i + j;
            let target = 2020_i32 - diff;
            if vec.contains(&target) {
                return (target * i * j as &i32).to_string();
            }
        }
    }

    "".to_string()
}
