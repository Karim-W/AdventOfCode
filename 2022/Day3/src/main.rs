use std::fs::File;
use std::io::{self, BufRead, BufReader, Lines};
use std::path::Path;
fn read_lines<P>(path: P) -> io::Result<Lines<BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(path)?;
    Ok(BufReader::new(file).lines())
}
fn main() {
    if let Ok(lines) = read_lines("src/in.txt") {
        let mut total: i32 = 0;
        for line in lines {
            if let Ok(line) = line {
                let common = AOC::compute_rucksack(line);
                println!("Common: {}", common);
                total += common;
            }
        }
        println!("Total: {}", total);
    }
}

pub struct AOC {}

impl AOC {
    pub fn new() -> AOC {
        AOC {}
    }
    pub fn compute_rucksack(line: String) -> i32 {
        //split line in half
        let firstHalf = &line[0..line.len() / 2];
        let secondHalf = &line[line.len() / 2..line.len()];
        // find the similar character
        let mut common_charachter: i32 = 64;
        for (i, c) in firstHalf.chars().enumerate() {
            // find if charachter is in second half
            for (j, d) in secondHalf.chars().enumerate() {
                if c == d {
                    println!("{} {} common is {}", firstHalf, secondHalf, c);
                    common_charachter = c as i32;
                    break;
                }
            }
            if common_charachter != 64 {
                break;
            }
        }
        // if lower case, add 32
        if common_charachter > 96 {
            return common_charachter - 96;
        } else {
            return common_charachter - 38;
        }
        // return the difference
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let lines: Vec<&str> = vec![
            "vJrwpWtwJgWrhcsFMMfFFhFp",
            "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL",
            "PmmdzqPrVvPwwTWBwg",
            "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn",
            "ttgJtRGJQctTZtZT",
            "CrZsJsPPZsGzwwsLwLmpwMDw",
        ];
        let mut score: i32 = 0;
        for line in lines {
            score += AOC::compute_rucksack(line.to_string());
        }
        assert_eq!(score, 157);
    }
}
