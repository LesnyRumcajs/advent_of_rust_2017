use itertools::Itertools;
use std::fs;

fn main() {
    part1();
    part2();
}

fn part1() {
    let data = fs::read_to_string("res/day1.in").unwrap();
    let mut data = data.trim().to_owned();
    data.push(data.chars().nth(0).unwrap());

    let result = data.chars().collect_vec().windows(2).fold(0, |sum, ch| {
        sum + if ch[0] == ch[1] {
            ch[0].to_digit(10).to_owned().unwrap()
        } else {
            0
        }
    });

    println!("Part 1: {}", result);
}

fn part2() {
    let data = fs::read_to_string("res/day1.in").unwrap();
    let data = data.trim();

    let mut result = 0;
    let step = data.len() / 2;

    for (i, ch) in data.chars().enumerate() {
        let idx = if i < step { i + step } else { i - step };

        if ch == data.chars().nth(idx).unwrap() {
            result += ch.to_digit(10).unwrap();
        }
    }

    println!("Part 2: {}", result);
}
