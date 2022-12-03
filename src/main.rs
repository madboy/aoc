use std::fs;

fn main() {
    println!("Hello, world!");
}

fn get_contents(path: &str) -> String {
    fs::read_to_string(path).expect("File not found")
}

pub mod day01 {
    fn get_calories(data: String) -> Vec<i32> {
        data.split("\r\n\r\n")
            .map(|pack| {
                pack.lines()
                    .map(|calorie| calorie.parse::<i32>().expect("Should be a number"))
                    .sum()
            })
            .collect()
    }

    pub fn solve(data: String) -> (i32, i32) {
        let mut packs = get_calories(data);
        packs.sort();

        let packs = packs.iter().rev();

        return (packs.clone().take(1).sum(), packs.clone().take(3).sum());
    }
}

pub mod day02 {
    use std::collections::HashMap;
    fn score(data: String, scoring: HashMap<(&str, &str), i32>) -> i32 {
        data.lines()
            .map(|line| line.split_whitespace())
            .map(|mut split| {
                scoring
                    .get(&(split.next().unwrap(), split.next().unwrap()))
                    .unwrap()
            })
            .sum::<i32>()
    }
    pub fn part1(data: String) -> i32 {
        // A, B, C = rock, paper, scissor
        // X, Y, Z = rock, paper, scissor
        // rock, paper, scissor = 1, 2, 3
        let scoring = HashMap::from([
            (("A", "X"), 1 + 3),
            (("A", "Y"), 2 + 6),
            (("A", "Z"), 3 + 0),
            (("B", "X"), 1 + 0),
            (("B", "Y"), 2 + 3),
            (("B", "Z"), 3 + 6),
            (("C", "X"), 1 + 6),
            (("C", "Y"), 2 + 0),
            (("C", "Z"), 3 + 3),
        ]);
        score(data, scoring)
    }

    pub fn part2(data: String) -> i32 {
        // A, B, C rock, paper, scissor
        // X, Y, Z lose, draw, win
        // rock, paper, scissor = 1, 2, 3
        let scoring = HashMap::from([
            (("A", "X"), 3 + 0),
            (("A", "Y"), 1 + 3),
            (("A", "Z"), 2 + 6),
            (("B", "X"), 1 + 0),
            (("B", "Y"), 2 + 3),
            (("B", "Z"), 3 + 6),
            (("C", "X"), 2 + 0),
            (("C", "Y"), 3 + 3),
            (("C", "Z"), 1 + 6),
        ]);
        score(data, scoring)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solve_day01() {
        assert_eq!((70613, 205805), day01::solve(get_contents("input/1")))
    }

    #[test]
    fn solve_day02_part1() {
        assert_eq!(10310, day02::part1(get_contents("input/2")))
    }

    #[test]
    fn solve_day02_part2() {
        assert_eq!(14859, day02::part2(get_contents("input/2")))
    }
}
