use crate::utils::open_file;

pub fn day3() -> Result<u32, std::io::Error> {
    let contents = open_file("./inputs/3/input.txt")?;

    // convert to matrix
    let schematic_matrix: Vec<_> = contents
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect();

    let mut schematic_numbers: Vec<u32> = vec![];
    let mut i = 0;

    while i < schematic_matrix.len() {
        let line = schematic_matrix.get(i).unwrap();
        let mut j = 0;
        while j < line.len() {
            let char = line.get(j).unwrap();
            if char.is_ascii_digit() && number_is_valid(&schematic_matrix, i, j) {
                let whole_number = find_whole_number(&schematic_matrix, i, &mut j);
                schematic_numbers.push(whole_number);
            }
            j += 1;
        }
        i += 1;
    }

    println!("{:?}", schematic_numbers);

    Ok(schematic_numbers.iter().sum::<u32>())
}

pub fn day3_part2() -> Result<u32, std::io::Error> {
    let contents = open_file("./inputs/3/input.txt")?;

    // convert to matrix
    let schematic_matrix: Vec<_> = contents
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect();

    let mut schematic_numbers: Vec<u32> = vec![];
    let mut i = 0;

    while i < schematic_matrix.len() {
        let line = schematic_matrix.get(i).unwrap();
        let mut j = 0;
        while j < line.len() {
            let char = line.get(j).unwrap();
            if is_gear(*char) {
                if let Some(number) = is_valid_gear(&schematic_matrix, i, j) {
                    schematic_numbers.push(number);
                }
            }
            j += 1;
        }
        i += 1;
    }

    println!("{:?}", schematic_numbers);

    Ok(schematic_numbers.iter().sum::<u32>())
}

fn is_gear(char: char) -> bool {
    char == '*'
}

fn is_valid_gear(schematic_matrix: &Vec<Vec<char>>, i: usize, j: usize) -> Option<u32> {
    let lower_i = i.saturating_sub(1);
    let upper_i = schematic_matrix.len().saturating_sub(1).min(i + 1);
    let lower_j = j.saturating_sub(1);
    let upper_j = schematic_matrix
        .get(i)
        .unwrap()
        .len()
        .saturating_sub(1)
        .min(j + 1);
    let mut gear_numbers: Vec<u32> = vec![];
    for i in lower_i..=upper_i {
        let mut j = lower_j;
        while j <= upper_j {
            let char = schematic_matrix.get(i).unwrap().get(j).unwrap();
            if char.is_ascii_digit() {
                let whole_number = find_whole_number(schematic_matrix, i, &mut j);
                gear_numbers.push(whole_number);
            }
            j += 1;
        }
    }

    print!("{:?} ", gear_numbers);

    if gear_numbers.len() == 2 {
        Some(gear_numbers.iter().product())
    } else {
        None
    }
}

fn number_is_valid(schematic_matrix: &Vec<Vec<char>>, i: usize, j: usize) -> bool {
    let lower_i = i.saturating_sub(1);
    let upper_i = schematic_matrix.len().saturating_sub(1).min(i + 1);
    let lower_j = j.saturating_sub(1);
    let upper_j = schematic_matrix
        .get(i)
        .unwrap()
        .len()
        .saturating_sub(1)
        .min(j + 1);
    for i in lower_i..=upper_i {
        for j in lower_j..=upper_j {
            let char = schematic_matrix.get(i).unwrap().get(j).unwrap();
            if is_symbol(*char) {
                return true;
            }
        }
    }

    false
}

fn is_symbol(char: char) -> bool {
    char != '.' && !char.is_alphanumeric() && !char.is_ascii_whitespace()
}

fn find_whole_number(schematic_matrix: &Vec<Vec<char>>, i: usize, j: &mut usize) -> u32 {
    let mut number_start_index = *j;

    let line = schematic_matrix.get(i).unwrap();

    loop {
        let temp_index = number_start_index.saturating_sub(1);
        let char = line.get(temp_index).unwrap();
        if char.is_ascii_digit() {
            number_start_index = temp_index;
        } else {
            break;
        }

        if temp_index == 0 {
            break;
        }
    }

    loop {
        let temp_index = *j + 1;
        let char = line.get(temp_index).unwrap();
        if char.is_ascii_digit() {
            *j = temp_index;
        } else {
            break;
        }

        if temp_index == line.len() - 1 {
            break;
        }
    }

    let number: String = line
        .iter()
        .skip(number_start_index)
        .take(*j - number_start_index + 1)
        .collect();

    number.parse::<u32>().unwrap()
}
