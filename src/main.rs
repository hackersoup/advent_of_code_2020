use std::{error::Error, fs::File};
use std::result::Result;
use std::io::{BufReader, BufRead};

#[allow(bare_trait_objects)]
fn main() -> Result<(), Box<Error>> {
    solve_01(BufReader::new(File::open("input/input_01")?));
    solve_02(BufReader::new(File::open("input/input_02")?));

    Ok(())
}

fn solve_01(input: BufReader<File>) {
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

fn solve_02(input: BufReader<File>) {

    let lines = input.lines()
        .map(|line| line.unwrap())
        .collect::<Vec<_>>();

    let mut valid_passwords_count_part1 = 0;
    let mut valid_passwords_count_part2 = 0;

    for line in lines {
        // First parse apart the string into the tokens as defined by the challenge
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
                if x == pass_char {
                    acc + 1
                } else {
                    acc
                }
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