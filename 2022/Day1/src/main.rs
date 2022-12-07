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
    // read the file
    if let Ok(lines) = read_lines("src/calories.txt") {
        // iterate over the lines
        let mut largest: i32 = 0;
        let mut current_tally: i32 = 0;
        for line in lines {
            if let Ok(line) = line {
                if line == "" {
                    //if the current tally is larger than the largest, set the largest to the current tally
                    if current_tally > largest {
                        largest = current_tally;
                    }
                    //reset the current tally to 0
                    current_tally = 0;
                } else {
                    //parse the line as an integer
                    let calories: i32 = line.parse().unwrap();
                    //add the calories to the current tally
                    current_tally = current_tally + calories;
                }
            }
        }
        //print the largest
        println!("The largest is {}", largest);
    }
}
