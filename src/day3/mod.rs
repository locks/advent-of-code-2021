pub fn run() {
    part1();
    part2();
}

pub fn part1() {
    let input = read_input();
    let mut gamma_rate = vec![];
    let mut epsilon_rate = vec![];

    let size = input.get(0).unwrap().len();

    for i in 0..size {
        let mut one = 0;
        let mut zero = 0;

        for number in &input {
            match number.get(i).unwrap() {
                0 => zero = zero + 1,
                1 => one = one + 1,
                _ => unreachable!("Digit should be 0 or 1"),
            }
        }
        if one > zero {
            gamma_rate.push(1);
            epsilon_rate.push(0)
        } else {
            gamma_rate.push(0);
            epsilon_rate.push(1);
        }
    }
    let gamma = gamma_rate.iter().fold(0, |acc, &b| acc * 2 + b);
    let epsilon = epsilon_rate.iter().fold(0, |acc, &b| acc * 2 + b);

    println!("Day\t3\tPart\t1: {}", gamma * epsilon);
}

pub fn part2() {
    let input = read_input();
    let mut oxygen: Vec<Vec<u32>> = input.clone();
    let mut c02_scrubber: Vec<Vec<u32>> = input.clone();

    let mut i = 0;
    let mut len = 2;
    while len > 1 {
        let mut one = 0;
        let mut zero = 0;

        for number in &oxygen {
            match number.get(i).unwrap() {
                0 => zero = zero + 1,
                1 => one = one + 1,
                _ => unreachable!("Digit should be 0 or 1"),
            }
        }

        if zero > one {
            oxygen = oxygen
                .into_iter()
                .filter(|number| *number.get(i).unwrap() == 0)
                .collect::<Vec<Vec<u32>>>();
        } else {
            oxygen = oxygen
                .into_iter()
                .filter(|number| *number.get(i).unwrap() == 1)
                .collect::<Vec<Vec<u32>>>();
        }

        i = i + 1;
        len = oxygen.len();
    }

    let mut i = 0;
    let mut len = 2;
    while len > 1 {
        let mut one = 0;
        let mut zero = 0;

        for number in &c02_scrubber {
            match number.get(i).unwrap() {
                0 => zero = zero + 1,
                1 => one = one + 1,
                _ => unreachable!("Digit should be 0 or 1"),
            }
        }

        if one >= zero {
            c02_scrubber = c02_scrubber
                .into_iter()
                .filter(|number| *number.get(i).unwrap() == 0)
                .collect::<Vec<Vec<u32>>>();
        } else {
            c02_scrubber = c02_scrubber
                .into_iter()
                .filter(|number| *number.get(i).unwrap() == 1)
                .collect::<Vec<Vec<u32>>>();
        }

        i = i + 1;
        len = c02_scrubber.len();
    }

    let gamma = oxygen.get(0).unwrap().iter().fold(0, |acc, &b| acc * 2 + b);
    let epsilon = c02_scrubber.get(0).unwrap().iter().fold(0, |acc, &b| acc * 2 + b);

    println!("Day\t3\tPart\t2: {}", gamma * epsilon);
}

fn read_input<'a>() -> Vec<Vec<u32>> {
    std::fs::read_to_string("./src/day3/input.txt")
        .unwrap()
        .trim()
        .split("\n")
        .map(|str| {
            str.chars()
                .map(|digit| digit.to_digit(10).unwrap())
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>()
}
