use std::collections::HashMap;
use structopt::StructOpt;

enum YahtzeeError {
  PathError(String),
  ParseError(String),
}

impl From<std::io::Error> for YahtzeeError {
  fn from(e: std::io::Error) -> Self {
    Self::PathError(format!("Path error: {}", e))
  }
}

impl From<std::num::ParseIntError> for YahtzeeError {
  fn from(e: std::num::ParseIntError) -> Self {
    Self::ParseError(format!("Parse error: {}", e))
  }
}

#[derive(StructOpt)]
struct Cli {
  #[structopt(short = "p", long = "path")]
  path: Option<std::path::PathBuf>,

  rolls: Vec<i64>,
}

fn rolls_for_path(path: std::path::PathBuf) -> Result<Vec<i64>, YahtzeeError> {
  let file_contents = std::fs::read_to_string(path)?;
  file_contents
    .lines()
    .map(|s| s.parse::<i64>())
    .collect::<Result<Vec<i64>, YahtzeeError>>()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
  let input = Cli::from_args();
  let rolls = match input.path {
    Some(p) => rolls_for_path(p)?,
    None => input.rolls,
  };
  let mut counts = HashMap::new();

  for roll in rolls {
    let count = counts.entry(roll).or_insert(0);
    *count += 1;
  }

  let mut highest_score = 0;
  for (roll, count) in counts {
    let score = roll * count;
    highest_score = if score > highest_score {
      score
    } else {
      highest_score
    };
  }

  println!("{}", highest_score);
  Ok(())
}
