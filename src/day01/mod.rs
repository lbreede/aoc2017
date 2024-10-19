use std::time::Instant;

use crate::aoc_result::AOCResult;

const INPUT: &str = include_str!("input.txt");

pub fn main() -> AOCResult<u32> {
    let now = Instant::now();
    let part_one_result = part_one(INPUT);
    let part_one_elapsed = now.elapsed();

    let now = Instant::now();
    let part_two_result = part_two(INPUT);
    let part_two_elapsed = now.elapsed();
    AOCResult::new(
        1,
        part_one_result,
        part_one_elapsed,
        part_two_result,
        part_two_elapsed,
    )
}

fn part_one(input: &str) -> u32 {
    let input: Vec<u32> = input.chars().flat_map(|c| c.to_digit(10)).collect();
    let mut result = 0;
    let n = input.len();
    for i in 0..n {
        let a = input[i];
        if a == input[(i + 1) % n] {
            result += a;
        }
    }
    result
}

fn part_two(input: &str) -> u32 {
    let input: Vec<u32> = input.chars().flat_map(|c| c.to_digit(10)).collect();
    let mut result = 0;
    let n = input.len();
    for i in 0..n {
        let a = input[i];
        if a == input[(i + n / 2) % n] {
            result += a;
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_example() {
        assert_eq!(part_one("1122"), 3);
        assert_eq!(part_one("1111"), 4);
        assert_eq!(part_one("1234"), 0);
        assert_eq!(part_one("91212129"), 9);
    }

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(INPUT), 1341);
    }

    #[test]
    fn test_part_two_example() {
        assert_eq!(part_two("1212"), 6);
        assert_eq!(part_two("1221"), 0);
        assert_eq!(part_two("123425"), 4);
        assert_eq!(part_two("123123"), 12);
        assert_eq!(part_two("12131415"), 4);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(INPUT), 1348);
    }
}
