mod amount;
mod cli;
mod commands;
mod display;
mod storage;

use clap::Parser;
use cli::{Cli, Command};
use std::process;

fn run(cli: Cli) -> Result<(), String> {
    let mut banks = storage::load()?;
    match cli.command.unwrap_or(Command::List) {
        Command::List => {
            display::list(&banks);
            return Ok(());
        }
        Command::New { name, goal } => commands::create(&mut banks, &name, goal.as_deref())?,
        Command::Delete { name } => commands::delete(&mut banks, &name)?,
    }
    storage::save(&banks)
}

fn main() {
    if let Err(e) = run(Cli::parse()) {
        eprintln!("error: {e}");
        process::exit(1);
    }
}