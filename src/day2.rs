use crate::utils::open_file;

pub fn day2() -> Result<u32, std::io::Error> {
    let contents = open_file("./inputs/2/input.txt")?;

    // Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
    // Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
    let result: Vec<_> = contents
        .lines()
        .map(|line| {
            let mut line_iter = line.split(':');
            let _game = line_iter
                .next()
                .unwrap()
                .split_ascii_whitespace()
                .last()
                .unwrap();
            let balls = line_iter
                .next()
                .unwrap()
                .split(';')
                .fold([0, 0, 0], |mut acc, set| {
                    set.split(',').for_each(|ball| {
                        let mut ball_iter = ball.trim().split_ascii_whitespace();
                        let number = ball_iter.next().unwrap().parse::<u32>().unwrap();
                        let color = ball_iter.next().unwrap();
                        match color {
                            "red" if acc[0] < number => acc[0] = number,
                            "green" if acc[1] < number => acc[1] = number,
                            "blue" if acc[2] < number => acc[2] = number,
                            _ => (),
                        }
                    });
                    acc
                });
            balls
        })
        .inspect(|balls| println!("{:?}", balls))
        .map(|balls| balls.iter().product())
        .inspect(|product| println!("{}", product))
        .collect();

    println!("{:?}", result.iter().sum::<u32>());

    let result = result.iter().sum::<u32>();

    Ok(result)
}
