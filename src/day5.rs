use crate::utils::open_file;
use core::str::Lines;
use std::ops::Range;

#[derive(Debug)]
struct Field {
    // (index, value)
    ranges: Range<u32>,
}

impl Field {
    fn shift(&mut self, shift: &Shift) {
        let (destination, source, len) = (shift.0, shift.1, shift.2);
    }
}

#[derive(Debug)]
struct Shift(u32, u32, u32);

impl Shift {
    fn parse(line: &str) -> Option<Self> {
        if line.is_empty() {
            return None;
        }

        let mut line_iter = line.split_ascii_whitespace();
        let destination = line_iter.next().unwrap().parse::<u32>().unwrap();
        let source = line_iter.next().unwrap().parse::<u32>().unwrap();
        let count = line_iter.next().unwrap().parse::<u32>().unwrap();
        Some(Self(destination, source, count))
    }
}

#[derive(Debug)]
struct ShiftGroup {
    shifts: Vec<Shift>,
}

impl ShiftGroup {
    fn parse(lines: &mut Lines) -> Self {
        let mut lines = lines.peekable();
        while let Some(line) = lines.peek() {
            if line.contains("map:") {
                lines.next();
                break;
            }
            lines.next();
        }

        let shifts: Vec<_> = lines
            .map(Shift::parse)
            .take_while(|shift| shift.is_some())
            .map(|shift| shift.unwrap())
            .collect();
        Self { shifts }
    }
}
