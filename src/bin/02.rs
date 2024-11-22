use regex::Regex;

advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u32> {
    let mut count = 0;

    let re = Regex::new(r"(?m)^([0-9]+)-([0-9]+) ([a-z]): ([a-z]+)$").unwrap();

    re.captures_iter(input).map(|c| c.extract()).for_each(|(_, [lower, upper, character, password])| {
        let amount = password.chars().filter(|c| *c == character.chars().next().unwrap()).count();

        if amount >= lower.parse().unwrap() && amount <= upper.parse().unwrap() {
            count += 1;
        }
    });

    Some(count)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut count = 0;

    let re = Regex::new(r"(?m)^([0-9]+)-([0-9]+) ([a-z]): ([a-z]+)$").unwrap();

    re.captures_iter(input).map(|c| c.extract()).for_each(|(_, [first, second, character, password])| {
        let c = character.chars().next().unwrap();

        if (password.chars().nth(first.parse::<usize>().unwrap() - 1).unwrap() == c) ^ (password.chars().nth(second.parse::<usize>().unwrap() - 1).unwrap() == c) {
            count += 1;
        }
    });

    Some(count)
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
        assert_eq!(result, Some(1));
    }
}
