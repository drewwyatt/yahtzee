use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
  rolls: Vec<i64>,
}

fn main() {
  let args = Cli::from_args();
  println!("rolls: {:?}", args.rolls);
}
