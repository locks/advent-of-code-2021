use std::{fmt, process::exit};

pub fn run() {
    part1();
    part2();
}

pub fn part1() {
    let (numbers, mut boards) = read_input();
    let (number, bingo_index) = first_bingo_board(numbers, &mut boards).unwrap();
    let board = boards.get(bingo_index).unwrap();

    let total = number * board.score();

    println!("Day\t4\tPart\t1: {}", total);
}

fn first_bingo_board(numbers: Vec<i32>, boards: &mut Vec<Board>) -> Option<(i32, usize)> {
    let mut numbers_iter = numbers.iter();
    while let Some(number) = numbers_iter.next() {
        for (board_index, board) in boards.iter_mut().enumerate() {
            board.mark(*number);
            if board.is_bingo() {
                return Some((*number, board_index));
            }
        }
    }

    None
}

pub fn part2() {
    let (numbers, mut boards) = read_input();
    let (number, bingo_index) = last_bingo_board(numbers, &mut boards).unwrap();
    let board = boards.get(bingo_index).unwrap();

    let total = number * board.score();

    println!("Day\t4\tPart\t1: {}", total);
}

fn last_bingo_board(numbers: Vec<i32>, boards: &mut Vec<Board>) -> Option<(i32, usize)> {
    let mut numbers_iter = numbers.iter();
    let mut bingo_count = boards.len();
    while let Some(number) = numbers_iter.next() {
        for (board_index, board) in boards.iter_mut().enumerate() {
            if !board.is_bingo() {
                board.mark(*number);

                if board.is_bingo() {
                    bingo_count -= 1;

                    if bingo_count == 0 {
                        return Some((*number, board_index));
                    }
                }
            }
        }
    }

    None
}

fn read_input<'a>() -> (Vec<i32>, Vec<Board>) {
    let file = std::fs::read_to_string("./src/day4/sample.txt").unwrap();
    let mut file = file.trim().split("\n\n");

    let numbers = file
        .next()
        .unwrap()
        .split(",")
        .map(|number| i32::from_str_radix(number, 10).unwrap())
        .collect::<Vec<i32>>();
    let boards = read_boards(&mut file);

    (numbers, boards)
}

fn read_boards(file: &mut core::str::Split<&str>) -> Vec<Board> {
    let mut boards = vec![];
    while let Some(line) = file.next() {
        let content = line
            .split("\n")
            .map(|row| {
                row.split_whitespace()
                    .map(|number| i32::from_str_radix(number, 10).unwrap())
                    .collect::<Vec<i32>>()
            })
            .collect::<Vec<Vec<i32>>>();

        boards.push(Board::from_vec(content));
    }

    boards
}

#[derive(Debug, Clone)]
struct Board {
    board: Vec<Vec<Cell>>,
}

#[derive(Clone)]
struct Cell {
    value: i32,
    marked: bool,
}

impl fmt::Debug for Cell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.value, self.marked)
    }
}

impl Cell {
    fn new(value: i32) -> Self {
        Cell {
            value,
            marked: false,
        }
    }

    fn mark(&mut self, value: i32) {
        if self.value == value {
            self.marked = true;
        }
    }
}

impl Board {
    fn from_vec(vec: Vec<Vec<i32>>) -> Self {
        Self {
            board: vec
                .iter()
                .map(|row| {
                    row.iter()
                        .map(|column| Cell::new(*column))
                        .collect::<Vec<Cell>>()
                })
                .collect::<Vec<Vec<Cell>>>(),
        }
    }

    pub(crate) fn mark(&mut self, number: i32) -> bool {
        for row in self.board.iter_mut() {
            for column in row.iter_mut() {
                column.mark(number);
            }
        }

        true
    }

    pub(crate) fn is_bingo(&self) -> bool {
        // row check
        let rows = self
            .board
            .iter()
            .any(|row| row.iter().all(|cell| cell.marked));
        if rows {
            return true;
        }

        // column check
        transpose(&self.board)
            .iter()
            .any(|row| row.iter().all(|cell| cell.marked))
    }

    pub(crate) fn score(&self) -> i32 {
        let mut sum = 0;
        for row in &self.board {
            for column in row {
                if !column.marked {
                    sum += column.value;
                }
            }
        }

        sum
    }
}

fn transpose(matrix: &Vec<Vec<Cell>>) -> Vec<Vec<&Cell>> {
    let size = matrix.get(0).unwrap().len() - 1;
    let mut transposed = vec![];
    for i in 0..size {
        let mut row: Vec<&Cell> = vec![];
        for j in 0..size {
            row.push(matrix.get(j).unwrap().get(i).unwrap());
        }

        transposed.push(row);
    }

    transposed
}
