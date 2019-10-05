use day8_helpers::model::*;
use itertools::Itertools;
use std::collections::BTreeMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

mod day8_helpers;

#[macro_use]
extern crate lazy_static;
fn main() {
    let reader = BufReader::new(File::open("res/day8.in").unwrap());
    let instructions = reader
        .lines()
        .map(|l| Instruction::from_str(&l.unwrap()).unwrap())
        .collect_vec();

    let mut registers: Registers = BTreeMap::new();

    let mut total_max: Option<i32> = None;
    for instruction in instructions {
        registers.entry(instruction.register.clone()).or_insert(0);
        registers
            .entry(instruction.condition.lhs.clone())
            .or_insert(0);

        let check = instruction.condition.check(&registers);
        if check {
            let val = registers.entry(instruction.register).or_insert(0);
            *val += match instruction.op {
                Operation::Inc => instruction.op_val,
                Operation::Dec => -instruction.op_val,
            };
        }
        let max = *registers.values().max().unwrap();
        if total_max.is_none() || max > total_max.unwrap() {
            total_max = Some(max);
        }
    }

    let max = registers.values().max().unwrap();
    println!("Max after whole process {}", max);
    println!("Local max: {}", total_max.unwrap());
}
