use std::collections::HashSet;

use aoc::point::Point2;

type Point = Point2<usize>;
type Map = Vec<Point>;

fn parse_input(input: &str) -> Map {
    input
        .split('\n')
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter_map(|(x, c)| {
                    if c == '#' {
                        Some(Point::new(x, y))
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>()
        })
        .collect()
}

macro_rules! get_space {
    ($map:ident, $var:ident) => {{
        let coords: HashSet<_> = $map.into_iter().map(|p| p.$var).collect();
        let binding: HashSet<_> = (0..*coords.iter().max().unwrap()).collect();
        let space: HashSet<_> = binding.difference(&coords).cloned().collect();
        space
    }};
}

fn expand_space(map: &mut [Point]) {
    let x_space = get_space!(map, x);
    let y_space = get_space!(map, y);
    for point in map {
        point.x += (0..point.x).filter(|x| x_space.contains(x)).count();
        point.y += (0..point.y).filter(|y| y_space.contains(y)).count();
    }
}

pub fn part1(input: &str) -> usize {
    let mut map = parse_input(input);
    expand_space(&mut map);
    map.iter()
        .enumerate()
        .map(|(idx, first)| {
            map.iter()
                .skip(idx + 1)
                .map(|second| first.abs_diff(second).sum())
                .sum::<usize>()
        })
        .sum()
}

aoc::cli::single::generate_main!();

#[cfg(test)]
mod tests {
    use aoc_derive::example_input;
    use pretty_assertions::assert_eq;

    use super::*;

    #[example_input(part1 = 374, test)]
    static EXAMPLE_INPUT: &str = "
        ...#......
        .......#..
        #.........
        ..........
        ......#...
        .#........
        .........#
        ..........
        .......#..
        #...#.....
    ";

    #[test]
    fn example_parse() {
        let actual = parse_input(&EXAMPLE_INPUT);
        let expected = vec![
            Point::new(3, 0),
            Point::new(7, 1),
            Point::new(0, 2),
            Point::new(6, 4),
            Point::new(1, 5),
            Point::new(9, 6),
            Point::new(7, 8),
            Point::new(0, 9),
            Point::new(4, 9),
        ];
        assert_eq!(actual, expected);
    }

    #[test]
    fn expand_space() {
        let mut map = vec![
            Point::new(3, 0),
            Point::new(7, 1),
            Point::new(0, 2),
            Point::new(6, 4),
            Point::new(1, 5),
            Point::new(9, 6),
            Point::new(7, 8),
            Point::new(0, 9),
            Point::new(4, 9),
        ];
        super::expand_space(&mut map);
        let expected = vec![
            Point::new(4, 0),
            Point::new(9, 1),
            Point::new(0, 2),
            Point::new(8, 5),
            Point::new(1, 6),
            Point::new(12, 7),
            Point::new(9, 10),
            Point::new(0, 11),
            Point::new(5, 11),
        ];
        assert_eq!(map, expected);
    }
}
