use std::fs::File;
use std::io::{self, BufRead};

struct Window {
    num: i32,
    elements: Vec<i32>,
}

impl Window {
    fn new(num: i32) -> Window {
        Self { num, elements: Vec::new() }
    }

    fn add(&mut self, e: i32) -> bool {
        if self.elements.len() == (self.num + 1) as usize  {
            let (_, tail) = self.elements.split_at(1);
            self.elements = Vec::from(tail);
        }
        self.elements.push(e);
        if self.elements.len() < (self.num + 1) as usize  {
            return false;
        }
        true
    }

    fn comp(&self) -> bool {
        let s1: i32 = self.elements.iter().take(self.num as usize).sum();
        let s2 = self.elements.iter().skip(1).sum();
        s1 < s2
    }
}

fn part_2(reader: io::BufReader<File>) {
    let mut window = Window::new(3);
    let mut num = 0i32;
    for line in reader.lines() {
        let value = line.unwrap().parse::<i32>().unwrap();
        if window.add(value) && window.comp() {
            num += 1;
        }
    }
    println!("number 2 {}", num);
}

fn part_1(reader: io::BufReader<File>) {
    let [num, _] = reader.lines().fold([0, -1] as [i32; 2], |acc, e| {
        let [sum, prev] = acc;
        let value = e.unwrap().parse::<i32>().unwrap();
        if prev < 0 {
            return [sum, value];
        }
        if prev < value {
            return [sum + 1, value];
        }
        [sum, value]
    });
    println!("number {}", num);
}

fn main() {
    part_1(io::BufReader::new(File::open("input").unwrap()));
    part_2(io::BufReader::new(File::open("input").unwrap()));
}
