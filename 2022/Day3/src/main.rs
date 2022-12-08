use std::collections::HashMap;
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
    let mut arr: Vec<String> = Vec::new();
    if let Ok(lines) = read_lines("src/in.txt") {
        for line in lines {
            if let Ok(line) = line {
                arr.push(line);
            }
        }
    }
    // make copy of arr
    let arr2 = arr.clone();
    let t1 = std::thread::spawn(move || {
        let mut score: i32 = 0;
        for line in &arr {
            score += compute_rucksack(line.to_string());
        }
        println!("PART 1 SOCRE :{}", score);
    });
    let t2 = std::thread::spawn(move || {
        let mut score: i32 = 0;
        let mut batch: Vec<String> = Vec::new();
        let mut idx: i32 = 0;
        // for each 3 lines, compute the score
        for line in &arr2 {
            batch.push(line.to_string());
            idx += 1;
            if idx % 3 == 0 {
                score += part_2(&batch);
                batch.clear();
            }
        }
        println!("PART 2 SOCRE :{}", score);
    });
    t1.join().unwrap();
    t2.join().unwrap();
}
fn compute_rucksack(line: String) -> i32 {
    //split line in half
    let first_half = &line[0..line.len() / 2];
    let second_half = &line[line.len() / 2..line.len()];
    // find the similar character
    let common_charachter: i32 =
        find_common_across_two_strings(&first_half.to_string(), &second_half.to_string()) as i32;
    // if lower case, add 32
    if common_charachter > 96 {
        return common_charachter - 96;
    } else {
        return common_charachter - 38;
    }
    // return the difference
}
fn part_2(strings: &Vec<String>) -> i32 {
    let str_1 = &strings[0];
    let str_2 = &strings[1];
    let str_3 = &strings[2];
    // find the similar character accross 3 strings
    let common_charachter = find_common_across_three_strings(str_1, str_2, str_3);
    // println!("common_charachter:{}", common_charachter.to_string());
    let i = common_charachter as i32;
    if i > 96 {
        return i - 96;
    } else {
        return i - 38;
    }
}

fn find_common_across_three_strings(str_1: &String, str_2: &String, str_3: &String) -> char {
    let mut map: HashMap<u8, bool> = HashMap::new();
    // let mut map_2: HashMap<u8, bool> = HashMap::new();
    for (_, c) in str_1.chars().enumerate() {
        map.insert(c as u8, true);
    }
    for (_, c) in str_2.chars().enumerate() {
        if !map.contains_key(&(c as u8)) {
            //delete from map
            map.remove(&(c as u8));
        }
    }
    let mut key: u8 = 0;
    for (_, c) in str_3.chars().enumerate() {
        if map.contains_key(&(c as u8)) {
            //return c
            key = c as u8;
            break;
        }
    }
    key as char
}

fn find_common_across_two_strings(str_1: &String, str_2: &String) -> char {
    let mut map: HashMap<char, bool> = HashMap::new();
    // let mut map_2: HashMap<u8, bool> = HashMap::new();
    for (_, c) in str_1.chars().enumerate() {
        map.insert(c, true);
    }
    let mut key: char = '@';
    for (_, c) in str_2.chars().enumerate() {
        if map.contains_key(&c) {
            //delete from map
            key = c;
            break;
        }
    }
    // loop through map and return the key
    key as char
}
