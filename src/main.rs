use clap::Parser;
use imaghex::handlers::cli::CLI;

fn main() {
  let args: CLI = CLI::parse();

  match args.subcommand {
    imaghex::handlers::cli::Commands::Encode { input } => {
      println!("{}", input);
    },
    imaghex::handlers::cli::Commands::Decode { input } => {
      println!("{}", input);
    },
  }
}