use clap::{Parser, Subcommand};

#[derive(Parser)]
#[clap(
    name = "ImagHex",
    about = "A tool for encoding files into graphical representations of their bytes and vice versa",
    author = "Etave Nathan",
    version,
    help_template = "{name} {version} - {author}\n\n{about}\n\n{all-args}",
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
        about = "Turns a file into a graphical representation of its bytes",
        author = "Etave Nathan",
        version,
        help_template = "{name} {version} - {author}\n\n{about}\n\n{all-args}",
    )]
    Encode {
        #[arg(help = "The input file to be encoded")]
        input: String,
    },
    #[command(
        arg_required_else_help = true,
        about = "Turns a graphical representation of bytes back into a file",
        author = "Etave Nathan",
        version,
        help_template = "{name} {version} - {author}\n\n{about}\n\n{all-args}",
    )]
    Decode {
        #[arg(help = "The input file to be decoded")]
        input: String,
    },
}