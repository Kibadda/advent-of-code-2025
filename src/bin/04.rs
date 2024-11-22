use std::collections::HashMap;

use regex::Regex;

advent_of_code::solution!(4);

fn check_passport(input: &str, check_function: fn(&HashMap<&str, &str>) -> bool) -> Option<u32> {
    let mut valid = 0;

    let mut passport = HashMap::new();

    input.lines().for_each(|line| {
        if line == "" {
            if check_function(&passport) {
                valid += 1;
            }
            passport.clear();
        } else {
            line.split_whitespace().for_each(|split| {
                let mut pair = split.split(':');
                let key = pair.next().unwrap();
                let value = pair.next().unwrap();

                passport.insert(key, value);
            });
        }
    });

    if check_function(&passport) {
        valid += 1;
    }

    Some(valid)
}

pub fn part_one(input: &str) -> Option<u32> {
    check_passport(input, |passport| {
        passport.len() == 8 || (passport.len() == 7 && !passport.contains_key("cid"))
    })
}

pub fn part_two(input: &str) -> Option<u32> {
    check_passport(input, |passport| {
        (match passport.get("byr") {
            Some(byr) => match byr.parse::<u32>() {
                Ok(byr) => byr >= 1920 && byr <= 2002,
                Err(_) => false
            }
            None => false
        })
        &&
        (match passport.get("iyr") {
            Some(iyr) => match iyr.parse::<u32>() {
                Ok(iyr) => iyr >= 2010 && iyr <= 2020,
                Err(_) => false
            }
            None => false
        })
        &&
        (match passport.get("eyr") {
            Some(eyr) => match eyr.parse::<u32>() {
                Ok(eyr) => eyr >= 2020 && eyr <= 2030,
                Err(_) => false
            }
            None => false
        })
        &&
        (match passport.get("hgt") {
            Some(hgt) => {
                let (height_str, unit) = hgt.split_at(hgt.len() - 2);
                let height = height_str.parse::<u32>().unwrap_or(0);

                match unit {
                    "cm" => height >= 150 && height <= 193,
                    "in" => height >= 59 && height <= 76,
                    _ => false
                }
            }
            None => false
        })
        &&
        (match passport.get("hcl") {
            Some(hcl) => Regex::new(r"^#[0-9a-f]{6}$").unwrap().is_match(hcl),
            None => false
        })
        &&
        (match passport.get("ecl") {
            Some(ecl) => Regex::new(r"^amb|brn|blu|gry|grn|hzl|oth$").unwrap().is_match(ecl),
            None => false
        })
        &&
        (match passport.get("pid") {
            Some(pid) => Regex::new(r"^[0-9]{9}$").unwrap().is_match(pid),
            None => false
        })
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
