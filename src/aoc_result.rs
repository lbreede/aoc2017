use std::{
    fmt::{self, Display},
    time::Duration,
};

#[derive(Debug)]
pub struct AOCResult<T: Display> {
    day: u8,
    part_one_result: T,
    part_one_elapsed: Duration,
    part_two_result: T,
    part_two_elapsed: Duration,
}

impl<T: Display> AOCResult<T> {
    pub fn new(
        day: u8,
        part_one_result: T,
        part_one_elapsed: Duration,
        part_two_result: T,
        part_two_elapsed: Duration,
    ) -> Self {
        Self {
            day,
            part_one_result,
            part_one_elapsed,
            part_two_result,
            part_two_elapsed,
        }
    }
    pub fn readme(&self) {
        println!(
            "| {} | {:.2?} | {:.2?} |",
            self.day, self.part_one_elapsed, self.part_two_elapsed
        );
    }
}

impl<T: Display> Display for AOCResult<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Day {:>2}: {:<7} ({:.2?})\n        {:<7} ({:.2?})\n",
            self.day,
            self.part_one_result,
            self.part_one_elapsed,
            self.part_two_result,
            self.part_two_elapsed
        )
    }
}
