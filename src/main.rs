#[macro_use]
extern crate clap;

#[derive(Debug)]
struct LockError;

fn value_of<'a>(matches: &'a clap::ArgMatches, name: &str) -> Result<&'a str, LockError> {
    match matches.value_of(name) {
        None => Err(LockError),
        Some(v) => Ok(v),
    }
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

    println!("{} - {}", cmd, lock);

    Ok(())
}
