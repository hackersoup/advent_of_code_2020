use std::io::{BufReader, BufRead};
use std::fs::File;

// It was really fun playing with rust benchmarks for this challenge!
// Used it to help find a faster solution than my initial naive approach

pub fn naive_one_round(numbers: &mut Vec<usize>) {
    let mut iter = numbers.iter().rev();
    let last_spoken = iter.next().unwrap();

    match iter.position(|x| *x == *last_spoken) {
        Some(num) => numbers.push(num + 1),
        None => numbers.push(0)
    }
}

pub fn naive_rounds_until(numbers: &mut Vec<usize>, round_count: usize) {
    for _i in numbers.len()..round_count {
        naive_one_round(numbers);
    }
}

/// Trying to make a faster version of the simple/naive approach
pub fn cached_rounds_until(numbers: &mut Vec<usize>, round_count: u32) {
    let cache_size = round_count;
    // Using u32 for the cache value seems to dramatically increase the performance
    // as compared to usize, on my system
    let mut cache: Vec<Option<u32>> = vec![None; cache_size as usize];

    // Build the initial cache state
    for (index, value) in numbers.iter().enumerate() {
        if *value < cache_size as usize {
            cache[*value] = Some(index as u32);
        }
    }

    for i in numbers.len()..round_count as usize {
        let last_spoken = *numbers.last().unwrap();
        let i = i - 1; // Took me forever to find the off by one bug that required adding this line -_-

        // Get the last index of the to-be-spoken word.
        // If the number hasn't appeared before, it will be zero.
        // If it has, it will be the index of the last appearance, so
        // the index will need to be subtracted from the current index
        if last_spoken < cache_size as usize {
            let last_index = cache[last_spoken];
            match last_index {
                None => numbers.push(0),
                Some(last_index) => numbers.push(i - last_index as usize),
            }

            // Update the cache of the most recent word
            cache[last_spoken] = Some(i as u32);
        } else {
            naive_one_round(numbers);
        }
    }
}

pub fn solve(input: BufReader<File>) {
    /*
        My solution for this is really really bad, running the 30,000,000 round
        count takes WAY too long to compute. There has to be a more efficient
        way than what I am doing right now... Maybe some kind of cache with
        a HashMap or an array?
    */
    let line = input.lines()
        .map(|line| line.unwrap())
        .collect::<Vec<_>>()
        .join("");
    
    let mut prev = 0;
    let mut numbers = line.split(',')
        .map(|n| {
            let i = n.parse::<usize>().unwrap();
            prev = i;
            i
        })
        .collect::<Vec<_>>();
    // Since the vecs get mutated, copy to a new one for the second part of the challenge.
    let mut numbers_2 = numbers.to_vec();

    // Using the slow version for this so that all my code, even the bad code, feels included :P
    // TODO Add logic to test round count to determine which method to use.
    naive_rounds_until(&mut numbers, 2020);
    let answer_1 = *numbers.last().unwrap();

    cached_rounds_until(&mut numbers_2, 30_000_000);
    let answer_2 = *numbers_2.last().unwrap();

    println!("[+] Day15-1: {}", answer_1);
    println!("[+] Day15-2: {}", answer_2);
}

#[cfg(test)]
mod tests {    
    use super::*;
    extern crate test;
    use test::bench::Bencher;

    #[bench]
    fn bench_cached_rounds_until_1000(b: &mut Bencher) {
        b.iter(|| {
            let mut numbers = vec![1, 2, 3];
            cached_rounds_until(&mut numbers, 1_000);
            let _answer = *numbers.last().unwrap();
        });
    }

    #[bench]
    fn bench_cached_rounds_until_10000(b: &mut Bencher) {
        b.iter(|| {
            let mut numbers = vec![1, 2, 3];
            cached_rounds_until(&mut numbers, 10_000);
            let _answer = *numbers.last().unwrap();
        });
    }

    #[bench]
    fn bench_cached_rounds_until_100000(b: &mut Bencher) {
        b.iter(|| {
            let mut numbers = vec![1, 2, 3];
            cached_rounds_until(&mut numbers, 100_000);
            let _answer = *numbers.last().unwrap();
        });
    }

    #[bench]
    fn bench_cached_rounds_until_1000000(b: &mut Bencher) {
        b.iter(|| {
            let mut numbers = vec![1, 2, 3];
            cached_rounds_until(&mut numbers, 1_000_000);
            let _answer = *numbers.last().unwrap();
        });
    }

    #[bench]
    fn bench_naive_rounds_until_1000(b: &mut Bencher) {
        b.iter(|| {
            let mut numbers = vec![1, 2, 3];
            naive_rounds_until(&mut numbers, 1_000);
            let _answer = *numbers.last().unwrap();
        });
    }

    #[bench]
    fn bench_naive_rounds_until_10000(b: &mut Bencher) {
        b.iter(|| {
            let mut numbers = vec![1, 2, 3];
            naive_rounds_until(&mut numbers, 10_000);
            let _answer = *numbers.last().unwrap();
        });
    }

    #[test]
    fn test_rounds_until() {
        let mut numbers = vec![0, 3, 6];
        cached_rounds_until(&mut numbers, 10);
        assert_eq!(numbers, vec![0, 3, 6, 0, 3, 3, 1, 0, 4, 0]);

        // Test for 2020 rounds

        let mut numbers = vec![1, 3, 2];
        cached_rounds_until(&mut numbers, 2020);
        let solution = numbers.last().unwrap();
        assert_eq!(*solution, 1);

        let mut numbers = vec![2, 1, 3];
        cached_rounds_until(&mut numbers, 2020);
        let solution = numbers.last().unwrap();
        assert_eq!(*solution, 10);

        let mut numbers = vec![1, 2, 3];
        cached_rounds_until(&mut numbers, 2020);
        let solution = numbers.last().unwrap();
        assert_eq!(*solution, 27);

        let mut numbers = vec![2, 3, 1];
        cached_rounds_until(&mut numbers, 2020);
        let solution = numbers.last().unwrap();
        assert_eq!(*solution, 78);

        let mut numbers = vec![3, 2, 1];
        cached_rounds_until(&mut numbers, 2020);
        let solution = numbers.last().unwrap();
        assert_eq!(*solution, 438);

        let mut numbers = vec![3, 1, 2];
        cached_rounds_until(&mut numbers, 2020);
        let solution = numbers.last().unwrap();
        assert_eq!(*solution, 1836);
    }

    #[test]
    fn test_rounds_until_simple() {
        let mut numbers = vec![0, 3, 6];
        naive_rounds_until(&mut numbers, 10);
        assert_eq!(numbers, vec![0, 3, 6, 0, 3, 3, 1, 0, 4, 0]);

        // Test for 2020 rounds

        let mut numbers = vec![1, 3, 2];
        naive_rounds_until(&mut numbers, 2020);
        let solution = numbers.last().unwrap();
        assert_eq!(*solution, 1);

        let mut numbers = vec![2, 1, 3];
        naive_rounds_until(&mut numbers, 2020);
        let solution = numbers.last().unwrap();
        assert_eq!(*solution, 10);

        let mut numbers = vec![1, 2, 3];
        naive_rounds_until(&mut numbers, 2020);
        let solution = numbers.last().unwrap();
        assert_eq!(*solution, 27);

        let mut numbers = vec![2, 3, 1];
        naive_rounds_until(&mut numbers, 2020);
        let solution = numbers.last().unwrap();
        assert_eq!(*solution, 78);

        let mut numbers = vec![3, 2, 1];
        naive_rounds_until(&mut numbers, 2020);
        let solution = numbers.last().unwrap();
        assert_eq!(*solution, 438);

        let mut numbers = vec![3, 1, 2];
        naive_rounds_until(&mut numbers, 2020);
        let solution = numbers.last().unwrap();
        assert_eq!(*solution, 1836);

        /* Commented out because of the vast amount of time these tests take.
        // Test for 30_000_000 rounds
        let mut numbers = vec![1, 3, 2];
        rounds_until_simple(&mut numbers, 30_000_000);
        let solution = numbers.last().unwrap();
        assert_eq!(*solution, 175594);

        let mut numbers = vec![2, 1, 3];
        rounds_until_simple(&mut numbers, 2020);
        let solution = numbers.last().unwrap();
        assert_eq!(*solution, 10);

        let mut numbers = vec![1, 2, 3];
        rounds_until_simple(&mut numbers, 2020);
        let solution = numbers.last().unwrap();
        assert_eq!(*solution, 27);

        let mut numbers = vec![2, 3, 1];
        rounds_until_simple(&mut numbers, 2020);
        let solution = numbers.last().unwrap();
        assert_eq!(*solution, 78);

        let mut numbers = vec![3, 2, 1];
        rounds_until_simple(&mut numbers, 2020);
        let solution = numbers.last().unwrap();
        assert_eq!(*solution, 438);

        let mut numbers = vec![3, 1, 2];
        rounds_until_simple(&mut numbers, 2020);
        let solution = numbers.last().unwrap();
        assert_eq!(*solution, 1836);
        */
    }

    /// Test a few iterations of one_round
    #[test]
    fn test_one_round_simple() {
        let mut numbers = vec![0, 3, 6];
        
        naive_one_round(&mut numbers);
        assert_eq!(numbers, vec![0, 3, 6, 0]);

        naive_one_round(&mut numbers);
        assert_eq!(numbers, vec![0, 3, 6, 0, 3]);

        naive_one_round(&mut numbers);
        assert_eq!(numbers, vec![0, 3, 6, 0, 3, 3]);

        naive_one_round(&mut numbers);
        assert_eq!(numbers, vec![0, 3, 6, 0, 3, 3, 1]);

        naive_one_round(&mut numbers);
        assert_eq!(numbers, vec![0, 3, 6, 0, 3, 3, 1, 0]);

        naive_one_round(&mut numbers);
        assert_eq!(numbers, vec![0, 3, 6, 0, 3, 3, 1, 0, 4]);

        naive_one_round(&mut numbers);
        assert_eq!(numbers, vec![0, 3, 6, 0, 3, 3, 1, 0, 4, 0]);
    }
}