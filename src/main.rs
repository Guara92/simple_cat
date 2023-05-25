mod cli;

use std::{
    fs::File,
    io::{self, BufRead, Read, StdoutLock, Write},
};

use clap::Parser;

const BUFFER_SIZE: usize = 1024 * 64;

struct OutputOptions {
    numbering_mode: NumberingMode,
    show_ends: bool,
}

impl OutputOptions {
    fn is_simple(&self) -> bool {
        matches!(
            (self.numbering_mode, self.show_ends),
            (NumberingMode::None, false)
        )
    }
}

impl From<&cli::Cli> for OutputOptions {
    fn from(cli_options: &cli::Cli) -> Self {
        Self {
            numbering_mode: (&cli_options.numbering_mode).into(),
            show_ends: cli_options.show_ends,
        }
    }
}

#[derive(Clone, Copy)]
enum NumberingMode {
    None,
    NonEmpty,
    All,
}

impl From<&cli::NumberingMode> for NumberingMode {
    fn from(mode: &cli::NumberingMode) -> Self {
        match mode {
            cli::NumberingMode::None => Self::None,
            cli::NumberingMode::NonEmpty => Self::NonEmpty,
            cli::NumberingMode::All => Self::All,
        }
    }
}

fn main() -> io::Result<()> {
    let cli = cli::Cli::parse();
    let output_options: OutputOptions = (&cli).into();
    concat_paths(cli.paths, output_options)
}

fn concat_paths(paths: Vec<String>, output_options: OutputOptions) -> io::Result<()> {
    for path in paths {
        cat_path(path, &output_options)?
    }
    Ok(())
}

fn cat_path(path: String, output_options: &OutputOptions) -> io::Result<()> {
    let mut file = File::open(path)?;

    if output_options.is_simple() {
        print_simple(&mut file)
    } else {
        print_with_options(&mut file, output_options)
    }
}

fn print_simple(file: &mut File) -> io::Result<()> {
    let stdout = io::stdout();
    let mut stdout_lock = stdout.lock();
    let mut buf = [0; BUFFER_SIZE];
    while let Ok(n) = file.read(&mut buf) {
        if n == 0 {
            break;
        }
        stdout_lock.write_all(&buf[..n])?;
    }
    Ok(())
}

fn print_with_options(file: &mut File, output_options: &OutputOptions) -> Result<(), io::Error> {
    let lines = io::BufReader::with_capacity(BUFFER_SIZE, file).lines();

    for (line_number, line) in lines.enumerate() {
        print_line(&mut line?, line_number, output_options)?
    }
    Ok(())
}

fn print_line(
    line: &mut String,
    line_number: usize,
    output_options: &OutputOptions,
) -> Result<(), io::Error> {
    let stdout = io::stdout();
    let mut stdout_lock = stdout.lock();
    match output_options.numbering_mode {
        NumberingMode::None => {}
        NumberingMode::NonEmpty => {
            if !line.is_empty() {
                print_line_number(&mut stdout_lock, line_number)?
            }
        }
        NumberingMode::All => print_line_number(&mut stdout_lock, line_number)?,
    }

    if output_options.show_ends && line.is_empty() {
        line.push_str("     $\n")
    } else if output_options.show_ends {
        line.push('$');
        line.push('\n')
    }
    stdout_lock.write_all(line.as_bytes())
}

fn print_line_number(stdout_lock: &mut StdoutLock, n: usize) -> io::Result<()> {
    write!(stdout_lock, "{0:6}\t", n)
}
