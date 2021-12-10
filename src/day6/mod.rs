pub fn run() {
    part1();
    part2();
}

type Population = [usize; 9];

pub fn part1() {
    let mut seed = read_input();
    let total = calculate_population(seed, 80);

    println!("Day\t6\tPart\t1: {}", total);
}

pub fn part2() {
    let mut seed = read_input();
    let total = calculate_population(seed, 256);

    println!("Day\t6\tPart\t2: {}", total);
}

fn calculate_population(seed: Vec<i32>, days: i32) -> i64 {
    let mut heatmap: Population = [0; 9];

    for life in seed {
        heatmap[life as usize] += 1;
    }

    for _ in 0..days {
        heatmap = tick(&heatmap);
    }
    
    heatmap.iter().fold(0, |acc, &v| acc + (v as i64))
}

fn tick(heatmap: &Population) -> Population {
    let mut next: Population = [0; 9];

    for i in 1..heatmap.len() {
        next[i - 1] = heatmap[i];
    }

    next[6] += heatmap[0];
    next[8] = heatmap[0];

    next
}

fn read_input<'a>() -> Vec<i32> {
    std::fs::read_to_string("./src/day6/input.txt")
        .unwrap()
        .trim()
        .split(",")
        .map(|str| i32::from_str_radix(&str, 10).unwrap())
        .collect::<Vec<i32>>()
}
