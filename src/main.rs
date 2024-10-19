use aoc_result::AOCResult;

mod aoc_result;

mod day01;

mod day02;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let result01 = day01::main();
    let result02 = day02::main();

    if &args[1] == "readme" {
        println!("# Advent of Code 2017\n");
        println!("| Day | Part 1 | Part 2 |");
        println!("|----:| ------ | ------ |");
        result01.readme();
        result02.readme();
    } else {
        println!("Advent of Code 2017\n");
        println!("{}", result01);
        println!("{}", result02);
    }
}
