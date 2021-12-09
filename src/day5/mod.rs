pub fn run() {
    part1();
    part2();
}

pub fn part1() {
    let input = read_input();
    let paths = input
        .iter()
        .filter(|v| v.is_line())
        .map(|vent| vent.path())
        .flatten()
        .collect::<Vec<(i32, i32)>>();

    let mut heatmap: std::collections::HashMap<(i32, i32), i32> = std::collections::HashMap::new();
    for path in paths {
        let counter = heatmap.entry(path).or_insert(0);
        *counter += 1;
    }

    let mut total = 0;
    for (_, counter) in heatmap.iter() {
        if *counter >= 2 {
            total += 1;
        }
    }

    println!("Day\t5\tPart\t1: {}", total);
}

pub fn part2() {
    let input = read_input();
    let paths = input
        .iter()
        .map(|vent| vent.path())
        .flatten()
        .collect::<Vec<(i32, i32)>>();

    let mut heatmap: std::collections::HashMap<(i32, i32), i32> = std::collections::HashMap::new();
    for path in paths {
        let counter = heatmap.entry(path).or_insert(0);
        *counter += 1;
    }

    let mut total = 0;
    for (_, counter) in heatmap.iter() {
        if *counter >= 2 {
            total += 1;
        }
    }

    println!("Day\t5\tPart\t2: {}", total);
}

fn read_input<'a>() -> Vec<Vent> {
    std::fs::read_to_string("./src/day5/input.txt")
        .unwrap()
        .trim()
        .split("\n")
        .map(|str| {
            let mut split = str.split("->");
            let beginning = split.next().unwrap();
            let ending = split.next().unwrap();

            let mut x1y1 = beginning
                .split(",")
                .map(|str| i32::from_str_radix(str.trim(), 10).unwrap());
            let x1 = x1y1.next().unwrap();
            let y1 = x1y1.next().unwrap();

            let mut x2y2 = ending
                .split(",")
                .map(|str| i32::from_str_radix(str.trim(), 10).unwrap());
            let x2 = x2y2.next().unwrap();
            let y2 = x2y2.next().unwrap();

            Vent {
                x1: x1,
                x2: x2,
                y1: y1,
                y2: y2,
            }
        })
        .collect::<Vec<Vent>>()
}

#[derive(Debug)]
struct Vent {
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32,
}

impl Vent {
    fn path(&self) -> Vec<(i32, i32)> {
        let mut path = vec![];

        if self.x1 == self.x2 {
            // vertical
            let (start, stop) = if self.y1 < self.y2 { (self.y1, self.y2) } else { (self.y2, self.y1) };

            for y in start..=stop {
                path.push((self.x1, y));
            }
        } else if self.y1 == self.y2 {
            // horizontal
            let (start, stop) = if self.x1 < self.x2 { (self.x1, self.x2) } else { (self.x2, self.x1) };

            for x in start..=stop {
                path.push((x, self.y1));
            }
        } else {
            // diagonal
            let dec_x = self.x1 > self.x2;
            let dec_y = self.y1 > self.y2;

            let range = (self.x1 - self.x2).abs();
            for inc in 0..=range {
                    match (dec_x, dec_y) {
                        (true, true) => path.push((self.x1 - inc, self.y1 - inc)),
                        (true, false) => path.push((self.x1 - inc, self.y1 + inc)),
                        (false, true) => path.push((self.x1 + inc, self.y1 - inc)),
                        (false, false) => path.push((self.x1 + inc, self.y1 + inc))
                }
            }
        }

        path
    }

    fn is_line(&self) -> bool {
        self.x1 == self.x2 || self.y1 == self.y2
    }
}
