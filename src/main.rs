use std::fmt::{Display, Formatter};

use clap::{Parser, ValueEnum};
use clap::builder::PossibleValue;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Part {
    One,
    Two,
    Both,
}

impl ValueEnum for Part {
    fn value_variants<'a>() -> &'a [Self] {
        &[Part::One, Part::Two, Part::Both]
    }

    fn from_str(input: &str, ignore_case: bool) -> Result<Self, String> {
        let checked_input = if ignore_case { input.to_lowercase() } else { input.to_owned() };
        match checked_input.as_str() {
            "1" => Ok(Part::One),
            "2" => Ok(Part::Two),
            "both" => Ok(Part::Both),
            _ => Err(format!("invalid part: {}", input)),
        }
    }

    fn to_possible_value(&self) -> Option<PossibleValue> {
        match self {
            Part::One => Some(PossibleValue::new("1")),
            Part::Two => Some(PossibleValue::new("2")),
            Part::Both => Some(PossibleValue::new("both")),
        }
    }
}

impl Display for Part {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_possible_value().unwrap().get_name())
    }
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(long, conflicts_with = "day")]
    all: bool,

    #[arg(short, long, conflicts_with = "all")]
    day: Option<u8>,

    #[arg(short, long, default_value_t = Part::Both)]
    part: Part,
}

fn execute(day: u8, part: Part) {
    let part1 = part == Part::One || part == Part::Both;
    let part2 = part == Part::Two || part == Part::Both;
    match day {
        1 => day_01::solve(part1, part2),
        2 => day_02::solve(part1, part2),
        3 => day_03::solve(part1, part2),
        4 => day_04::solve(part1, part2),
        5 => day_05::solve(part1, part2),
        6 => day_06::solve(part1, part2),
        7 => day_07::solve(part1, part2),
        8 => day_08::solve(part1, part2),
        _ => panic!("invalid day: {}", day),
    }
}

fn main() {
    let args = Args::parse();
    if args.all {
        for day in 1..=8 {
            execute(day, args.part);
        }
    } else if let Some(day) = args.day {
        execute(day, args.part);
    }
}
