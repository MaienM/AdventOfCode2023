#![allow(unused_variables)]

use aoc::generate_day_main;

fn parse_input(input: &str) -> usize {
    0
}

pub fn part1(input: &str) -> usize {
    let input = parse_input(input);
    0
}

generate_day_main!(part1);

#[cfg(test)]
mod tests {
    use aoc_derive::example_input;
    use pretty_assertions::assert_eq;

    use super::*;

    #[example_input(part1 = 1, test)]
    static EXAMPLE_INPUT: &str = "
        FOO
        BAR
    ";

    #[test]
    fn example_parse() {
        let actual = parse_input(&EXAMPLE_INPUT);
        let expected = 1;
        assert_eq!(actual, expected);
    }
}
