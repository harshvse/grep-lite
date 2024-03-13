extern crate clap;
extern crate regex;

use clap::{App, Arg};
use regex::Regex;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;

fn process_line<T: BufRead + Sized>(reader: T, re: Regex) {
    for line_ in reader.lines() {
        let line = line_.unwrap();
        match re.find(&line) {
            Some(_) => println!("{:?}", line),
            None => (),
        }
    }
}

fn main() {
    let args = App::new("grep-lite")
        .version("0.01")
        .about("An app that work like grep with limited features")
        .arg(
            Arg::with_name("pattern")
                .help("The pattern to search for")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("input")
                .help("input file to search through")
                .takes_value(true)
                .required(false),
        )
        .get_matches();

    let pattern = args.value_of("pattern").unwrap();
    let search_pattern = Regex::new(pattern).unwrap();

    let input = args.value_of("input").unwrap_or("-");
    if input == "-" {
        let stdin = io::stdin();
        let reader = stdin.lock();
        process_line(reader, search_pattern)
    } else {
        let f: File = File::open(input).unwrap();
        let reader = BufReader::new(f);
        process_line(reader, search_pattern)
    }
}
