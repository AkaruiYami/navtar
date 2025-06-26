#![allow(unused)]

use crate::workspace::Workspace;
use clap::{CommandFactory, Parser};
use std::{
    env,
    fs::{self, File, OpenOptions},
    io::{self, BufRead, BufReader, ErrorKind, Write},
    path::{Path, PathBuf},
    process,
};
mod commands;
mod workspace;

fn get_storage_path() -> PathBuf {
    PathBuf::from(env::var("NAVTAR_DIR").expect("Please set NAVTAR_DIR in environment variable."))
}

fn get_data_file_path() -> PathBuf {
    let root_path = get_storage_path();
    root_path.join(".data")
}

fn load_workspaces() -> Vec<Workspace> {
    let file = match File::open(get_data_file_path()) {
        Ok(f) => f,
        Err(e) if e.kind() == ErrorKind::NotFound => {
            let new_ws_vec: Vec<Workspace> = vec![];
            if let Err(er) = save_workspace(&new_ws_vec) {
                panic!("Failed to create default workspace file: {}", er);
            }
            File::open(get_data_file_path())
                .expect("Failed to reopen newly created workspace file.")
        }
        Err(e) => panic!("Failled to load the workspaces. Error: {}", e),
    };
    let reader = BufReader::new(file);
    let mut result: Vec<Workspace> = Vec::new();
    for line in reader.lines().map_while(Result::ok) {
        if let Some((name, path_string)) = line.split_once("-->") {
            let ws = Workspace::new(name.trim(), path_string.trim());
            result.push(ws);
        }
    }
    result
}

fn save_workspace(ws_vec: &[Workspace]) -> io::Result<File> {
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(get_data_file_path())?;
    for ws in ws_vec {
        writeln!(file, "{}-->{}", ws.name, ws.path.display())?;
    }
    file.flush()?;
    Ok(file)
}

fn append_workspace(ws: &Workspace) -> io::Result<&Workspace> {
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(get_data_file_path())?;

    writeln!(file, "{}-->{}", ws.name, ws.path.display())?;
    file.flush()?;
    Ok(ws)
}

fn validate_workspace(ws: &Workspace) -> Result<(), String> {
    if load_workspaces().iter().any(|ews| ews.name == ws.name) {
        return Err(format!(
            "Name '{}' already being used! Please try other name.",
            ws.name
        ));
    }
    if !ws.exists() {
        return Err(format!("\"{}\" does not exist!", ws.get_path_string()));
    }
    if !ws.is_dir() {
        return Err(format!("\"{}\" is not a directory!", ws.get_path_string()));
    }
    Ok(())
}

fn prompt_add_new(name: &str) {
    println!("'{}' does not exist!", name);
    println!();
    println!(
        "Would you want to register current directory as '{}' instead [Y/n]",
        name
    );

    use std::io::prelude::*;
    use std::io::{stdin, stdout};

    print!("> ");
    stdout().flush().unwrap();

    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let answer = input.trim().to_lowercase();

    if answer == "y" || answer == "yes" || answer.is_empty() {
        let current_dir = env::current_dir().expect("Unable to get current directory");
        let new_workspace = Workspace::new(name, current_dir.to_str().unwrap());
        try_register_workspace(&new_workspace);
    } else {
        println!("No workspace added.");
        process::exit(-1);
    }
}

fn try_register_workspace(ws: &Workspace) {
    match validate_workspace(ws) {
        Ok(_) => match append_workspace(ws) {
            Ok(ws) => {
                println!("[Added]: {} -> \"{}\"", ws.name, ws.path.display())
            }
            Err(e) => {
                println!("Failed to add '{}'. Error: {}", ws.name, e)
            }
        },
        Err(e) => {
            println!("Failed to add '{}'. Error: {}", ws.name, e)
        }
    }
}

fn main() {
    let parser = commands::Cli::parse();

    let mut all_workspace = load_workspaces();
    match parser.cmd {
        Some(commands::Command::Add { name, path }) => {
            let new_workspace = Workspace::new(&name, &path);
            try_register_workspace(&new_workspace);
        }
        Some(commands::Command::Get { name }) => {
            match all_workspace.iter().find(|ws| ws.name == name) {
                Some(ws) => println!("\"{}\"", ws.get_path_string()),
                None => prompt_add_new(&name),
            }
        }
        Some(commands::Command::Remove { name }) => {
            if let Some(idx) = all_workspace.iter().position(|ws| ws.name == name) {
                let removed_ws = all_workspace.remove(idx);
                println!("Removing '{}'...", removed_ws.name);
                match save_workspace(&all_workspace) {
                    Ok(v) => {
                        println!("[Removed]: '{}'", removed_ws.name);
                    }
                    Err(e) => {
                        print!("Removing operation failed. ");
                        println!("Cannot update the workspace database.");
                        println!("[Error]: {}", e);
                        all_workspace.insert(idx, removed_ws);
                    }
                }
            } else {
                println!("'{}' does not exist!", name);
            }
        }
        Some(commands::Command::List) => {
            for (i, ws) in all_workspace.iter().enumerate() {
                println!("[{}] '{}' = \"{}\"", i + 1, ws.name, ws.get_path_string());
            }
        }
        Some(commands::Command::Rename { old_name, new_name }) => {
            if let Some(idx) = all_workspace.iter().position(|ws| ws.name == old_name) {
                let old_path = &all_workspace[idx].get_path_string();
                let new_ws = Workspace::new(&new_name, old_path);
                match validate_workspace(&new_ws) {
                    Ok(_) => {
                        all_workspace[idx] = new_ws;
                        save_workspace(&all_workspace);
                        println!(
                            "Successfuly rename workspace '{}' -> '{}' : \"{}\"",
                            old_name, new_name, old_path
                        );
                    }
                    Err(e) => {
                        println!("Failed to rename. {}", e)
                    }
                }
            } else {
                println!("'{}' does not exist!", old_name);
            }
        }
        None => commands::Cli::command().print_help().unwrap(),
        _ => panic!("Command not found."),
    }
}
