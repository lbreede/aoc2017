use std::time::Instant;

const INPUT: u32 = 368_078;

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

fn part_one(input: u32) -> u32 {
    let mut diameter = (input as f32).sqrt().ceil() as u32;
    if diameter % 2 == 0 {
        diameter += 1;
    }

    let dist_to_corner = (0..4)
        .map(|i| {
            let corner = diameter * diameter - i * (diameter - 1);
            corner.abs_diff(input)
        })
        .min()
        .unwrap();
    diameter - 1 - dist_to_corner
}

fn part_two(input: u32) -> u32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_example() {
        assert_eq!(part_one(1), 0);
        assert_eq!(part_one(12), 3);
        assert_eq!(part_one(23), 2);
        assert_eq!(part_one(1024), 31);
    }

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(INPUT), 371);
    }

    // #[test]
    // fn test_part_two_example() {
    // assert_eq!(part_two(example), 9);
    // }
    //
    // #[test]
    // fn test_part_two() {
    // assert_eq!(part_two(INPUT), 244);
    // }
}
