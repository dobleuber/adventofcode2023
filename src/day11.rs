use std::collections::BTreeMap;

use crate::utils::open_file;

pub fn day11() -> Result<usize, std::io::Error> {
    let contents = open_file("./inputs/11/input.txt")?;
    let result = resolve_puzzle(&contents);

    Ok(result)
}

fn resolve_puzzle(input: &str) -> usize {
    let mut universe = Universe::parse(input);
    universe.expand();

    universe.get_all_distances()
}

struct Universe {
    grid: Vec<(usize, usize)>,
}

impl Universe {
    fn parse(input: &str) -> Self {
        let grid = input
            .lines()
            .enumerate()
            .flat_map(|(x, line)| {
                line.chars().enumerate().filter_map(move |(y, c)| match c {
                    '#' => Some((x, y)),
                    '.' => None,
                    _ => panic!("invalid char"),
                })
            })
            .collect();

        Self { grid }
    }

    fn expand(&mut self) {
        let (mut rows, mut cols): (Vec<_>, Vec<_>) = self.grid.iter().cloned().unzip();
        rows.sort();
        rows.dedup();
        cols.sort();
        cols.dedup();
        let expandded_rows = expand_galaxy_indices(&rows);
        let expandded_cols = expand_galaxy_indices(&cols);
        let grid: Vec<_> = self
            .grid
            .iter()
            .map(|&(x, y)| (expandded_rows[&x], expandded_cols[&y]))
            .collect();

        self.grid = grid;
    }

    fn get_all_distances(&self) -> usize {
        let grid = self.grid.clone();

        let distances: Vec<_> = grid
            .iter()
            .enumerate()
            .flat_map(|(index, (x1, y1))| {
                grid.iter().skip(index + 1).map(move |(x2, y2)| {
                    let x = x1.abs_diff(*x2);
                    let y = y1.abs_diff(*y2);

                    x + y
                })
            })
            .collect();

        distances.iter().sum()
    }
}

fn expand_galaxy_indices(galaxy_rows: &[usize]) -> BTreeMap<usize, usize> {
    let first = *galaxy_rows.first().unwrap();
    let mut expanded_indices = BTreeMap::from([(first, first)]);
    let mut additional_gap = 0;
    let values = galaxy_rows;

    values
        .iter()
        .zip(values.iter().skip(1))
        .for_each(|(&c, &n)| {
            match n - c {
                2 => {
                    additional_gap += 1;
                }
                3.. => (c..n).for_each(|_| {
                    additional_gap += 1;
                }),
                _ => (),
            }
            expanded_indices.insert(n, n + additional_gap);
        });

    expanded_indices
}

mod tests {
    use super::*;

    #[test]
    fn parse_galaxy() {
        let input = "...#...
....#..
.......
#......";

        let universe = Universe::parse(input);

        let expected = vec![(0, 3), (1, 4), (3, 0)];

        assert_eq!(universe.grid, expected);
    }

    #[test]
    fn expand_galaxy() {
        let input = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";

        let mut universe = Universe::parse(input);
        universe.expand();

        let expected = vec![
            (0, 4),
            (1, 9),
            (2, 0),
            (5, 8),
            (6, 1),
            (7, 12),
            (10, 9),
            (11, 0),
            (11, 5),
        ];

        assert_eq!(universe.grid, expected);
    }

    #[test]
    fn expand_galaxy_indices_test() {
        let galaxy_rows = vec![0, 2, 5, 6, 9];

        let expanded_indices = expand_galaxy_indices(&galaxy_rows);

        let expected = BTreeMap::from([(0, 0), (2, 3), (5, 9), (6, 10), (9, 16)]);

        assert_eq!(expanded_indices, expected);
    }

    #[test]
    fn get_all_distances() {
        let input = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";

        let mut universe = Universe::parse(input);
        universe.expand();

        let distances = universe.get_all_distances();

        assert_eq!(distances, 0);
    }
}
