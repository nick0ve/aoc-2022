use aoc_runner_derive::{aoc, aoc_generator};

// Enum for rock paper scissors
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum RPS {
    Rock,
    Paper,
    Scissors,
}

enum RPSResult {
    Win,
    Lose,
    Draw,
}

impl RPSResult {
    fn from(a: &RPS, b: &RPS) -> RPSResult {
        match (a, b) {
            (RPS::Rock, RPS::Scissors) => RPSResult::Win,
            (RPS::Paper, RPS::Rock) => RPSResult::Win,
            (RPS::Scissors, RPS::Paper) => RPSResult::Win,
            (RPS::Rock, RPS::Paper) => RPSResult::Lose,
            (RPS::Paper, RPS::Scissors) => RPSResult::Lose,
            (RPS::Scissors, RPS::Rock) => RPSResult::Lose,
            _ => RPSResult::Draw,
        }
    }

    fn from_str(s: &str) -> RPSResult {
        match s {
            "X" => RPSResult::Win,
            "Y" => RPSResult::Lose,
            "Z" => RPSResult::Draw,
            _ => panic!("Invalid RPSResult"),
        }
    }

    fn points(&self) -> u64 {
        match self {
            RPSResult::Win => 6,
            RPSResult::Lose => 0,
            RPSResult::Draw => 3,
        }
    }
}

impl RPS {
    fn points(&self) -> u64 {
        match self {
            RPS::Rock => 1,
            RPS::Paper => 2,
            RPS::Scissors => 3,
        }
    }

    fn get_winner(a: &RPS) -> RPS {
        match a {
            RPS::Rock => RPS::Paper,
            RPS::Paper => RPS::Scissors,
            RPS::Scissors => RPS::Rock,
        }
    }

    fn get_loser(a: &RPS) -> RPS {
        match a {
            RPS::Rock => RPS::Scissors,
            RPS::Paper => RPS::Rock,
            RPS::Scissors => RPS::Paper,
        }
    }

}

impl TryFrom<char> for RPS {
    type Error = ();

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'X' => Ok(RPS::Rock),
            'Y' => Ok(RPS::Paper),
            'Z' => Ok(RPS::Scissors),
            'A' => Ok(RPS::Rock),
            'B' => Ok(RPS::Paper),
            'C' => Ok(RPS::Scissors),
            _ => Err(()),
        }
    }
}

type Parsed = Vec<Vec<RPS>>;
type Parsed2 = u64;

#[aoc_generator(day2, part1)]
fn parse_input_1(input: &str) -> Parsed {
    input
        .split("\n")
        .map(|l| {
            l
                .split(" ")
                .map(|s| s.chars().next().unwrap())
                .map(|ch| RPS::try_from(ch).unwrap())
                .collect()
        })
        .collect()
}

#[aoc_generator(day2, part2)]
fn parse_input_2(input: &str) -> u64 {
    input
        .split("\n")
        .map(|l| {
            let mut iter = l.split(" ");
            let a = RPS::try_from(iter.next().unwrap().chars().next().unwrap()).unwrap();
            let result = RPSResult::from_str(iter.next().unwrap());
            let b = match result {
                RPSResult::Win => RPS::get_winner(&a),
                RPSResult::Lose => RPS::get_loser(&a),
                RPSResult::Draw => a,
            };
            b.points() + result.points()
        })
        .sum()
}

#[aoc(day2, part1)]
fn part1(input: &Parsed) -> u64 {
    input
        .iter()
        .map(|l| {
            l[1].points() + RPSResult::from(&l[1], &l[0]).points()
        })
        .sum()
}

#[aoc(day2, part2)]
fn part2(input: &u64) -> u64 {
    *input
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample1() {
        let input = r#"A Y
B X
C Z"#;
        //println!("{:?}", parse_input(input));
        assert_eq!(part1(&parse_input_1(input)), 15);
        assert_eq!(part2(&parse_input_2(input)), 12);
    }
}