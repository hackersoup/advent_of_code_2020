use std::io::{BufReader, BufRead};
use std::fs::File;

pub fn solve(input: BufReader<File>) {


    let lines = &input.lines()
        .map(|line| line.unwrap())
        .collect::<Vec<_>>();

    // Solve part 1
    let result_1 = count_trees(lines, 3, 1);

    let result_2 = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)].iter()
        .fold(1, |acc, (x, y)| {
            acc * count_trees(&lines, *x as usize, *y as usize)
        });

    println!("[+] Day03-1: {}", result_1);
    println!("[+] Day03-2: {}", result_2);
}

/// Count the trees encountered for a given x and y hop value
fn count_trees(lines: &Vec<String>, x: usize, y: usize) -> usize {
    lines.iter()
        .enumerate()
        .fold((0, 0), |(index, counter), (linenum, line)| {
            if linenum % y != 0 {
                // Skip this line
                return (index, counter)
            }
            let index = index % line.len();
            let result = match line.as_bytes()[index] == '#' as u8 {
                true => counter + 1,
                false => counter
            };

            (index + x, result)
        }).1
}