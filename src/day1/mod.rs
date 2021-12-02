pub fn run() {
    part1();
    part2();
}

pub fn part1() {
    let input = read_input();
    let total = compare_depths(&input);

    println!("Day\t1\tPart\t1: {}", total);
}

pub fn part2() {
    let input = read_input();

    let a = input.split_at(0).1.chunks(3);
    let b = input.split_at(1).1.chunks(3);
    let c = input.split_at(2).1.chunks(3);
    let chain = a
        .zip(b)
        .zip(c)
        .map(|((x, y), z)| [x, y, z])
        .fold(vec![], |mut acc, chunk| {
            for item in chunk {
                acc.push(item)
            }

            acc
        });
    let chain = chain.iter().take_while(|x| x.len() == 3).map(|window| {
        window.iter().sum()
    }).collect::<Vec<i32>>();

    let total = compare_depths(&chain);
    println!("Day\t1\tPart\t2: {}", total);
}

fn compare_depths(input: &Vec<i32>) -> i32 {
    input
        .iter()
        .fold((0, i32::MAX), |(total, value), item| {
            let number = *item;
            let total = total + if value < *item { 1 } else { 0 };

            (total, number)
        })
        .0
}

fn read_input<'a>() -> Vec<i32> {
    std::fs::read_to_string("./src/day1/input.txt")
        .unwrap()
        .trim()
        .split("\n")
        .map(|str| i32::from_str_radix(&str, 10).unwrap())
        .collect::<Vec<i32>>()
}
