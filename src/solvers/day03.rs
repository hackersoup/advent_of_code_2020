use std::io::{BufReader, BufRead};
use std::fs::File;

pub fn solve(input: BufReader<File>) {


    let lines = input.lines()
        .map(|line| line.unwrap())
        .collect::<Vec<_>>();

    // Solve part 1
    let result_1 = count_trees(lines, 3, 1);

    //let lines = input.lines().map(|line| line.unwrap()).collect::<Vec<_>>();

    println!("[+] Day03-1: {}", result_1);
}

/// Count the trees encountered for a given x and y hop value
fn count_trees(lines: Vec<String>, x: usize, y: usize) -> usize {
    lines.iter()
        .fold((0, 0), |(index, counter), line| {
            let index = index % line.len();
            let result = match line.as_bytes()[index] == '#' as u8 {
                true => counter + 1,
                false => counter
            };

            (index + x, result)
        }).1
}