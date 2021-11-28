#[macro_use]
extern crate clap;

use clap::{crate_version, App, Arg, ArgMatches, OsValues};
use samplr::reservoir_sample;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::{self, stdout};
use std::path::{Path, PathBuf};

const STDIN_INPUT: &str = "-";

#[derive(Debug, PartialEq)]
enum Input {
    File(PathBuf),
    Stdin,
}

fn inputs_from_arg_values(values: Option<OsValues>) -> Vec<Input> {
    match values.map(|vs| vs.map(Path::new)) {
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
}

struct Sample {
    inputs: Vec<Input>,
    n: u32,
    seed: Option<u64>,
}

impl Sample {
    fn from_arg_matches(matches: ArgMatches) -> Self {
        let n = value_t!(matches, "sample-size", u32).expect("invalid sample-size");
        let seed = matches
            .value_of("seed")
            .map(|x| x.parse().expect("invalid seed"));

        let inputs = inputs_from_arg_values(matches.values_of_os("FILE"));
        Sample { inputs, n, seed }
    }

    pub fn run(&self, w: &mut impl Write) {
        let lines = self.inputs.iter().flat_map(|inp| {
            let r: Box<dyn io::Read> = match inp {
                Input::Stdin => Box::new(io::stdin()),
                Input::File(ref fname) => {
                    let f = File::open(fname).unwrap();
                    Box::new(f)
                }
            };
            io::BufReader::new(r).lines().map(|l| l.unwrap())
        });
        let sampled = reservoir_sample(lines, self.n, self.seed);
        for l in sampled {
            writeln!(w, "{}", l).unwrap();
        }
    }
}

fn build_app() -> App<'static, 'static> {
    App::new("sample")
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
        ).arg(
            Arg::with_name("seed")
                .help("RNG seed")
                .long("seed")
                .takes_value(true)
        )
}

fn main() {
    let app = build_app();
    let matches = app.get_matches();
    Sample::from_arg_matches(matches).run(&mut stdout());
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;
    use Input::*;

    // TODO: Invalid cases e.g. --sample-size = not_a_number

    #[test_case(vec![] => vec![Stdin] ; "implicit stdin")]
    #[test_case(vec!["-"] => vec![Stdin] ; "explicit stdin")]
    #[test_case(vec!["things.txt"] => vec![File("things.txt".into())] ; "single file")]
    #[test_case(
        vec!["things.txt", "other_things.txt"]
        =>
        vec![File("things.txt".into()), File("other_things.txt".into())]
        ;
        "multiple files"
    )]
    #[test_case(
        vec!["things.txt", "-", "other_things.txt"]
        =>
        vec![File("things.txt".into()), Stdin, File("other_things.txt".into())]
        ;
        "multiple files and stdin"
    )]
    fn inputs(args: Vec<&str>) -> Vec<Input> {
        let raw_args = vec!["sample"].into_iter().chain(args);
        let app = build_app();
        let matches = app.get_matches_from(raw_args);
        let sample = Sample::from_arg_matches(matches);
        sample.inputs
    }

    #[test_case(vec![] => 10; "default")]
    #[test_case(vec!["-n", "20"] => 20; "short")]
    #[test_case(vec!["--sample-size", "20"] => 20; "long")]
    fn sample_size(args: Vec<&str>) -> u32 {
        let raw_args = vec!["sample"].into_iter().chain(args);
        let app = build_app();
        let matches = app.get_matches_from(raw_args);
        let sample = Sample::from_arg_matches(matches);
        sample.n
    }

    #[test_case(vec![] => None; "default")]
    #[test_case(vec!["--seed", "20"] => Some(20); "long")]
    fn seed(args: Vec<&str>) -> Option<u64> {
        let raw_args = vec!["sample"].into_iter().chain(args);
        let app = build_app();
        let matches = app.get_matches_from(raw_args);
        let sample = Sample::from_arg_matches(matches);
        sample.seed
    }
}
