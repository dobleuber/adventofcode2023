use crate::utils::open_file;
use std::collections::BTreeMap;

pub fn day14() -> Result<usize, std::io::Error> {
    let contents = open_file("./inputs/14/input.txt")?;
    let result = resolve_puzzle(&contents);

    Ok(result)
}

fn resolve_puzzle(input: &str) -> usize {
    let mut platform = Platform::parse(input);
    platform.tilt_platform(TiltDirection::North);
    platform.total_load()
}

fn resolve_puzzle2(input: &str) -> usize {
    let mut platform = Platform::parse(input);
    for _ in 0..1_000_000_000 {
        platform.cycle();
    }

    platform.total_load()
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum TileType {
    RoundRock,
    CubeRock,
    Empty,
}

#[derive(Debug, PartialEq, Eq)]
struct Platform {
    grid: BTreeMap<(usize, usize), TileType>,
    rows: usize,
    cols: usize,
}

enum TiltDirection {
    North,
    South,
    East,
    West,
}

impl Platform {
    fn parse(input: &str) -> Self {
        let grid = input
            .lines()
            .enumerate()
            .flat_map(|(x, line)| {
                line.chars().enumerate().map(move |(y, c)| match c {
                    'O' => ((y, x), TileType::RoundRock),
                    '#' => ((y, x), TileType::CubeRock),
                    '.' => ((y, x), TileType::Empty),
                    _ => panic!("invalid char"),
                })
            })
            .collect();
        let rows = input.lines().count();
        let cols = input.lines().next().unwrap().chars().count();

        Self { grid, rows, cols }
    }

    fn tilt_platform(&mut self, direction: TiltDirection) {
        use TileType::*;
        let mut grid = self.grid.clone();

        loop {
            let mut changed = false;

            for y in 0..self.cols {
                for x in 0..self.rows {
                    match direction {
                        TiltDirection::North => {
                            let current = grid.get(&(y, x));
                            let next = grid.get(&(y, x + 1));

                            if let (Some(Empty), Some(RoundRock)) = (current, next) {
                                grid.insert((y, x), RoundRock);
                                grid.insert((y, x + 1), Empty);
                                changed = true;
                            }
                        }
                        TiltDirection::South => {
                            let current = grid.get(&(y, x));
                            let next = grid.get(&(y, x.saturating_sub(1)));

                            if let (Some(Empty), Some(RoundRock)) = (current, next) {
                                grid.insert((y, x), RoundRock);
                                grid.insert((y, x - 1), Empty);
                                changed = true;
                            }
                        }
                        TiltDirection::East => {
                            let current = grid.get(&(y, x));
                            let next = grid.get(&(y.saturating_sub(1), x));

                            if let (Some(Empty), Some(RoundRock)) = (current, next) {
                                grid.insert((y, x), RoundRock);
                                grid.insert((y - 1, x), Empty);
                                changed = true;
                            }
                        }
                        TiltDirection::West => {
                            let current = grid.get(&(y, x));
                            let next = grid.get(&(y + 1, x));

                            if let (Some(Empty), Some(RoundRock)) = (current, next) {
                                grid.insert((y, x), RoundRock);
                                grid.insert((y + 1, x), Empty);
                                changed = true;
                            }
                        }
                    }
                }
            }

            if changed {
                self.grid = grid.clone();
            } else {
                break;
            }
        }
    }

    fn cycle(&mut self) {
        self.tilt_platform(TiltDirection::North);
        self.tilt_platform(TiltDirection::West);
        self.tilt_platform(TiltDirection::South);
        self.tilt_platform(TiltDirection::East);
    }

    fn total_load(&self) -> usize {
        let total = self.grid.iter().fold(0, |acc, (pos, tile)| {
            let (x, y) = pos;
            let load = match tile {
                TileType::RoundRock => self.rows - y,
                _ => 0,
            };

            acc + load
        });
        total
    }
}

mod tests {
    use super::*;

    #[test]
    fn parse() {
        let input = "OOOO.#.O..
OO..#....#
OO..O##..O";

        let platform = Platform::parse(input);

        let expected_values = vec![
            ((0, 0), TileType::RoundRock),
            ((1, 0), TileType::RoundRock),
            ((2, 0), TileType::RoundRock),
            ((3, 0), TileType::RoundRock),
            ((4, 0), TileType::Empty),
            ((5, 0), TileType::CubeRock),
            ((6, 0), TileType::Empty),
            ((7, 0), TileType::RoundRock),
            ((8, 0), TileType::Empty),
            ((9, 0), TileType::Empty),
            ((0, 1), TileType::RoundRock),
            ((1, 1), TileType::RoundRock),
            ((2, 1), TileType::Empty),
            ((3, 1), TileType::Empty),
            ((4, 1), TileType::CubeRock),
            ((5, 1), TileType::Empty),
            ((6, 1), TileType::Empty),
            ((7, 1), TileType::Empty),
            ((8, 1), TileType::Empty),
            ((9, 1), TileType::CubeRock),
            ((0, 2), TileType::RoundRock),
            ((1, 2), TileType::RoundRock),
            ((2, 2), TileType::Empty),
            ((3, 2), TileType::Empty),
            ((4, 2), TileType::RoundRock),
            ((5, 2), TileType::CubeRock),
            ((6, 2), TileType::CubeRock),
            ((7, 2), TileType::Empty),
            ((8, 2), TileType::Empty),
            ((9, 2), TileType::RoundRock),
        ];

        let expected = expected_values.into_iter().collect::<BTreeMap<_, _>>();

        assert_eq!(platform.grid, expected);
        assert_eq!(platform.rows, 3);
        assert_eq!(platform.cols, 10);
    }

    #[test]
    fn tilt_platform() {
        let input = "OO.#.O...
O..#....#
OO.O##O.O";

        let mut platform = Platform::parse(input);
        platform.tilt_platform(TiltDirection::North);

        let expected_input = "OO.#.OO..
OO.#....#
O..O##..O";

        let expected = Platform::parse(expected_input);

        assert_eq!(platform, expected);
    }

    #[test]
    fn total_load() {
        let input = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";

        let mut platform = Platform::parse(input);
        platform.tilt_platform(TiltDirection::North);
        let total_load = platform.total_load();

        assert_eq!(total_load, 136);
    }

    #[test]
    fn test_cycle() {
        let input = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";
        let mut platform = Platform::parse(input);
        platform.cycle();
        let expected = ".....#....
....#...O#
...OO##...
.OO#......
.....OOO#.
.O#...O#.#
....O#....
......OOOO
#...O###..
#..OO#....";

        let expected = Platform::parse(expected);

        assert_eq!(platform.grid, expected.grid);

        platform.cycle();

        let expected = ".....#....
....#...O#
.....##...
..O#......
.....OOO#.
.O#...O#.#
....O#...O
.......OOO
#..OO###..
#.OOO#...O";
        let expected = Platform::parse(expected);

        assert_eq!(platform.grid, expected.grid);

        platform.cycle();

        let expected = ".....#....
....#...O#
.....##...
..O#......
.....OOO#.
.O#...O#.#
....O#...O
.......OOO
#...O###.O
#.OOO#...O";
        let expected = Platform::parse(expected);

        assert_eq!(platform.grid, expected.grid);
    }

    #[test]
    fn test_resolve_puzzle_2() {
        let input = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";

        let mut platform = Platform::parse(input);

        for _ in 0..3 {
            platform.cycle();
        }

        assert_eq!(platform.total_load(), 64);
    }
}
