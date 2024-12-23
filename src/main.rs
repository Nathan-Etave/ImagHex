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
            let result: Result<(), String> = encode::encode_file(&input, &output_location);

            match result {
                Ok(_) => {
                    println!(
                        "File encoded successfully to {}",
                        output_location.unwrap_or(String::from("."))
                    );
                }
                Err(e) => {
                    eprintln!("{}", e);
                }
            }
        }
        imaghex::handlers::cli::Commands::Decode {
            input,
            output_location,
        } => {
            let result: Result<(), String> = decode::decode_file(&input, &output_location);

            match result {
                Ok(_) => {
                    println!(
                        "File decoded successfully to {}",
                        output_location.unwrap_or(String::from("."))
                    );
                }
                Err(e) => {
                    eprintln!("{}", e);
                }
            }
        }
    }
}
