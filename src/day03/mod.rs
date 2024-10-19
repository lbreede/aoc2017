use std::time::Instant;

const INPUT: &str = include_str!("input.txt");

pub fn main() {
    println!("Day 3: Spiral Memory");
    let now = Instant::now();
    let result = part_one(INPUT);
    let elapsed = now.elapsed();
    println!("Part 1: {} ({:.2?})", result, elapsed);

    let now = Instant::now();
    let result = part_two(INPUT);
    let elapsed = now.elapsed();
    println!("Part 2: {} ({:.2?})\n", result, elapsed);
}

fn part_one(input: &str) -> u32 {
    0
}

fn part_two(input: &str) -> u32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_example() {
        let example = include_str!("example.txt");
        assert_eq!(part_one(example), 18);
    }

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(INPUT), 36_174);
    }

    #[test]
    fn test_part_two_example() {
        let example = include_str!("example2.txt");
        assert_eq!(part_two(example), 9);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(INPUT), 244);
    }
}
