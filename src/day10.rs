use crate::utils::open_file;
use std::{collections::BTreeMap, usize};

pub fn day10() -> Result<usize, std::io::Error> {
    let contents = open_file("./inputs/10/input.txt")?;

    let result = resolve_puzzle(&contents);

    Ok(result)
}

fn resolve_puzzle(input: &str) -> usize {
    let mut maze = PipeMaze::parse(input);
    maze.connect_pipes();

    println!("loop one: {:?}", maze.loop_one);
    maze.loop_one.len() / 2
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum PipeType {
    Start,
    Horizontal,
    Vertical,
    NorthEast,
    NorthWest,
    SouthEast,
    SouthWest,
    Ground,
}

#[derive(Debug, PartialEq)]
struct PipeMaze {
    pipes: BTreeMap<(usize, usize), Pipe>,
    start: (usize, usize),
    loop_one: Vec<(usize, usize)>,
    loop_two: Vec<(usize, usize)>,
}

#[derive(Debug, Clone, PartialEq)]
struct Pipe {
    symbol: String,
    code: PipeType,
}

impl PipeMaze {
    fn parse(input: &str) -> Self {
        let mut start = (0, 0);
        let pipes: BTreeMap<(usize, usize), Pipe> = input
            .lines()
            .enumerate()
            .flat_map(|(x, line)| {
                line.chars()
                    .enumerate()
                    .map(|(y, char)| {
                        if char == 'S' {
                            start = (x, y);
                        }
                        ((x, y), Pipe::parse(char))
                    })
                    .collect::<Vec<_>>()
            })
            .filter(|(_, pipe)| pipe.code != PipeType::Ground)
            .collect();

        Self {
            pipes,
            start,
            loop_one: vec![start],
            loop_two: vec![start],
        }
    }

    fn connect_pipes(&mut self) {
        let mut current = self.start;
        let mut index = 0;
        loop {
            let pipe = self.pipes.get(&current).unwrap().clone();
            println!("pipe: {:?}-{:?}", current, pipe);

            match pipe.code {
                PipeType::Start => {
                    if let Some(next_pipe) =
                        pipe.find_candidates(current).iter().find(|candidate| {
                            let candidate_pipe = self.pipes.get(candidate);
                            if let Some(candidate_pipe) = candidate_pipe {
                                match (
                                    current.0.checked_sub(candidate.0),
                                    current.1.checked_sub(candidate.1),
                                ) {
                                    (Some(0), _) => {
                                        candidate_pipe.code == PipeType::Horizontal
                                            || candidate_pipe.code == PipeType::NorthWest
                                            || candidate_pipe.code == PipeType::NorthEast
                                    }
                                    (_, Some(0)) => {
                                        candidate_pipe.code == PipeType::Vertical
                                            || candidate_pipe.code == PipeType::SouthWest
                                            || candidate_pipe.code == PipeType::SouthEast
                                    }
                                    _ => false,
                                }
                            } else {
                                false
                            }
                        })
                    {
                        current = *next_pipe;
                        self.loop_one.push(current);
                    }
                }
                PipeType::Ground => {
                    panic!("Ground found");
                }
                _ => {
                    if let Some(next_pipe) = pipe
                        .find_candidates(current)
                        .iter()
                        .find(|candidate| !self.loop_one.contains(candidate))
                    {
                        current = *next_pipe;
                        self.loop_one.push(current);
                    } else {
                        break;
                    }
                }
            }

            if current == self.start {
                break;
            }

            index += 1;

            if index > 10 {
                break;
            }
        }
    }
}

impl Pipe {
    fn parse(input: char) -> Self {
        let (symbol, code) = match input {
            '-' => ("↔".to_string(), PipeType::Horizontal),
            '|' => ("↕️".to_string(), PipeType::Vertical),
            'L' => ("↳".to_string(), PipeType::NorthEast),
            'J' => ("↲".to_string(), PipeType::NorthWest),
            '7' => ("↰".to_string(), PipeType::SouthWest),
            'F' => ("↱".to_string(), PipeType::SouthEast),
            'S' => ("🏁".to_string(), PipeType::Start),
            _ => ("🚫".to_string(), PipeType::Ground),
        };
        Self { code, symbol }
    }

    fn find_candidates(self, current: (usize, usize)) -> Vec<(usize, usize)> {
        match self.code {
            PipeType::Start => [(-1, 0), (0, 1), (1, 0), (0, -1)]
                .iter()
                .map(|(x, y)| (current.0 as i64 + x, current.1 as i64 + y))
                .filter(|(x, y)| *x >= 0 && *y >= 0)
                .map(|(x, y)| (x as usize, y as usize))
                .collect(),
            PipeType::Horizontal => [(0, -1), (0, 1)]
                .iter()
                .map(|(x, y)| (current.0 as i64 + x, current.1 as i64 + y))
                .filter(|(x, y)| *x >= 0 && *y >= 0)
                .map(|(x, y)| (x as usize, y as usize))
                .collect(),
            PipeType::Vertical => [(-1, 0), (1, 0)]
                .iter()
                .map(|(x, y)| (current.0 as i64 + x, current.1 as i64 + y))
                .filter(|(x, y)| *x >= 0 && *y >= 0)
                .map(|(x, y)| (x as usize, y as usize))
                .collect(),
            PipeType::NorthEast => [(-1, 0), (0, 1)]
                .iter()
                .map(|(x, y)| (current.0 as i64 + x, current.1 as i64 + y))
                .filter(|(x, y)| *x >= 0 && *y >= 0)
                .map(|(x, y)| (x as usize, y as usize))
                .collect(),
            PipeType::NorthWest => [(-1, 0), (0, -1)]
                .iter()
                .map(|(x, y)| (current.0 as i64 + x, current.1 as i64 + y))
                .filter(|(x, y)| *x >= 0 && *y >= 0)
                .map(|(x, y)| (x as usize, y as usize))
                .collect(),
            PipeType::SouthEast => [(1, 0), (0, 1)]
                .iter()
                .map(|(x, y)| (current.0 as i64 + x, current.1 as i64 + y))
                .filter(|(x, y)| *x >= 0 && *y >= 0)
                .map(|(x, y)| (x as usize, y as usize))
                .collect(),
            PipeType::SouthWest => [(1, 0), (0, -1)]
                .iter()
                .map(|(x, y)| (current.0 as i64 + x, current.1 as i64 + y))
                .filter(|(x, y)| *x >= 0 && *y >= 0)
                .map(|(x, y)| (x as usize, y as usize))
                .collect(),
            _ => todo!(),
        }
    }
}

mod tests {
    use super::*;

    #[test]
    fn test_resolve_puzzle() {
        let input = ".....
.S-7.
.|.|.
.L-J.
.....";

        let result = resolve_puzzle(input);

        assert_eq!(result, 4);
    }

    fn test_resolve_puzzle_2() {
        let input = "..F7.
.FJ|.
SJ.L7
|F--J
LJ...";

        let result = resolve_puzzle(input);

        assert_eq!(result, 8);
    }

    #[test]
    fn parse_input_1() {
        let input = ".....
.S-7.
.|.|.
.L-J.
.....";

        let result = PipeMaze::parse(input);

        let values: Vec<_> = vec![
            ((1, 1), Pipe::parse('S')),
            ((1, 2), Pipe::parse('-')),
            ((1, 3), Pipe::parse('7')),
            ((2, 1), Pipe::parse('|')),
            ((2, 3), Pipe::parse('|')),
            ((3, 1), Pipe::parse('L')),
            ((3, 2), Pipe::parse('-')),
            ((3, 3), Pipe::parse('J')),
        ];

        let expected = values.into_iter().collect::<BTreeMap<_, _>>();

        assert_eq!(result.pipes, expected);
        assert_eq!(result.start, (1, 1));
    }

    #[test]
    fn start_candidates() {
        let start_pipe = Pipe::parse('S');
        let result = start_pipe.find_candidates((1, 1));

        let expected = vec![(0, 1), (1, 2), (2, 1), (1, 0)];

        assert_eq!(result, expected);
    }

    #[test]
    fn start_candidates_at_corner() {
        let start_pipe = Pipe::parse('S');
        let result = start_pipe.find_candidates((0, 0));

        let expected = vec![(0, 1), (1, 0)];

        assert_eq!(result, expected);
    }

    #[test]
    fn horizontal_candidates() {
        let start_pipe = Pipe::parse('-');
        let result = start_pipe.find_candidates((1, 1));

        let expected = vec![(1, 0), (1, 2)];

        assert_eq!(result, expected);
    }

    #[test]
    fn vertical_candidates() {
        let start_pipe = Pipe::parse('|');
        let result = start_pipe.find_candidates((1, 1));

        let expected = vec![(0, 1), (2, 1)];

        assert_eq!(result, expected);
    }

    #[test]
    fn north_east_candidates() {
        let start_pipe = Pipe::parse('L');
        let result = start_pipe.find_candidates((1, 1));

        let expected = vec![(0, 1), (1, 2)];

        assert_eq!(result, expected);
    }

    #[test]
    fn north_west_candidates() {
        let start_pipe = Pipe::parse('J');
        let result = start_pipe.find_candidates((1, 1));

        let expected = vec![(0, 1), (1, 0)];

        assert_eq!(result, expected);
    }
}
