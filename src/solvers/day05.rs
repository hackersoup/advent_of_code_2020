use std::io::{BufReader, BufRead};
use std::fs::File;
use std::cmp::max;

pub fn solve(input: BufReader<File>) {
    let lines = input.lines()
        .map(|line| line.unwrap())
        .collect::<Vec<_>>();

    let mut max_id = 0;
    let mut id_list: Vec<usize> = Vec::new(); // Save the IDs for part 2 of the challenge
    for line in lines {
        let (_, _, id) = solve_line(&line);
        max_id = max(max_id, id);
        id_list.push(id);
    }
    id_list.sort(); // Our ID will be the only gap between two ID's, so we can sort the list and look for the gap


    // TODO Try turning this into an iterator-based functional solution
    let mut our_seat = 0;
    let mut prev_seat = 0;
    for seat_id in id_list.iter() {
        if seat_id - prev_seat == 2 { our_seat = prev_seat + 1; break; }
        prev_seat = *seat_id;
    }

    println!("[+] Day05-1: {}", max_id);
    println!("[+] Day05-2: {}", our_seat);
}

/// Solves the Binary Space Partition puzzle for one string input, returns tuple of (row, col, id)
pub fn solve_line(line: &str) -> (usize, usize, usize) {
    let mut row_lower_bound = 0;
    let mut row_upper_bound = 127;
    let mut col_lower_bound = 0;
    let mut col_upper_bound = 7;

    for c in line.chars() {
        // TODO Make this loop into functional-based iterator (eg using map) instead of a loop
        match c {
            'F' => {
                // Take upper half of BSP
                row_upper_bound = row_lower_bound + (row_upper_bound - row_lower_bound) / 2;
            },
            'B' => {
                // Take lower half of BSP
                row_lower_bound = (row_lower_bound + ((row_upper_bound - row_lower_bound) / 2)) + 1;
            },
            'L' => {
                col_upper_bound = col_lower_bound + (col_upper_bound - col_lower_bound) / 2;
            },
            'R' => {
                col_lower_bound = (col_lower_bound + ((col_upper_bound - col_lower_bound) / 2)) + 1;
            },
            _ => panic!()
        }

        //println!("-> {}: {} to {}", c, row_lower_bound, row_upper_bound);
    }
    // Lower and upper bounds should be equal now, either can be used as the row & col reference

    let row = row_upper_bound;
    let col = col_upper_bound;
    let id = row * 8 + col;

    (row, col, id)
}