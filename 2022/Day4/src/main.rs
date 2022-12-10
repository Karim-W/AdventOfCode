use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Lines};
use std::path::Path;
/*
CAMP CLEANUP
*/
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
    let mut count: i32 = 0;
    for i in 0..arr.len() {
        if is_engulfed(&arr[i]) {
            println!("{} contain one another", arr[i]);
            count += 1;
        }
    }
    println!("{} pairs of lines contain one another", count);
}

fn is_engulfed(line: &str) -> bool {
    // split the line into two lines by comma
    let line_split = line.split(',').collect::<Vec<&str>>();
    let clean_line_1 = line_split[0].split('-').collect::<Vec<&str>>();
    let clean_line_2 = line_split[1].split('-').collect::<Vec<&str>>();
    if clean_line_2[0] <= clean_line_1[0] && clean_line_1[1] <= clean_line_2[1]
        || clean_line_1[0] <= clean_line_2[0] && clean_line_2[1] >= clean_line_1[1]
    {
        return true;
    }
    return false;
}
