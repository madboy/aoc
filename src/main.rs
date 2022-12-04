use std::fs;

fn main() {
    println!("{}", get_contents("input/hello"));
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

        (packs.clone().take(1).sum(), packs.clone().take(3).sum())
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
            (("A", "Z"), 3),
            (("B", "X"), 1),
            (("B", "Y"), 2 + 3),
            (("B", "Z"), 3 + 6),
            (("C", "X"), 1 + 6),
            (("C", "Y"), 2),
            (("C", "Z"), 3 + 3),
        ]);
        score(data, scoring)
    }

    pub fn part2(data: String) -> i32 {
        // A, B, C rock, paper, scissor
        // X, Y, Z lose, draw, win
        // rock, paper, scissor = 1, 2, 3
        let scoring = HashMap::from([
            (("A", "X"), 3),
            (("A", "Y"), 1 + 3),
            (("A", "Z"), 2 + 6),
            (("B", "X"), 1),
            (("B", "Y"), 2 + 3),
            (("B", "Z"), 3 + 6),
            (("C", "X"), 2),
            (("C", "Y"), 3 + 3),
            (("C", "Z"), 1 + 6),
        ]);
        score(data, scoring)
    }
}

pub mod day03 {
    use std::collections::HashMap;

    static PRIORITIES: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

    fn get_priority(c: &char) -> usize {
        match PRIORITIES.find(*c) {
            Some(val) => val + 1,
            _ => {
                eprint!("Did not find char in priorities {}", c);
                usize::MAX
            }
        }
    }

    pub fn part1(data: String) -> usize {
        let mut priority = 0;
        for line in data.lines() {
            let mut seen = HashMap::new();
            let length = line.len();
            for (i, c) in line.chars().enumerate() {
                if i < (length / 2) {
                    seen.insert(c, 1);
                } else {
                    match seen.get(&c) {
                        Some(_item) => {
                            priority += get_priority(&c);
                            break;
                        }
                        _ => continue,
                    }
                }
            }
        }
        priority
    }

    pub fn part2(data: String) -> usize {
        let mut priority = 0;
        let mut group_member = 1;
        let mut first_member = HashMap::new();
        let mut second_member = HashMap::new();
        for line in data.lines() {
            for c in line.chars() {
                if group_member == 1 {
                    first_member.insert(c, 1);
                } else if group_member == 2 {
                    match first_member.get(&c) {
                        Some(_item) => {
                            second_member.insert(c, 1);
                            continue;
                        }
                        _ => continue,
                    }
                } else if group_member == 3 {
                    match second_member.get(&c) {
                        Some(_item) => {
                            priority += get_priority(&c);
                            break;
                        }
                        _ => continue,
                    }
                }
            }
            group_member += 1;
            if group_member > 3 {
                group_member = 1;
                first_member.clear();
                second_member.clear();
            }
        }
        priority
    }
}

pub mod day04 {
    fn fully_contains(bounds: &[i32; 4]) -> bool {
        let &[fl, fh, sl, sh] = bounds;
        if (fl <= sl && fh >= sh) || (fh <= sh && fl >= sl) {
            return true;
        }
        false
    }

    fn overlaps(bounds: &[i32; 4]) -> bool {
        let &[fl, fh, sl, sh] = bounds;
        if fh < sl || fl > sh {
            return false;
        }
        true
    }

    pub fn solve(data: String) -> (i32, i32) {
        let mut bounds: [i32; 4] = [0; 4];
        let mut contained = 0;
        let mut overlap = 0;
        for line in data.lines() {
            let pairs = line.split(',');
            let mut idx = 0;
            for pair in pairs {
                let upper: Vec<i32> = pair
                    .split('-')
                    .map(|bound| bound.parse::<i32>().expect("I want only numbers"))
                    .collect();
                bounds[idx] = upper[0];
                bounds[idx + 1] = upper[1];
                idx += 2;
            }
            if fully_contains(&bounds) {
                contained += 1;
            }

            if overlaps(&bounds) {
                overlap += 1;
            }
        }
        (contained, overlap)
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

    #[test]
    fn solve_day03_part1() {
        assert_eq!(8109, day03::part1(get_contents("input/3")))
    }

    #[test]
    fn solve_day03_part2() {
        assert_eq!(2738, day03::part2(get_contents("input/3")))
    }

    #[test]
    fn solve_day04() {
        assert_eq!((605, 914), day04::solve(get_contents("input/4")))
    }
}
