#[cfg(test)]
mod tests {
    use AOC::AOC;
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
            score += AOC::compute_rucksack(line);
        }
        assert_eq!(0, -1);
    }
}
