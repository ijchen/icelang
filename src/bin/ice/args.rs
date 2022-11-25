use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    /// The path to a text file containing icelang code
    pub file_path: Option<String>,

    // Whether or not to show additional debug information
    #[clap(short, long)]
    pub debug_info: bool,
}
