use std::iter::Step;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Play {
    Rock,
    Paper,
    Scissors,
}

impl Play {
    fn value(&self) -> usize {
        match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3,
        }
    }

    fn loses_to(&self) -> Self {
        match self {
            Self::Rock => Self::Paper,
            Self::Paper => Self::Scissors,
            Self::Scissors => Self::Rock,
        }
    }

    fn wins_against(&self) -> Self {
        match self {
            Self::Rock => Self::Scissors,
            Self::Paper => Self::Rock,
            Self::Scissors => Self::Paper,
        }
    }
}

impl From<&str> for Play {
    fn from(value: &str) -> Self {
        match value {
            "A" | "X" => Self::Rock,
            "B" | "Y" => Self::Paper,
            "C" | "Z" => Self::Scissors,
            _ => panic!("Failed to parse the play into rock,paper, or scissors."),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Outcome {
    Win,
    Lose,
    Draw,
}

impl Outcome {
    fn value(&self) -> usize {
        match self {
            Self::Win => 6,
            Self::Lose => 0,
            Self::Draw => 3,
        }
    }
}

impl From<&str> for Outcome {
    fn from(value: &str) -> Self {
        match value {
            "X" => Self::Lose,
            "Y" => Self::Draw,
            "Z" => Self::Win,
            _ => panic!("Failed to parse the play into rock,paper, or scissors."),
        }
    }
}

fn main() -> std::io::Result<()> {
    let file = std::fs::read_to_string("files/input_day2")?;
    let mut total = 0;

    for line in file.lines() {
        let game: Vec<&str> = line.splitn(2, ' ').collect();
        let opponent = Play::from(game[0]);
        let result = Outcome::from(game[1]);

        let myself = if result == Outcome::Draw {
            opponent
        } else if result == Outcome::Win {
            opponent.loses_to()
        } else {
            opponent.wins_against()
        };
        let score: usize = myself.value() + result.value();
        total += score;
    }

    println!("Your total score is: {}", total);
    Ok(())
}
