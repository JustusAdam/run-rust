#[macro_use]
extern crate clap;
extern crate rand;

use clap::{AppSettings, Arg};
use std::process::Command;

const SOURCE_FILE_ARG: &'static str = "SOURCE_FILE";
const COMPILER_ARG: &'static str = "COMPILER";
const SCRIPT_ARGS: &'static str = "SCRIPT_ARGS";

fn main() {
    let matches = app_from_crate!()
        .setting(AppSettings::TrailingVarArg)
        .arg(
            Arg::with_name(SOURCE_FILE_ARG)
                .index(1)
                .required(true)
                .help("Source file to run"),
        )
        .arg(
            Arg::with_name(COMPILER_ARG)
                .long("compiler")
                .takes_value(true)
                .help("Path to the rustc executable to use for compilation"),
        )
        .arg(
            Arg::with_name(SCRIPT_ARGS)
                .last(true)
                .multiple(true)
                .help("Additional arguments to pass to the script"),
        )
        .get_matches();
    let file = matches.value_of(SOURCE_FILE_ARG).unwrap();
    let compiler = matches.value_of(COMPILER_ARG).unwrap_or("rustc");
    let mut fname = std::env::temp_dir();
    fname.push(format!("{}", rand::random::<u16>()));
    let compilation_result = Command::new(compiler)
        .args(&[file, "-o", fname.to_str().unwrap()])
        .status()
        .unwrap();
    if !compilation_result.success() {
        eprintln!("Compilation failed, aborting.");
        return ();
    }
    let status = Command::new(fname)
        .args(matches.values_of(SCRIPT_ARGS).unwrap_or_default())
        .status()
        .unwrap();
    if !status.success() {
        eprint!("Program finished unsuccessfully");
        match status.code() {
            None => eprintln!(" by signal"),
            Some(c) => eprintln!(" with exit code {}", c),
        }
    }
}
