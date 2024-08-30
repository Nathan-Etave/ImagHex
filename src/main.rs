use clap::Parser;
use imaghex::handlers::cli::CLI;

fn main() {
  let args: CLI = CLI::parse();

  match args.subcommand {
    imaghex::handlers::cli::Commands::Encode { input, output_location } => {
      println!("{}, {}", input, output_location.unwrap_or(String::from("./")));
    },
    imaghex::handlers::cli::Commands::Decode { input, output_location } => {
      println!("{}, {}", input, output_location.unwrap_or(String::from("./")));
    },
  }
}