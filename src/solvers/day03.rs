use std::io::{BufReader, BufRead};
use std::fs::File;

pub fn solve(input: BufReader<File>) {
    /*
    /// Count the trees encountered for a given x and y hop value
    fn count_trees(lines: Vec<String>, x: usize, y: usize) -> usize {
        lines.iter()
            .enumerate()
            .filter_map(|(i, e)| {
                if i % y == 0 { Some(e) } else { None }
            })
            .fold((0, 0), |acc, line| {
                if line.as_bytes()[i % line.len()] == '#' as u8 { (acc + 1, i + x) } else { (acc, i + x) }
            })
    }
    */

    // Solve part 1
    let result_1 = input.lines()
        .fold((0, 0), |acc, line| {
            let line = line.unwrap();
            let index = acc.0 % line.len();
            let result = match line.as_bytes()[index] == '#' as u8 {
                true => acc.1 + 1,
                false => acc.1
            };

            (index + 3, result)
        }).1;

    //let lines = input.lines().map(|line| line.unwrap()).collect::<Vec<_>>();

    println!("[+] Day03-1: {}", result_1);
}