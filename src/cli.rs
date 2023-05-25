use clap::{Parser, ValueEnum};

#[derive(Parser)]
#[command(
    author,
    version,
    about,
    about = "Concatenate FILE(s) to standard output."
)]
pub struct Cli {
    pub paths: Vec<String>,
    #[arg(value_enum, short, long, default_value_t = NumberingMode::None)]
    pub numbering_mode: NumberingMode,
    #[arg(short = 'E', long, default_value_t = false)]
    pub show_ends: bool,
}

#[derive(Clone, Copy, ValueEnum)]
pub enum NumberingMode {
    None,
    NonEmpty,
    All,
}

impl ToString for NumberingMode {
    fn to_string(&self) -> String {
        match self {
            NumberingMode::None => "none",
            NumberingMode::NonEmpty => "non_empty",
            NumberingMode::All => "all",
        }
        .into()
    }
}
