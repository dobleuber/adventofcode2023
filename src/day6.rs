use crate::utils::open_file;

pub fn day6_part2() -> Result<u32, std::io::Error> {
    let contents = open_file("./inputs/6/input.txt")?;
    let times: Vec<_> = contents
        .lines()
        .next()
        .unwrap()
        .split_ascii_whitespace()
        .skip(1)
        .collect();
    let distances: Vec<_> = contents
        .lines()
        .last()
        .unwrap()
        .split_ascii_whitespace()
        .skip(1)
        .collect();

    let total_time = times.join("").parse::<u64>().unwrap();
    let total_distance = distances.join("").parse::<u64>().unwrap();

    println!("{:?}", total_time);
    println!("{:?}", total_distance);

    let ways = (1..total_time)
        .map(|i| (total_time - i) * i)
        .filter(|&md| md > total_distance)
        .collect::<Vec<_>>();

    Ok(ways.len() as u32)
}

pub fn day6_part1() -> Result<u32, std::io::Error> {
    let contents = open_file("./inputs/6/input.txt")?;
    let times: Vec<_> = contents
        .lines()
        .next()
        .unwrap()
        .split_ascii_whitespace()
        .skip(1)
        .map(|time| time.parse::<u32>().unwrap())
        .collect();
    let distances: Vec<_> = contents
        .lines()
        .last()
        .unwrap()
        .split_ascii_whitespace()
        .skip(1)
        .map(|time| time.parse::<u32>().unwrap())
        .collect();

    let ways: Vec<_> = times
        .iter()
        .zip(distances.iter())
        .map(|(&t, &d)| {
            (1..t)
                .map(|i| (t - i) * i)
                .filter(|&md| md > d)
                .collect::<Vec<_>>()
        })
        .map(|w| w.len())
        .collect();

    let result = ways.iter().map(|&w| w as u32).product::<u32>();

    Ok(result)
}
