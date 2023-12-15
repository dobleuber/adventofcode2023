use crate::utils::open_file;
use queues::{IsQueue, Queue};
use std::{collections::BTreeMap, usize};

pub fn day10() -> Result<usize, std::io::Error> {
    let contents = open_file("./inputs/10/input.txt")?;

    let result = resolve_puzzle(&contents);

    Ok(result)
}

fn resolve_puzzle(input: &str) -> usize {
    let mut maze = PipeMaze::parse(input);
    maze.connect_pipes();

    // println!("loop one: {:?}", maze.pipe_loop);
    maze.pipe_loop.len() / 2
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

#[derive(Debug, PartialEq, Clone)]
struct PipeMaze {
    pipes: BTreeMap<(usize, usize), Pipe>,
    start: (usize, usize),
    pipe_loop: Vec<(usize, usize)>,
}

#[derive(Debug, Clone, PartialEq)]
struct Pipe {
    symbol: String,
    code: PipeType,
    is_connected: bool,
    visited: bool,
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
            .collect();

        Self {
            pipes,
            start,
            pipe_loop: vec![start],
        }
    }

    fn connect_pipes(&mut self) {
        let mut current = self.start;
        loop {
            let mut pipe = self.pipes.get_mut(&current).unwrap().clone();

            match pipe.code {
                PipeType::Start => {
                    if let Some(next_pipe) =
                        pipe.clone()
                            .find_candidates(current)
                            .iter()
                            .find(|candidate| {
                                let candidate_pipe = self.pipes.get(candidate);
                                if let Some(candidate_pipe) = candidate_pipe {
                                    match (
                                        current.0.checked_sub(candidate.0),
                                        current.1.checked_sub(candidate.1),
                                    ) {
                                        (Some(0), _) => {
                                            pipe.is_connected = true;
                                            pipe.visited = true;
                                            candidate_pipe.code == PipeType::Horizontal
                                                || candidate_pipe.code == PipeType::NorthWest
                                                || candidate_pipe.code == PipeType::NorthEast
                                        }
                                        (_, Some(0)) => {
                                            pipe.is_connected = true;
                                            pipe.visited = true;
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
                        self.pipe_loop.push(current);
                    }
                }
                PipeType::Ground => {
                    panic!("Ground found");
                }
                _ => {
                    if let Some(next_pipe) = pipe
                        .find_candidates(current)
                        .iter()
                        .find(|candidate| !self.pipe_loop.contains(candidate))
                    {
                        current = *next_pipe;
                        self.pipe_loop.push(current);
                    } else {
                        break;
                    }
                }
            }

            if current == self.start {
                break;
            }
        }
    }

    fn look_external(&mut self) {
        self.connect_pipes();
        let current = self.start;
        let mut queue: Queue<(usize, usize)> = Queue::new();
        let current_pipe = self.pipes.get_mut(&current).unwrap();

        current_pipe.visited = true;
        queue.add(current).unwrap();

        while queue.size() > 0 {
            let current = queue.remove().unwrap();
            let current_pipe = self.pipes.get_mut(&current).unwrap().clone();

            let candidates: Vec<_> = current_pipe.find_candidates(current);
            for candidate in candidates.iter() {
                let candidate_pipe = self.pipes.get_mut(candidate).unwrap();
                if !candidate_pipe.visited && !candidate_pipe.is_connected {
                    candidate_pipe.visited = true;
                    queue.add(*candidate).unwrap();
                }
            }
        }
    }

    fn enclosed_tiles(&mut self) -> usize {
        self.look_external();
        dbg!(self.pipes.clone());
        self.pipes.iter().filter(|(_, pipe)| !pipe.visited).count()
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
        Self {
            code,
            symbol,
            is_connected: false,
            visited: true,
        }
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
            _ => unreachable!(),
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
            ((0, 0), Pipe::parse('.')),
            ((0, 1), Pipe::parse('.')),
            ((0, 2), Pipe::parse('.')),
            ((0, 3), Pipe::parse('.')),
            ((0, 4), Pipe::parse('.')),
            ((1, 0), Pipe::parse('.')),
            ((1, 1), Pipe::parse('S')),
            ((1, 2), Pipe::parse('-')),
            ((1, 3), Pipe::parse('7')),
            ((1, 4), Pipe::parse('.')),
            ((2, 0), Pipe::parse('.')),
            ((2, 1), Pipe::parse('|')),
            ((2, 2), Pipe::parse('.')),
            ((2, 3), Pipe::parse('|')),
            ((2, 4), Pipe::parse('.')),
            ((3, 0), Pipe::parse('.')),
            ((3, 1), Pipe::parse('L')),
            ((3, 2), Pipe::parse('-')),
            ((3, 3), Pipe::parse('J')),
            ((3, 4), Pipe::parse('.')),
            ((4, 0), Pipe::parse('.')),
            ((4, 1), Pipe::parse('.')),
            ((4, 2), Pipe::parse('.')),
            ((4, 3), Pipe::parse('.')),
            ((4, 4), Pipe::parse('.')),
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

    // #[test]
    fn enclosed_tiles_1() {
        let input = "...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........";

        let mut result = PipeMaze::parse(input);

        assert_eq!(result.enclosed_tiles(), 4);
    }

    // #[test]
    fn enclosed_tiles_2() {
        let input = ".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...";
        let mut result = PipeMaze::parse(input);

        assert_eq!(result.enclosed_tiles(), 8);
    }

    // #[test]
    fn enclosed_tiles_3() {
        let input = "FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L";

        let mut result = PipeMaze::parse(input);

        assert_eq!(result.enclosed_tiles(), 10);
    }
}
