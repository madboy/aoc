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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solve_day01() {
        assert_eq!((70613, 205805), day01::solve(get_contents("input/1")))
    }
}
