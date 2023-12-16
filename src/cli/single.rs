use std::time::Duration;

use ansi_term::Colour::{Cyan, Red};
use clap::{Parser, ValueHint};

use crate::{
    cli::{
        runner::{DurationThresholds, Solver, SolverRunResult},
        source::{source_path_fill_tokens, Source, SourceValueParser},
    },
    derived::Day,
};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct SingleArgs {
    /// Path to a file containing the input.
    #[arg(
        value_hint = ValueHint::FilePath,
        default_value = "inputs/day{day0}.txt",
        value_parser = SourceValueParser,
    )]
    input: Source,

    /// Path to a file containing the expected result of part 1.
    #[arg(
        value_hint = ValueHint::FilePath,
        default_value = "inputs/day{day0}.solution{part}.txt",
        value_parser = SourceValueParser,
    )]
    part1: Source,

    /// Path to a file containing the expected result of part 2.
    #[arg(
        value_hint = ValueHint::FilePath,
        default_value = "inputs/day{day0}.solution{part}.txt",
        value_parser = SourceValueParser,
    )]
    part2: Source,
}

const THRESHOLDS: DurationThresholds = DurationThresholds {
    good: Duration::from_millis(1),
    acceptable: Duration::from_secs(1),
};

pub fn main(day: &Day) {
    let args = SingleArgs::parse();

    let input_path = source_path_fill_tokens!(args.input, day = day);
    let part1_path = source_path_fill_tokens!(args.part1, day = day, part = 1);
    let part2_path = source_path_fill_tokens!(args.part2, day = day, part = 2);

    println!(
        "Running {} using input {}...",
        Cyan.paint(format!("day {}", day.num)),
        Cyan.paint(input_path.source().unwrap()),
    );

    let input = match input_path.read() {
        Ok(input) => input,
        Err(err) => {
            println!("{}", Red.paint(err));
            return;
        }
    };

    for (i, part, solution_path) in [(1, &day.part1, part1_path), (2, &day.part2, part2_path)] {
        let solution = solution_path.read_maybe();
        let result = match solution {
            Ok(solution) => {
                let solver: Solver<_> = (*part).into();
                solver.run(&input, solution)
            }
            Err(err) => SolverRunResult::Error(err),
        };
        result.print(&format!("Part {i}"), &THRESHOLDS, true);
    }
}

#[macro_export]
macro_rules! __generate_day_main {
    () => {
        #[::aoc_derive::inject_day]
        static DAY: Day;

        pub fn main() {
            ::aoc::cli::single::main(&*DAY);
        }
    };
}
pub use __generate_day_main as generate_main;
