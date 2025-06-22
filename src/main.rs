#![allow(unused)]

use std::{
    collections::HashMap,
    env,
    fs::{self, File},
    io::{self, BufRead, BufReader},
    path::{Path, PathBuf},
};

use clap::{CommandFactory, Parser};
mod commands;

struct Workspace {
    name: String,
    path: PathBuf,
}

impl Workspace {
    fn new(name: &str, path: &str) -> Workspace {
        let mut path_buf = PathBuf::new();
        path_buf.push(path);
        Workspace {
            name: name.to_string(),
            path: path_buf,
        }
    }

    fn exists(&self) -> bool {
        self.path.exists()
    }

    fn is_dir(&self) -> bool {
        self.path.is_dir()
    }

    fn is_file(&self) -> bool {
        self.path.is_file()
    }
}

fn get_storage_path() -> PathBuf {
    PathBuf::from(env::var("NAVTAR_DIR").expect("Please set NAVTAR_DIR in environment variable."))
}

fn load_workspaces() -> Vec<Workspace> {
    let root_path = get_storage_path();

    // TODO: Handle if the file is not there (create default)
    let file = File::open(root_path.join(".data")).unwrap();
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

fn main() {
    let parser = commands::Cli::parse();

    let all_workspace = load_workspaces();
    match parser.cmd {
        Some(commands::Command::Add { name, path }) => {
            todo!() // TODO: Implement registering new workspace
        }
        Some(commands::Command::Get { name }) => {
            todo!() // TODO: Implement the retrieval of workspace path
        }
        Some(commands::Command::Remove { name }) => {
            todo!() // TODO: Implement the removing of the exisitng workspace
        }
        Some(commands::Command::List) => {
            for (i, ws) in all_workspace.iter().enumerate() {
                println!("[{}] {} = {}", i + 1, ws.name, ws.path.to_str().unwrap());
            }
        }
        None => commands::Cli::command().print_help().unwrap(),
    }
}
