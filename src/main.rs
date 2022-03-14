use std::env;
use std::error::Error;
use std::fmt;
use std::fs::File;
use std::io::{self, BufRead, Write};
use std::path::Path;
use std::str::FromStr;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    let input_file = &args[1];
    let output_file = &args[2];
    let mut output_file = File::create(output_file)?;

    let mut prev_line = String::new();
    let mut counter: u32 = 1;
    if let Ok(lines) = read_lines(input_file) {
        for line in lines.flatten() {
            if line.eq(&prev_line) {
                continue;
            } else {
                prev_line = line.clone();
            }

            let mut entries = line.split_whitespace();
            let name = parse::<String>("name", entries.next())?;
            let date_init = parse::<String>("date_init", entries.next())?;
            let time_init = parse::<String>("time_init", entries.next())?;
            let date_fin = parse::<String>("date_fin", entries.next())?;
            let time_fin = parse::<String>("time_fin", entries.next())?;

            if date_init.eq(&date_fin) && time_init.eq(&time_fin) {
                continue;
            }

            counter += 1;

            let log = Log {
                name,
                date_init,
                time_init,
                date_fin,
                time_fin,
            };

            writeln!(&mut output_file, "고에너지물리계산 박찬범 {log}")?;
        }
    }

    println!("{} entries have been parsed.", counter);

    Ok(())
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[derive(Debug)]
struct Log {
    name: String,
    date_init: String,
    time_init: String,
    date_fin: String,
    time_fin: String,
}

impl fmt::Display for Log {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} IBS CTPU {} {} {} {}",
            self.name,
            self.date_init,
            self.time_init,
            self.date_fin,
            self.time_fin
        )
    }
}

#[derive(Debug)]
enum ParserError {
    MissingEntry(String),
    ConversionError(String),
}

impl Error for ParserError {}

impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use self::ParserError::*;
        match self {
            MissingEntry(entry) => write!(f, "Missing entry '{}'", entry),
            ConversionError(entry) => {
                write!(f, "Failed to convert to a number: '{}'", entry)
            }
        }
    }
}

fn parse<T: FromStr>(
    name: &str, text: Option<&str>,
) -> Result<T, Box<dyn Error>> {
    let text: &str = text.ok_or_else(|| {
        Box::new(ParserError::MissingEntry(String::from(name)))
    })?;
    match text.parse::<T>() {
        Ok(t) => Ok(t),
        Err(_) => Err(Box::new(ParserError::ConversionError(text.to_owned()))),
    }
}
