use std::io::{BufReader, Read};
use std::fs::File;
use std::collections::HashSet;

pub fn solve(mut input: BufReader<File>) { 
    let mut file_data = String::new();
    input.read_to_string(&mut file_data).unwrap();
    // Split apart the groups based on the double newline delimiter as specified
    let groups = file_data.split("\n\n").collect::<Vec<_>>();

    let mut part1_answer = 0;
    for group in &groups {
        let mut answers_count_part1: HashSet<char> = HashSet::new();
        for line in group.lines() {
            for c in line.chars() {
                answers_count_part1.insert(c);
            }
        }
        part1_answer += answers_count_part1.len();
    }

    /* Part 2 Not working yet
    let mut part2_answer = 0;
    for group in &groups {
        let mut answers_list: Vec<HashSet<char>> = Vec::new();
        for line in group.lines() {
            let mut answers: HashSet<char> = HashSet::new();
            for c in line.chars() {
                answers.insert(c);
            }
            answers_list.push(answers);
        }
    }
    */

    println!("[+] Day06-1: {}", part1_answer);
    // println!("[+] Day06-2: {}", part2_answer);
}