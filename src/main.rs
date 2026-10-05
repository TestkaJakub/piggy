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
        Command::Add { name, amount } => commands::deposit(&mut banks, &name, &amount)?,
        Command::Take { name, amount } => commands::withdraw(&mut banks, &name, &amount)?,
        Command::Goal { name, amount } => commands::set_goal(&mut banks, &name, amount.as_deref())?,
    }
    storage::save(&banks)
}

fn main() {
    if let Err(e) = run(Cli::parse()) {
        eprintln!("error: {e}");
        process::exit(1);
    }
}