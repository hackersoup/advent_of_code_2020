use std::io::{BufReader, BufRead};
use std::fs::File;
use std::cmp::max;

pub fn solve(input: BufReader<File>) {
    let lines = input.lines()
        .map(|line| line.unwrap())
        .collect::<Vec<_>>();


    // Part 1
    let max_id = &lines.iter()
        // Map each line into its seat ID
        .map(|line| solve_line(line))
        // Return the maximum seat ID found in the array
        .fold(0, |max_id, (_, _, id)| {
            max(max_id, id)
        });

    // Part 2
    // First get the list of seat IDs
    let mut seats = lines.iter().map(|line| solve_line(line).2).collect::<Vec<_>>();
    // Now sort it so that we can apply the search function
    seats.sort();
    let seats = seats;
    let my_seat = find_my_seat(seats);

    println!("[+] Day05-1: {}", max_id);
    println!("[+] Day05-2: {}", my_seat);
}

/// Take in a sorted vector of seat IDs and return the seat the person has
pub fn find_my_seat(seats: Vec<usize>) -> usize {
    // This is hideous and nearly unreadable, but I just wanted to see if I could
    // solve this with functional programming and no muts
    seats.iter()
        .fold((0, 0), |(prev_seat, our_seat), curr_seat| {
            if *curr_seat - prev_seat == 2 {
                // Found our seat! Note, this is not robust AT ALL, if the first
                // list element is 2 then that will be mistaken for the correct seat...
                // But the problem statement explicitly says that cannot be the case, so... :)
                (*curr_seat, prev_seat + 1)
            }
            else { (*curr_seat, our_seat) }
        }).1
}

pub fn solve_line(line: &str) -> (usize, usize, usize) {
    let (row_lower_bound, _, col_lower_bound, _) = line.chars()
        // Input tuple is (row_lower_bound, row_upper_bound, col_lower_bound, col_upper_bound)
        .fold((0, 127, 0, 7), |(rl, ru, cl, cu), c| {
            match c {
                'F' => (rl, (rl + ((ru - rl) / 2)), cl, cu), // Lower row BSP result
                'B' => ((rl + ((ru - rl) / 2) + 1), ru, cl, cu), // Upper row BSP result
                'L' => (rl, ru, cl, (cl + (cu - cl) / 2)), // Lower col BSP result
                'R' => (rl, ru, (cl + ((cu - cl) / 2) + 1), cu), // Upper col BSP result
                _ => (rl ,ru, cl, cu) // Ignore bad chars, assume the line is always well formed
            }
        });

    (row_lower_bound, col_lower_bound, row_lower_bound * 8 + col_lower_bound)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_line() {
        assert_eq!(solve_line("FBFBBFFRLR"), (44, 5, 357));
        assert_eq!(solve_line("BFFFBBFRRR"), (70, 7, 567));
        assert_eq!(solve_line("FFFBBBFRRR"), (14, 7, 119));
        assert_eq!(solve_line("BBFFBBFRLL"), (102, 4, 820));
    }
}

#[allow(dead_code)]
mod deprecated {
    // Previous iteration of the solution, just keeping it here for the sake of reference, so people don't have to dig through commits to see it
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
                    // Take lower half of row BSP
                    row_upper_bound = row_lower_bound + (row_upper_bound - row_lower_bound) / 2;
                },
                'B' => {
                    // Take upper half of row BSP
                    row_lower_bound = (row_lower_bound + ((row_upper_bound - row_lower_bound) / 2)) + 1;
                },
                'L' => {
                    // Take lower half of column BSP
                    col_upper_bound = col_lower_bound + (col_upper_bound - col_lower_bound) / 2;
                },
                'R' => {
                    // Take upper half of column BSP
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
}