#[macro_use]
extern crate clap;

use chrono::prelude::*;
use colored::*;
use file_lock::FileLock;
use std::io::prelude::*;
use std::io::Error;
use std::process::{Command, Output};

#[derive(Debug)]
enum LockError {
    Argument(),
    Io(std::io::Error),
}

impl From<Error> for LockError {
    fn from(error: Error) -> Self {
        LockError::Io(error)
    }
}

fn value_of<'a>(matches: &'a clap::ArgMatches, name: &str) -> Result<&'a str, LockError> {
    match matches.value_of(name) {
        None => Err(LockError::Argument()),
        Some(v) => Ok(v),
    }
}

fn command(cmd: &str) -> Result<Output, Error> {
    let cmd: Vec<&str> = cmd.split_whitespace().collect();
    Command::new(cmd[0]).args(&cmd[1..]).output()
}

fn main() -> Result<(), LockError> {
    let matches: clap::ArgMatches = clap_app!(app =>
                            (version: "0.0.1")
                            (@arg CMD: -c --command <command> +takes_value "Command to run")
                            (@arg LOCK: -l --lock <file> +takes_value "File to use as lock")
    )
    .get_matches();

    let cmd = value_of(&matches, "CMD")?;
    let lock = value_of(&matches, "LOCK")?;

    println!("** Acquiring lock file: {} **", lock.blue());
    let mut file_lock = FileLock::lock(lock, true, true)?;

    println!("** Executing command: {} **", cmd.yellow());
    let output = command(cmd)?;
    std::io::stdout().write_all(&output.stdout)?;

    file_lock
        .file
        .write_all(format!("command[{}] - last run[{:?}]", cmd, Local::now()).as_bytes())?;

    Ok(())
}
