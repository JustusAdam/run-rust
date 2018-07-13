# `run_rust`, a simple utility to execute a rust source file directly

`run_rust` makes it just a little bit easier to use rust for scripting. Simply
call it on a rust source file that has a `main` function and have it execute.

## Installing

You can install this program simply by cloning the repository and installing
it using `cargo`.

## Usage

Supports passing arguments to the script as well as choosing the `rustc`
executable to use.

Use the `--help` option to explore the available flags and arguments.

```
run_rust 0.1.0
Justus Adam <dev@justus.science>
Run rust source files directly

USAGE:
    run_rust [OPTIONS] <SOURCE_FILE> [-- <SCRIPT_ARGS>...]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
        --compiler <COMPILER>    Path to the rustc executable to use for compilation

ARGS:
    <SOURCE_FILE>       Source file to run
    <SCRIPT_ARGS>...    Additional arguments to pass to the script
```

### Shebang

You can use this also to make rust sources themselves executable. Add
`#!/usr/bin/env run_rust` as first line in your script. Or, if you know the
exact installation location `#!/exact/path/to/run_rust` will also work.

## How it works

Its actually a rather naive program which does nothing more than compile your
source code to a temporary directory and execute it.

### Limitations

It currently only supports using the standard library, and no additional crates
and libraries.

## Why?

This utility exists because I wanted to use rust for scripting. The standard
library is rather good, reminiscent of python in my opinion, but unlike python
there is a compiler that catches all the horrible mistakes I make all the time.
However running rust code as a script always requires to compile an actual
binary file, which is clutter to me and also takes up space.
