use itertools::Itertools;
use std::collections::BTreeSet;
use std::fs;

fn has_unique_elements<T>(iter: T) -> bool
where
    T: IntoIterator,
    T::Item: Eq + Ord,
{
    let mut unique = BTreeSet::new();
    iter.into_iter().all(|el| unique.insert(el))
}

fn index_of_max(values: &[i32]) -> Option<usize> {
    let max = values.iter().max().unwrap();
    values
        .iter()
        .enumerate()
        .find(|(_, &x)| x == *max)
        .map(|(x, _)| x)
}

fn main() {
    let mut memory_bank = fs::read_to_string("res/day6.in")
        .unwrap()
        .split_whitespace()
        .map(|num| num.parse::<i32>().unwrap())
        .collect_vec();

    let mut memory_bank_history: Vec<Vec<i32>> = Vec::new();
    memory_bank_history.push(memory_bank.to_owned());

    while has_unique_elements(&memory_bank_history) {
        let idx = index_of_max(&memory_bank).unwrap();

        let mut distribution = memory_bank[idx];
        memory_bank[idx] = 0;
        for i in (0..memory_bank.len()).cycle().skip(idx + 1) {
            memory_bank[i] += 1;
            distribution -= 1;

            if distribution == 0 {
                break;
            }
        }
        memory_bank_history.push(memory_bank.to_owned());
    }
    let memory_bank_len = memory_bank_history.len() - 1;
    println!("Part 1: {}", memory_bank_len);

    let infinite_loop_begin = memory_bank_history
        .iter()
        .find_position(|x| x == &memory_bank_history.last().unwrap())
        .unwrap()
        .0;
    println!("Part 2: {}", memory_bank_len - infinite_loop_begin);
}
