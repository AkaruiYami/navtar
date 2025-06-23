use std::{fmt, path::PathBuf};

#[derive(PartialEq, Eq)]
pub struct Workspace {
    pub name: String,
    pub path: PathBuf,
}

impl fmt::Display for Workspace {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} --> {}", self.name, self.path.display())
    }
}

impl Workspace {
    pub fn new(name: &str, path: &str) -> Self {
        let mut path_buf = PathBuf::from(path);
        let normalized_path = path_buf.canonicalize().unwrap_or(path_buf);
        Workspace {
            name: name.to_string(),
            path: normalized_path,
        }
    }

    pub fn get_path_string(&self) -> String {
        self.path
            .display()
            .to_string()
            .trim_start_matches(r"\\?\")
            .to_string()
    }

    pub fn exists(&self) -> bool {
        self.path.exists()
    }

    pub fn is_dir(&self) -> bool {
        self.path.is_dir()
    }

    pub fn is_file(&self) -> bool {
        self.path.is_file()
    }
}
