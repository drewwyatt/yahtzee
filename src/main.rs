use std::collections::HashMap;
use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
  #[structopt(short = "p", long = "path")]
  path: Option<std::path::PathBuf>,

  rolls: Vec<i64>,
}

fn rolls_for_path(path: std::path::PathBuf) -> Result<Vec<i64>, Box<dyn std::error::Error>> {
  let file_contents = std::fs::read_to_string(path)?;
  Ok(
    file_contents
      .lines()
      .map(|s| s.parse::<i64>().unwrap())
      .collect(),
  )
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
