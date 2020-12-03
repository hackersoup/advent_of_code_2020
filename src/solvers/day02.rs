use std::io::{BufReader, BufRead};
use std::fs::File;

pub fn solve(input: BufReader<File>) {

    let lines = input.lines()
        .map(|line| line.unwrap())
        .collect::<Vec<_>>();

    let mut valid_passwords_count_part1 = 0;
    let mut valid_passwords_count_part2 = 0;

    for line in lines {
        // First parse apart the string into the tokens as defined by the challenge
        // TODO Would it be better to create a struct for this data and implement FromStr to parse
        let hyphen_index = line.find('-').unwrap();
        let first_whitespace = line.find(char::is_whitespace).unwrap();
        let first_colon = line.find(':').unwrap();
        let min_count = line[0..hyphen_index].parse::<usize>().unwrap();
        let max_count = line[hyphen_index+1..first_whitespace].parse::<usize>().unwrap();
        let pass_char = line[(first_whitespace+1)..first_colon].parse::<char>().unwrap();
        let password = &line[first_colon+1..];

        // Aggregate a count over the list where characters matching the pass_char increment the count
        let count = password.chars()
            .fold(0, |acc, x| {
                if x == pass_char { acc + 1 } else { acc }
            });

        // Only count passwords where the pass_count from above is between the min and max
        if min_count <= count && count <= max_count {
            valid_passwords_count_part1 += 1;
        }

        // Count passwords for the second part, where min and max are used as indices into password, and only one is allowed to be the pass_char
        valid_passwords_count_part2 += match (password.as_bytes()[min_count] as char == pass_char) ^ (password.as_bytes()[max_count] as char == pass_char) {
            true => 1,
            false => 0,
        }
    }
    println!("[+] Day02-1: {}", valid_passwords_count_part1);
    println!("[+] Day02-2: {}", valid_passwords_count_part2);
}