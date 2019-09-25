use itertools::Itertools;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    part1();
    part2();
}

fn part1() {
    let reader = BufReader::new(File::open("res/day4.in").unwrap());
    let unique_passphrases_count = reader
        .lines()
        .map(|line| {
            line.unwrap()
                .split_whitespace()
                .map(|w| w.to_owned())
                .collect_vec()
        })
        .filter(|l| l.iter().unique().count() == l.len())
        .count();

    println!("Part 1: {}", unique_passphrases_count)
}

fn part2() {
    let reader = BufReader::new(File::open("res/day4.in").unwrap());
    let unique_passphrases_count = reader
        .lines()
        .map(|line| {
            line.unwrap()
                .split_whitespace()
                .map(|w| {
                    let mut word = w.to_owned().chars().collect_vec();
                    word.sort();
                    word.into_iter().collect::<String>()
                })
                .collect_vec()
        })
        .filter(|l| l.iter().unique().count() == l.len())
        .count();

    println!("Part 2: {}", unique_passphrases_count)
}
