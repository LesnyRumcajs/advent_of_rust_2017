fn main() {
    part1();
    part2();
}

fn nearest_perfect_odd_square(n: i32) -> i32 {
    let sq = (n as f32).sqrt() as i32;
    if sq % 2 == 0 {
        sq + 1
    } else {
        sq
    }
}

#[test]
fn nearest_perfect_odd_square_test() {
    assert_eq!(nearest_perfect_odd_square(9), 3);
    assert_eq!(nearest_perfect_odd_square(8), 3);
    assert_eq!(nearest_perfect_odd_square(6), 3);
}

fn part1() {
    let puzzle_input = 277678;
    let n = nearest_perfect_odd_square(puzzle_input);
    let distance = n - 1 - (n * n - puzzle_input).abs();

    println!("Part 1 answer: {}", distance)
}

use multiarray::*;

fn part2() {
    let puzzle_input = 277678;
    const ARR_DIMENSION: usize = 100;
    let mut spiral = Array2D::new([ARR_DIMENSION, ARR_DIMENSION], 0);

    let mut x_pos = ARR_DIMENSION / 2;
    let mut y_pos = ARR_DIMENSION / 2;

    spiral[[x_pos,y_pos]] = 1;

    enum Direction {
        Left, Right, Up, Down
    }

    let mut direction = Direction::Right;
    let mut right_border = x_pos + 1;
    let mut left_border = x_pos - 1;
    let mut bottom_border = y_pos;
    let mut top_border = y_pos - 1;

    let mut current_value = 0;

    while current_value < puzzle_input {
        current_value = sum_neighbours(&mut spiral, x_pos, y_pos);

        spiral[[x_pos,y_pos]] = current_value;

        match direction {
            Direction::Right => {
                x_pos += 1;
                if x_pos >= right_border {
                    direction = Direction::Up;
                    bottom_border += 1;
                }
            }
            Direction::Down => {
                y_pos += 1;
                if y_pos >= bottom_border {
                    direction = Direction::Right;
                    left_border -= 1;
                }
            }
            Direction::Left => {
                x_pos -= 1;
                if x_pos <= left_border {
                    direction = Direction::Down;
                    top_border -= 1;
                }
            }
            Direction::Up => {
                y_pos -= 1;
                if y_pos <= top_border {
                    direction = Direction::Left;
                    right_border += 1;
                }
            }
        }
    }
    println!("Part 2 answer: {}", current_value);
}

fn sum_neighbours(arr: &mut MultiArray<i32, Dim2>, x: usize, y: usize) -> i32{
    arr[[x - 1, y - 1]] +
        arr[[x - 1, y]] +
        arr[[x - 1, y + 1]] +
        arr[[x, y - 1]] +
        arr[[x, y]] +
        arr[[x, y + 1]] +
        arr[[x + 1, y - 1]] +
        arr[[x + 1, y]] +
        arr[[x + 1, y + 1]]
}