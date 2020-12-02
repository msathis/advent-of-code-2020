use crate::aoc::utils::Input;
use parse_display::{Display as PDisplay, FromStr as PFromStr};
use std::error::Error;

pub async fn part1() -> String {
    let input = Input {
        path: "src/aoc/day2/input.txt".to_string()
    };
    
    let parts = parse_password_rules(input.read().await).unwrap();
    let mut count = 0;

    for rule in parts {
        let mut char_count = 0;
        
        for c in rule.password.chars() {
            if c == rule.letter {
                char_count = char_count + 1;
            }
        }

        if char_count >= rule.min && char_count <= rule.max {
            count = count + 1;
        }
    }

    count.to_string()
}

pub async fn part2() -> String {
    let input = Input {
        path: "src/aoc/day2/input.txt".to_string()
    };
    
    let parts = parse_password_rules(input.read().await).unwrap();
    let mut count = 0;

    for rule in parts {
        
        if ( rule.password.chars().nth((rule.min - 1) as usize).is_none() || rule.password.chars().nth((rule.min - 1) as usize).unwrap().ne(&rule.letter)) && rule.password.chars().nth((rule.max - 1) as usize).is_some() && rule.password.chars().nth((rule.max - 1) as usize).unwrap().eq(&rule.letter) {
            count = count + 1;
        } else if (rule.password.chars().nth((rule.max - 1) as usize).is_none() || rule.password.chars().nth((rule.max - 1) as usize).unwrap().ne(&rule.letter)) &&  rule.password.chars().nth((rule.min - 1) as usize).is_some() && rule.password.chars().nth((rule.min - 1) as usize).unwrap().eq(&rule.letter) {
            count = count + 1;
        }
    }

    count.to_string()
}

fn parse_password_rules(input: Vec<String>) -> Result<Vec<PasswordRule>, impl Error> {
    input.iter().map(|line| line.parse::<PasswordRule>())
        .collect()
}

#[derive(PDisplay, PFromStr, Debug)]
#[display("{min}-{max} {letter}: {password}")]
struct PasswordRule {
    min: u32,
    max: u32,
    letter: char,
    password: String,
}
