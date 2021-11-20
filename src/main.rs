#[macro_use]
extern crate clap;

use clap::{crate_version, App, Arg};
use sample::reservoir_sample;
use std::env;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::path::{Path, PathBuf};

const STDIN_INPUT: &str = "-";

#[derive(Debug)]
enum Input {
    File(PathBuf),
    Stdin,
}

fn main() {
    let app = App::new("sample")
        .version(crate_version!())
        .about("Randomly sample lines")
        .arg(
            Arg::with_name("FILE")
                .help("File(s) to sample. Use '-' for standard input.")
                .long_help("File(s) to sample. Use a dash ('-') or no argument to read from standard input. \
                    Multiple files are sampled as a single population.")
                .takes_value(true)
                .multiple(true),
        )
        .arg(
            Arg::with_name("sample-size")
                .long("sample-size")
                .short("n")
                .takes_value(true)
                .default_value("10"),
        );
    let matches = app.get_matches();
    let n = value_t!(matches, "sample-size", u32).expect("invalid sample-size");

    // TODO: Extract all of this and test
    let inputs = {
        match matches.values_of_os("FILE").map(|vs| vs.map(Path::new)) {
            Some(filenames) => filenames
                .map(|s| {
                    if s.to_str().unwrap_or_default() == STDIN_INPUT {
                        Input::Stdin
                    } else {
                        Input::File(s.to_path_buf())
                    }
                })
                .collect(),
            None => vec![Input::Stdin],
        }
    };

    let lines = inputs.iter().flat_map(|inp| {
        let r: Box<dyn io::Read> = match inp {
            Input::Stdin => Box::new(io::stdin()),
            Input::File(ref fname) => {
                let f = File::open(fname).unwrap();
                Box::new(f)
            }
        };
        io::BufReader::new(r).lines().map(|l| l.unwrap())
    });
    let sampled = reservoir_sample(lines, n);
    for l in sampled {
        println!("{}", l);
    }
}
