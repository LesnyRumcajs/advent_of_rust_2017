use itertools::Itertools;
use std::fs::File;
use std::io::{BufRead, BufReader};
use itertools::MinMaxResult;
fn main() {
    part1();
    part2();
}

fn part1() {
    let reader = BufReader::new(File::open("res/day2.in").unwrap());
    let result: i32 = reader
        .lines()
        .map(|line| line.unwrap())
        .map(|x| {
            x.split_whitespace()
                .map(|x| x.parse::<i32>().unwrap())
                .collect_vec()
        })
        .map(|v| match v.iter().minmax(){
            MinMaxResult::MinMax(v1, v2) => v2 - v1,
            MinMaxResult::OneElement(_) => 0,
            _ => panic!("empty vector!")
        })
        .sum();

    println!("Part 1: {}", result);
}

fn part2() {
    let reader = BufReader::new(File::open("res/day2.in").unwrap());
    let rows: Vec<Vec<i32>> = reader
        .lines()
        .map(|line| line.unwrap())
        .map(|x| {
            x.split_whitespace()
                .map(|x| x.parse::<i32>().unwrap())
                .collect_vec()
        }).collect_vec();

    let mut result = 0;
    for nums in rows {
        for num in &nums {
            let other = nums.iter().find(|&x| (x % num == 0 || num % x == 0) && num != x);
            match other {
                Some(other) => {
                    result += num.max(other) / num.min(other);
                    break;
                },
                None => ()
            }
        }
    }

    println!("Part 2: {:?}", result);
}
