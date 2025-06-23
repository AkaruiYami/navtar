#![allow(unused)]

use crate::workspace::Workspace;
use clap::{CommandFactory, Parser};
use std::{
    env,
    fs::{self, File, OpenOptions},
    io::{self, BufRead, BufReader, ErrorKind, Write},
    path::{Path, PathBuf},
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
            let new_ws_vec: Vec<&Workspace> = vec![];
            if let Err(er) = save_workspace(&new_ws_vec) {
                panic!("Failed to create default workspace file: {}", er);
            }
            File::open(get_data_file_path())
                .expect("Failed to reopen newly created workspace file.")
        }
        Err(e) => panic!("Failled to load the workspaces."),
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

fn save_workspace(ws_vec: &[&Workspace]) -> io::Result<File> {
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
    // TODO: Implement duplication validation
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(get_data_file_path())?;
    writeln!(file, "{}-->{}", ws.name, ws.path.display())?;
    file.flush()?;
    Ok(ws)
}

fn main() {
    let parser = commands::Cli::parse();

    let all_workspace = load_workspaces();
    match parser.cmd {
        Some(commands::Command::Add { name, path }) => {
            let new_workspace = Workspace::new(&name, &path);
            match append_workspace(&new_workspace) {
                Ok(ws) => println!("[Added]: {} -> {}", ws.name, ws.path.display()),
                Err(e) => println!("Failed to add {}", new_workspace.name),
            }
        }
        Some(commands::Command::Get { name }) => {
            todo!() // TODO: Implement the retrieval of workspace path
        }
        Some(commands::Command::Remove { name }) => {
            todo!() // TODO: Implement the removing of the exisitng workspace
        }
        Some(commands::Command::List) => {
            for (i, ws) in all_workspace.iter().enumerate() {
                println!("[{}] {} = {}", i + 1, ws.name, ws.get_path_string());
            }
        }
        None => commands::Cli::command().print_help().unwrap(),
    }
}
