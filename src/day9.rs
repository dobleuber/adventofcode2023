use crate::utils::open_file;

pub fn day9() -> Result<i64, std::io::Error> {
    let contents = open_file("./inputs/9/input.txt")?;

    let result = resolve_puzzle_2(&contents);

    Ok(result)
}

fn resolve_puzzle(input: &str) -> i64 {
    let mut readings = input.lines().map(Reading::parse).collect::<Vec<_>>();
    let next_readings = readings
        .iter_mut()
        .map(|reading| {
            reading.calc_diffs();
            reading.next()
        })
        // .inspect(|next_reading| println!("next reading: {}", next_reading))
        .collect::<Vec<_>>();

    next_readings.iter().sum()
}

fn resolve_puzzle_2(input: &str) -> i64 {
    let mut readings = input.lines().map(Reading::parse).collect::<Vec<_>>();
    let prev_readings = readings
        .iter_mut()
        .map(|reading| {
            reading.calc_diffs();
            reading.prev()
        })
        // .inspect(|prev_reading| println!("result: {}", prev_reading))
        .collect::<Vec<_>>();

    prev_readings.iter().sum()
}

struct Reading {
    values: Vec<i64>,
    diffs: Vec<Vec<i64>>,
}

impl Reading {
    fn parse(input: &str) -> Self {
        let values = input
            .split_ascii_whitespace()
            .map(|s| s.parse::<i64>().unwrap())
            .collect();

        Self {
            values,
            diffs: vec![],
        }
    }

    fn calc_diffs(&mut self) {
        let mut values = self.values.clone();
        let mut diffs = vec![values.clone()];

        loop {
            let diff_row: Vec<i64> = values
                .iter()
                .zip(values.iter().skip(1))
                .map(|(a, b)| b - a)
                .collect();

            diffs.push(diff_row.clone());

            if diff_row.iter().all(|&d| d == 0) {
                break;
            }

            values = diff_row;
        }

        self.diffs = diffs;
    }

    fn next(&self) -> i64 {
        let next_values: i64 = self.diffs.iter().map(|diff| diff.last().unwrap()).sum();
        next_values
    }

    fn prev(&self) -> i64 {
        let prev_values: i64 = self
            .diffs
            .iter()
            .rev()
            .map(|diff| diff.first().unwrap())
            .fold(0, |acc, v| v - acc);
        prev_values
    }
}

mod tests {
    #[test]
    fn parse() {
        let line = "0 3 6 9 12 15";

        let reading = super::Reading::parse(line);

        assert_eq!(reading.values, vec![0, 3, 6, 9, 12, 15]);
    }

    #[test]
    fn diffs_1() {
        let line = "0 3 6 9 12 15";

        let mut reading = super::Reading::parse(line);

        reading.calc_diffs();

        assert_eq!(
            reading.diffs,
            vec![
                vec![0, 3, 6, 9, 12, 15],
                vec![3, 3, 3, 3, 3],
                vec![0, 0, 0, 0]
            ]
        );
    }

    #[test]
    fn diffs_2() {
        let line = "1 3 6 10 15 21";

        let mut reading = super::Reading::parse(line);

        reading.calc_diffs();

        assert_eq!(
            reading.diffs,
            vec![
                vec![1, 3, 6, 10, 15, 21],
                vec![2, 3, 4, 5, 6],
                vec![1, 1, 1, 1],
                vec![0, 0, 0]
            ]
        );
    }

    #[test]
    fn next_reading_1() {
        let line = "0 3 6 9 12 15";

        let mut reading = super::Reading::parse(line);
        reading.calc_diffs();

        assert_eq!(reading.next(), 18);
    }

    #[test]
    fn next_reading_2() {
        let line = "1 3 6 10 15 21";

        let mut reading = super::Reading::parse(line);
        reading.calc_diffs();

        assert_eq!(reading.next(), 28);
    }

    #[test]
    fn resolve_next() {
        let contents = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";

        let result = super::resolve_puzzle(contents);

        assert_eq!(result, 114);
    }

    #[test]
    fn prev_reading_1() {
        let line = "0 3 6 9 12 15";

        let mut reading = super::Reading::parse(line);
        reading.calc_diffs();

        assert_eq!(reading.prev(), -3);
    }

    #[test]
    fn prev_reading_2() {
        let line = "1 3 6 10 15 21";

        let mut reading = super::Reading::parse(line);
        reading.calc_diffs();

        assert_eq!(reading.prev(), 0);
    }

    #[test]
    fn prev_reading_3() {
        let line = "10 13 16 21 30 45";

        let mut reading = super::Reading::parse(line);
        reading.calc_diffs();

        assert_eq!(reading.prev(), 5);
    }

    #[test]
    fn resolve_prev() {
        let contents = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";

        let result = super::resolve_puzzle_2(contents);

        assert_eq!(result, 2);
    }

    #[test]
    fn prev_reading_4() {
        let line = "10 19 48 117 252 484 864 1510 2714 5159 10327 21218 43546 87630 171255 323839 592306 1049131 1803090 3013315 4907320";

        let mut reading = super::Reading::parse(line);
        reading.calc_diffs();
        println!("{:?}", reading.diffs);

        assert_eq!(reading.prev(), 9);
    }
}
