advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    for line1 in input.lines() {
        for line2 in input.lines() {
            if line1.parse::<u32>().unwrap() + line2.parse::<u32>().unwrap() == 2020 {
                return Some(line1.parse::<u32>().unwrap() * line2.parse::<u32>().unwrap())
            }
        }
    }

    None
}

pub fn part_two(input: &str) -> Option<u32> {
    for line1 in input.lines() {
        for line2 in input.lines() {
            for line3 in input.lines() {
                if line1.parse::<u32>().unwrap() + line2.parse::<u32>().unwrap() + line3.parse::<u32>().unwrap() == 2020 {
                    return Some(line1.parse::<u32>().unwrap() * line2.parse::<u32>().unwrap() * line3.parse::<u32>().unwrap())
                }
            }
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(514579));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(241861950));
    }
}
