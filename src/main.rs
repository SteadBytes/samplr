use clap::{crate_version, App, Arg};
use sample::sample;
use std::env;
use std::io;
use std::io::prelude::*;

fn main() {
    let app = App::new("sample").version(crate_version!()).arg(
        Arg::with_name("sample-size")
            .long("sample-size")
            .short("n")
            .takes_value(true)
            .default_value("10"),
    );

    let matches = app.get_matches();
    let n: u32 = matches
        .value_of("sample-size")
        .unwrap()
        .parse()
        .expect("invalid sample-size");

    let stdin = io::stdin();
    let sampled = sample(stdin.lock().lines().map(|l| l.unwrap()), n);
    for l in sampled {
        println!("{}", l);
    }
}
