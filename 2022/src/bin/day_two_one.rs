#[derive(Debug, Clone, Copy)]
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

    fn versus(&self, opp: &Play) -> Outcome {
        match self {
            Self::Rock => match opp {
                Self::Rock => Outcome::Draw,
                Self::Paper => Outcome::Lose,
                Self::Scissors => Outcome::Win,
            },
            Self::Paper => match opp {
                Self::Rock => Outcome::Win,
                Self::Paper => Outcome::Draw,
                Self::Scissors => Outcome::Lose,
            },
            Self::Scissors => match opp {
                Self::Rock => Outcome::Lose,
                Self::Paper => Outcome::Win,
                Self::Scissors => Outcome::Draw,
            },
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

#[derive(Debug)]
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

fn main() -> std::io::Result<()> {
    let file = std::fs::read_to_string("files/input_day2")?;
    let mut total = 0;

    for line in file.lines() {
        let plays: Vec<Play> = line.splitn(2, ' ').map(Play::from).collect();
        let opponent = plays[0];
        let myself = plays[1];

        let result = myself.versus(&opponent);
        let score: usize = myself.value() + result.value();
        total += score;
    }

    println!("Your total score is: {}", total);
    Ok(())
}
