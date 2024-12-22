use clap::{Parser, Subcommand};

const AUTHOR: &str = "Etave Nathan";
const HELP_TEMPLATE: &str = "{name} {version} - {author}\n\n{about}\n\n{all-args}";
const APP_NAME: &str = "ImagHex";
const APP_ABOUT: &str =
    "A tool for encoding files into graphical representations of their bytes and vice versa";
const ENCODE_ABOUT: &str = "Turns a file into a graphical representation of its bytes";
const DECODE_ABOUT: &str = "Turns a graphical representation of bytes back into a file";

#[derive(Parser)]
#[clap(
    name = APP_NAME,
    about = APP_ABOUT,
    author = AUTHOR,
    version,
    help_template = HELP_TEMPLATE
)]
#[command(author, about, version)]
pub struct CLI {
    #[clap(subcommand)]
    pub subcommand: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    #[command(
        arg_required_else_help = true,
        about = ENCODE_ABOUT,
        author = AUTHOR,
        version,
        help_template = HELP_TEMPLATE
    )]
    Encode {
        #[arg(help = "The input file to be encoded")]
        input: String,
        #[arg(help = "The output location of the encoded file")]
        output_location: Option<String>,
    },
    #[command(
        arg_required_else_help = true,
        about = DECODE_ABOUT,
        author = AUTHOR,
        version,
        help_template = HELP_TEMPLATE
    )]
    Decode {
        #[arg(help = "The input file to be decoded")]
        input: String,
        #[arg(help = "The output location of the decoded file")]
        output_location: Option<String>,
    },
}
