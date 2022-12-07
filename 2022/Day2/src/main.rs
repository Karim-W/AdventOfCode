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
    if let Ok(lines) = read_lines("src/outcomes.txt") {
        // iterate over the lines
        let mut runningScore: i32 = 0;
        let mut runningScore2: i32 = 0;
        for line in lines {
            if let Ok(line) = line {
                // split the line into a vector of strings
                let v: Vec<&str> = line.split_whitespace().collect();
                runningScore = runningScore + computeScore(v[0], v[1]);
                runningScore2 = runningScore2 + computePart2(v[0], v[1]);
                println!(
                    "{} {} {} {}",
                    v[0],
                    v[1],
                    computeScore(v[0], v[1]),
                    computePart2(v[0], v[1])
                );
            }
        }
        println!("Final Score: {}", runningScore);
        println!("Final Score Part 2: {}", runningScore2);
    }
}

fn computeScore(opponent: &str, mine: &str) -> i32 {
    let opponentsPick: i32 = match opponent {
        "A" => 1,
        "B" => 2,
        "C" => 3,
        _ => 0,
    };
    let myPick: i32 = match mine {
        "X" => 1,
        "Y" => 2,
        "Z" => 3,
        _ => 0,
    };
    let score: i32 = match (opponentsPick, myPick) {
        (1, 1) => 3,
        (1, 2) => 6,
        (1, 3) => 0,
        (2, 1) => 0,
        (2, 2) => 3,
        (2, 3) => 6,
        (3, 1) => 6,
        (3, 2) => 0,
        (3, 3) => 3,
        _ => 0,
    };
    return score + myPick;
}

fn computePart2(opponent: &str, mine: &str) -> i32 {
    let opponentsPick: i32 = match opponent {
        "A" => 1,
        "B" => 2,
        "C" => 3,
        _ => 0,
    };
    let outcome: i32 = match mine {
        "X" => 0,
        "Y" => 3,
        "Z" => 6,
        _ => 0,
    };
    let myPick: i32 = match (opponentsPick, outcome) {
        (1, 0) => 3,
        (1, 3) => 1,
        (1, 6) => 2,
        (2, 0) => 1,
        (2, 3) => 2,
        (2, 6) => 3,
        (3, 0) => 2,
        (3, 3) => 3,
        (3, 6) => 1,
        _ => 0,
    };
    return outcome + myPick;
}
