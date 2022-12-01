use aoc_runner_derive::{aoc, aoc_generator};

type Parsed = Vec<i64>;

#[aoc_generator(day1)]
fn parse_input(input: &str) -> Parsed {
    input
        .split("\n\n")
        .map(|s| {
            s
                .split("\n")
                .map(|s| s.parse::<i64>().expect("Failed to parse"))
                .sum()
        })
        .collect()
}

#[aoc(day1, part1)]
fn part1(input: &Parsed) -> i64 {
    input
        .iter()
        .max()
        .map(|&x| x)
        .expect("Failed to find max")
}

#[aoc(day1, part2)]
fn part2(input: &Parsed) -> i64 {
    let sorted_input = {
        let mut input = input.clone();
        input.sort();
        input
    };
    sorted_input
        .iter()
        .rev()
        .take(3)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample1() {
        let input = r#"1000
2000
3000

4000

5000
6000

7000
8000
9000

10000"#;
        assert_eq!(part1(&parse_input(input)), 24000);
        assert_eq!(part2(&parse_input(input)), 45000);
    }
}