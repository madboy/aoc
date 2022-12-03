use std::fs;

fn main() {
    println!("Hello, world!");
}

fn day01_part1(path: &str) -> i32 {
    let contents = fs::read_to_string(path).expect("File not found");

    let packs: Vec<i32> = contents
        .split("\r\n\r\n")
        .map(|pack| {
            pack.lines()
                .map(|calorie| calorie.parse::<i32>().expect("Should be a number"))
                .sum()
        })
        .collect();

    packs
        .iter()
        .max()
        .expect("We should have a max calorie value")
        .clone()
}

fn day01_part2(path: &str) -> i32 {
    let contents = fs::read_to_string(path).expect("File not found");

    let mut packs: Vec<i32> = contents
        .split("\r\n\r\n")
        .map(|pack| {
            pack.lines()
                .map(|calorie| calorie.parse::<i32>().expect("Should be a number"))
                .sum()
        })
        .collect();

    packs.sort();

    packs.iter().rev().take(3).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day01_part1_test() {
        assert_eq!(70613, day01_part1("input/1"))
    }

    #[test]
    fn day01_part2_test() {
        assert_eq!(205805, day01_part2("input/1"))
    }
}
