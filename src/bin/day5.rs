use std::io::{BufReader, BufRead};
use std::fs::File;
use itertools::Itertools;

fn main() {
    part1();
    part2();
}

fn part1() {
    let reader = BufReader::new(File::open("res/day5.in").unwrap());
    let mut maze : Vec<i32> = reader.lines()
        .map(|l| l.unwrap().parse::<i32>().unwrap())
        .collect();

    let mut counter = 0;
    let mut position = 0;
    let maze_len = maze.len() as i32;

    while position >= 0 && position < maze_len {
        let jump = maze[position as usize];
        maze[position as usize] += 1;

        position += jump;
        counter += 1;
    }

    println!("Part 1: {}", counter)
}

fn part2() {
    let reader = BufReader::new(File::open("res/day5.in").unwrap());
    let mut maze : Vec<i32> = reader.lines()
        .map(|l| l.unwrap().parse::<i32>().unwrap())
        .collect();

    let mut counter = 0;
    let mut position = 0;
    let maze_len = maze.len() as i32;

    while position >= 0 && position < maze_len {
        let jump = maze[position as usize];

        maze[position as usize] += if jump >= 3 {
            -1
        } else {
            1
        };

        position += jump;
        counter += 1;
    }

    println!("Part 2: {}", counter)
}
