#![allow(unused)]

use clap::Parser;
mod commands;

fn main() {
    let parser = commands::Cli::parse();
    todo!("Handle each command");
    match parser.cmd {
        Some(commands::Command::Add { name, path }) => {}
        Some(commands::Command::Get { name }) => {}
        Some(commands::Command::Remove { name }) => {}
        Some(commands::Command::List) => {}
        None => {}
    }
}
