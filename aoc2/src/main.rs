use std::fs::File;
use std::io::{self, BufRead};

#[derive(Copy, Clone)]
struct Submarine {
    x: i32,
    y: i32,
}

impl Submarine {
    fn new() -> Self {
        Self { x: 0, y: 0 }
    }
    fn mv(&mut self, direction: &str, value: i32) {
        match direction {
            "forward" => {
                self.x = self.x + value;
            },
            "down" => {
                self.y += value;
            },
            "up" => {
                self.y -= value;
            },
            _ => {

            }
        };
    }

    fn get_final_position(self) -> i32 {
        self.x * self.y
    }
}

#[derive(Copy, Clone)]
struct SubmarinePartTwo {
    x: i32,
    y: i32,
    aim: i32,
}

impl SubmarinePartTwo {
    fn new() -> Self {
        Self { x: 0, y: 0, aim: 0 }
    }
    fn mv(&mut self, direction: &str, value: i32) {
        match direction {
            "forward" => {
                self.x = self.x + value;
                self.y += self.aim * value;
            },
            "down" => {
                self.aim += value;
            },
            "up" => {
                self.aim -= value;
            },
            _ => {

            }
        };
    }

    fn get_final_position(self) -> i32 {
        self.x * self.y
    }
}

fn part_2(reader: io::BufReader<File>) {
    let mut submarine = SubmarinePartTwo::new();
    reader.lines().map(|line| {
        match line.unwrap().split(' ').collect::<Vec<&str>>().as_slice() {
            &[direction, value] => return (direction.to_owned(), value.parse::<i32>().unwrap()),
            _ => unreachable!()
        }
    }).for_each(|(direction, value)| {
        submarine.mv(&direction, value);
    });
    println!("result: {}", submarine.get_final_position());
}

fn part_1(reader: io::BufReader<File>) {
    let mut submarine = Submarine::new();
    reader.lines().map(|line| {
        match line.unwrap().split(' ').collect::<Vec<&str>>().as_slice() {
            &[direction, value] => return (direction.to_owned(), value.parse::<i32>().unwrap()),
            _ => unreachable!()
        }
    }).for_each(|(direction, value)| {
        submarine.mv(&direction, value);
    });
    println!("result: {}", submarine.get_final_position());
}

fn main() {
    part_1(io::BufReader::new(File::open("input2").unwrap()));
    part_2(io::BufReader::new(File::open("input2").unwrap()));
}
