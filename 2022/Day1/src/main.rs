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

//LRU Cache
use std::collections::HashMap;
use std::collections::VecDeque;

struct LRUCache {
    capacity: usize,
    queue: VecDeque<i32>,
    map: HashMap<i32, i32>,
}

impl LRUCache {
    fn new(capacity: i32) -> Self {
        LRUCache {
            capacity: capacity as usize,
            queue: VecDeque::new(),
            map: HashMap::new(),
        }
    }

    fn get(&mut self, key: i32) -> i32 {
        if self.map.contains_key(&key) {
            let value = self.map.get(&key).unwrap();
            self.queue.retain(|&x| x != key);
            self.queue.push_front(key);
            return *value;
        }
        return -1;
    }

    fn put(&mut self, key: i32, value: i32) {
        if self.map.contains_key(&key) {
            self.queue.retain(|&x| x != key);
            self.queue.push_front(key);
            self.map.insert(key, value);
        } else {
            if self.queue.len() == self.capacity {
                let last = self.queue.pop_back().unwrap();
                self.map.remove(&last);
            }
            self.queue.push_front(key);
            self.map.insert(key, value);
        }
    }
}

fn main() {
    // read the file
    if let Ok(lines) = read_lines("src/calories.txt") {
        // iterate over the lines
        let mut largest: i32 = 0;
        let mut second_largest: i32 = 0;
        let mut third_largest: i32 = 0;
        let mut current_tally: i32 = 0;
        for line in lines {
            if let Ok(line) = line {
                if line == "" {
                    //if the current tally is larger than the largest, set the largest to the current tally
                    if current_tally > largest {
                        third_largest = second_largest;
                        second_largest = largest;
                        largest = current_tally;
                    } else if current_tally > second_largest {
                        //if the current tally is larger than the second largest, set the second largest to the current tally
                        third_largest = second_largest;
                        second_largest = current_tally;
                    } else if current_tally > third_largest {
                        //if the current tally is larger than the third largest, set the third largest to the current tally
                        third_largest = current_tally;
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
        //print the second largest
        println!("The second largest is {}", second_largest);
        //print the third largest
        println!("The third largest is {}", third_largest);
        //print the total calories
        println!(
            "The total calories is {}",
            largest + second_largest + third_largest
        );
    }
}
