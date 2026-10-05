use clap::{Parser, Subcommand};

/// A tiny piggy bank tracker for the terminal.
#[derive(Parser)]
#[command(name = "piggy", version, about)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Command>,
}

#[derive(Subcommand)]
pub enum Command {
    /// Create a new piggy bank
    New {
        name: String,
        /// Optional savings goal, e.g. 2000 or 149.99
        #[arg(short, long)]
        goal: Option<String>,
    },
    /// Delete a piggy bank
    Delete { name: String },
    /// Show all piggy banks (default)
    List,
    /// Put money into a piggy bank
    Add { name: String, amount: String },
    /// Take money out of a piggy bank
    Take { name: String, amount: String },
    /// Set or change the goal (omit AMOUNT to remove it)
    Goal { name: String, amount: Option<String> },
}