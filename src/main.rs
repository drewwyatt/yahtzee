use std::collections::HashMap;
use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
  rolls: Vec<i64>,
}

fn main() {
  let rolls = Cli::from_args().rolls;
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

  println!("{}", highest_score)
}
