use std::io::{BufReader, BufRead};
use std::fs::File;

pub fn solve(input: BufReader<File>) {
    let mut solution_1 = 0;
    let mut solution_2 = 0;

    // Read lines from STDIN and process as integers, ignore errors
    let lines = input.lines()
        .map(|line| line.unwrap().parse::<isize>().unwrap())
        .collect::<Vec<_>>();

    let length = lines.len();

    for i in 0..length {
        for j in i..length {
            if lines[i] + lines[j] == 2020 {
                solution_1 = lines[i] * lines[j];
                //println!("{} + {} = 2020", lines[i], lines[j]);
                //println!("{} * {} = {}", lines[i], lines[j], solution_1);
                break;
            }
            for k in j..length {
                if lines[i] + lines[j] + lines[k] == 2020 {
                    solution_2 = lines[i] * lines[j] * lines[k];
                    //println!("{} + {} + {} = 2020", lines[i], lines[j], lines[k]);
                    //println!("{} * {} * {} = {}", lines[i], lines[j], lines[k], solution_2);
                    break;
                }
            }
        }
    }

    println!("[+] Day01-1: {}", solution_1);
    println!("[+] Day01-2: {}", solution_2);
}