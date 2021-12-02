struct Position {
    x: i32,
    y: i32
}

pub fn part1() {
    let input = read_input();

    let mut position = Position { x: 0, y: 0 };
    for line in input {
        let mut command = line.split(" ");
        let direction = command.next().unwrap();
        let amount = i32::from_str_radix(command.next().unwrap(), 10).unwrap();

        match direction {
            "forward" => position.x = position.x + amount,
            "up" => position.y = position.y - amount,
            "down" => position.y = position.y + amount,
            _ => unreachable!()
        };
    }

    println!("Day\t2\tPart\t1: {}", position.x * position.y);
}

pub fn part2() {
    let input = read_input();

    let mut position = Position { x: 0, y: 0 };
    let mut aim = 0;

    for line in input {
        let mut command = line.split(" ");
        let direction = command.next().unwrap();
        let amount = i32::from_str_radix(command.next().unwrap(), 10).unwrap();

        match direction {
            "down" => aim = aim + amount,
            "up" => aim = aim - amount,
            "forward" => {
                position.x = position.x + amount;
                position.y = position.y + aim * amount
            },
            _ => unreachable!()
        };
    }

    println!("Day\t2\tPart\t2: {}", position.x * position.y);
}

fn read_input<'a>() -> Vec<String> {
    let input = std::fs::read_to_string("./src/day2/input.txt").unwrap();
    input
        .trim()
        .split("\n")
        .map(ToOwned::to_owned)
        .collect::<Vec<String>>()
}
