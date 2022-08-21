use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};

use clap::{App, Arg};

#[derive(Debug)]
struct Config {
    files: Vec<String>,
    add_number_to_lines: bool,
}

fn main() {
    let matches = App::new("catr")
        .version("0.1.0")
        .arg(
            Arg::new("files")
                .value_name("FILE")
                .multiple_values(true)
                .default_value("-")
                .help("Input file(s)")
        )
        .arg(
            Arg::new("add_number_to_lines")
                .short('n')
                .long("number")
                .help("Add number to lines")
                .takes_value(false)
        )
        .get_matches();
    let files: Vec<String> = matches
        .get_many::<String>("files")
        .unwrap()
        .map(|v| v.to_owned())
        .collect();

    let add_number_to_lines = matches.is_present("add_number_to_lines");
    let config = Config { files, add_number_to_lines };
    for file_name in config.files {
        let buf: Box<dyn BufRead> = match file_name.as_ref() {
            "-" => Box::new(BufReader::new(io::stdin())),
            _ => Box::new(BufReader::new(File::open(file_name).unwrap()))
        };
        if config.add_number_to_lines {
            let mut n = 1_u64;
            for line in buf.lines() {
                let line = line.unwrap();
                println!("{:>6}\t{}", n, line);
                n += 1;
            }
        } else {
            for line in buf.lines() {
                println!("{}", line.unwrap());
            }
        }
    }
}
