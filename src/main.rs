use clap::Parser;
use imaghex::handlers::cli::CLI;

use imaghex::processors::decode;
use imaghex::processors::encode;

fn main() {
    let args: CLI = CLI::parse();

    match args.subcommand {
        imaghex::handlers::cli::Commands::Encode {
            input,
            output_location,
        } => {
            encode::encode_file(&input, output_location);
        }
        imaghex::handlers::cli::Commands::Decode {
            input,
            output_location,
        } => {
            decode::decode_file(&input, output_location);
        }
    }
}
