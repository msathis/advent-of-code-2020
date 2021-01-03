use crate::aoc::utils::Input;
use regex::Regex;

#[derive(Debug)]
struct Passport {
    byr: Option<String>,
    iyr: Option<String>,
    eyr: Option<String>,
    hgt: Option<String>,
    hcl: Option<String>,
    ecl: Option<String>,
    pid: Option<String>,
    cid: Option<String>,
}

impl Passport {
    fn new() -> Self {
        Self {
            byr: None,
            iyr: None,
            eyr: None,
            hgt: None,
            hcl: None,
            ecl: None,
            pid: None,
            cid: None,
        }
    }

    fn valid(&self) -> bool {
        if self.byr.is_some()
            && self.iyr.is_some()
            && self.eyr.is_some()
            && self.hgt.is_some()
            && self.hcl.is_some()
            && self.ecl.is_some()
            && self.pid.is_some()
        {
            return true;
        }
        false
    }

    fn byr_valid(&self) -> bool {
        if self.byr.is_some() {
            let byr = self.byr.clone().unwrap().clone().parse::<i32>().unwrap();
            return byr >= 1920 && byr <= 2002;
        }
        return false;
    }

    fn iyr_valid(&self) -> bool {
        if self.iyr.is_some() {
            let re = Regex::new(r"^\d{4}$").unwrap();
            let str = self.iyr.clone().unwrap().clone();

            if !re.is_match(&str) {
                return false;
            }

            let iyr = str.parse::<i32>().unwrap();
            return iyr >= 2010 && iyr <= 2020;
        }
        return false;
    }

    fn eyr_valid(&self) -> bool {
        if self.eyr.is_some() {
            let re = Regex::new(r"^\d{4}$").unwrap();
            let str = self.eyr.clone().unwrap().clone();

            if !re.is_match(&str) {
                return false;
            }
            let eyr = str.parse::<i32>().unwrap();
            return eyr >= 2020 && eyr <= 2030;
        }
        return false;
    }

    fn hgt_valid(&self) -> bool {
        if self.hgt.is_some() {
            let str = self.hgt.clone().unwrap();
            let re = Regex::new(r"([0-9]+)(cm|in)").unwrap();

            if !re.is_match(&str) {
                return false;
            }

            let captures = re.captures(&str).unwrap();

            match &captures[2] {
                "cm" => {
                    let cm = captures[1].parse::<i32>().unwrap();
                    return cm >= 150 && cm <= 193;
                }
                "in" => {
                    let inch = captures[1].parse::<i32>().unwrap();
                    return inch >= 59 && inch <= 76;
                }
                _ => return false,
            }
        }
        return false;
    }

    fn hcl_valid(&self) -> bool {
        if self.hcl.is_some() {
            let str = self.hcl.clone().unwrap();
            let re1 = Regex::new(r"^#([abcedf\d]{6})$").unwrap();
            return re1.is_match(&str);
        }

        return false;
    }

    fn ecl_valid(&self) -> bool {
        if self.ecl.is_some() {
            match self.ecl.clone().unwrap().clone().as_ref() {
                "amb" => {
                    return true;
                }
                "blu" => {
                    return true;
                }
                "brn" => {
                    return true;
                }
                "gry" => {
                    return true;
                }
                "grn" => {
                    return true;
                }
                "hzl" => {
                    return true;
                }
                "oth" => {
                    return true;
                }
                _ => {
                    return false;
                }
            }
        }
        return false;
    }

    fn pid_valid(&self) -> bool {
        if self.pid.is_some() {
            let str = self.pid.clone().unwrap();
            let re1 = Regex::new(r"^\d{9}$").unwrap();

            return re1.is_match(&str);
        }
        return false;
    }

    fn strict_valid(&self) -> bool {
        if self.byr_valid()
            && self.iyr_valid()
            && self.eyr_valid()
            && self.hgt_valid()
            && self.hcl_valid()
            && self.ecl_valid()
            && self.pid_valid()
        {
            return true;
        }
        false
    }
}

pub async fn part1() -> String {
    let count = count(false).await;
    count.to_string()
}

pub async fn part2() -> String {
    let count = count(true).await;
    count.to_string()
}

pub async fn count(strict: bool) -> i32 {
    let input = Input {
        path: "src/aoc/day4/input.txt".to_string(),
    };
    let lines = input.read().await;
    let mut valid = 0;
    let mut passport = Passport::new();

    for line in lines {
        if line.is_empty() {
            if (strict && passport.strict_valid()) || (!strict && passport.valid()) {
                valid += 1;
            }
            passport = Passport::new();
        } else {
            let tokens = line.split_whitespace().flat_map(|x| x.split(":"));
            let keys = tokens
                .clone()
                .step_by(2)
                .map(|s| s.to_string())
                .collect::<Vec<String>>();
            let values = tokens
                .skip(1)
                .step_by(2)
                .map(|s| s.to_string())
                .collect::<Vec<String>>();

            for i in 0..keys.len() {
                let key = keys.get(i).unwrap();
                let val = Some(values.get(i).unwrap().clone());

                match key.as_ref() {
                    "byr" => passport.byr = val,
                    "iyr" => passport.iyr = val,
                    "eyr" => passport.eyr = val,
                    "hgt" => passport.hgt = val,
                    "hcl" => passport.hcl = val,
                    "ecl" => passport.ecl = val,
                    "pid" => passport.pid = val,
                    "cid" => passport.cid = val,
                    _ => panic!("Invalid property"),
                }
            }
        }
    }

    valid
}
