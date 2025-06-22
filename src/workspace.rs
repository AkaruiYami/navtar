use std::path::PathBuf;

pub struct Workspace {
    pub name: String,
    pub path: PathBuf,
}

impl PartialEq for Workspace {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.path == other.path
    }
}

impl Workspace {
    pub fn new(name: &str, path: &str) -> Self {
        let mut path_buf = PathBuf::new();
        path_buf.push(path);
        Workspace {
            name: name.to_string(),
            path: path_buf,
        }
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
