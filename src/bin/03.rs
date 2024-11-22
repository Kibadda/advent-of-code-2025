advent_of_code::solution!(3);

fn check_slope(grid: Vec<Vec<char>>, slope: (usize, usize)) -> Option<u32> {
    let mut pos = (0, 0);
    let mut trees = 0;

    while pos.1 < grid.len() {
        if grid[pos.1][pos.0] == '#' {
            trees += 1;
        }

        pos.0 += slope.0;
        pos.0 %= grid[pos.1].len();
        pos.1 += slope.1;
    }

    Some(trees)
}

pub fn part_one(input: &str) -> Option<u32> {
    let grid = input.lines().map(|s| s.chars().collect()).collect::<Vec<Vec<char>>>();
    check_slope(grid, (3, 1))
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid = input.lines().map(|s| s.chars().collect()).collect::<Vec<Vec<char>>>();

    Some(vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)].iter().map(|slope| check_slope(grid.clone(), *slope).unwrap()).product())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(7));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(336));
    }
}
