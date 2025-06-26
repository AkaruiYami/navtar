use clap::{Parser, Subcommand};

/// A CLI tool to manage workspace directory powered by Rust.
///
/// This tool used to quickly navigate to your workspace directory
/// without the need to put lengthy path when trying to cd into it.
/// By just using using the unique workspace ID, we can cd into it.
/// The ID can be reistered using the Add command. However, if you try
/// to navigate into a workspace using ID that is not registered, it
/// will assume that you are trying to registered the current directory
/// into the managegment system using the given unregistered ID.
#[derive(Parser)]
#[command(name = "wsp", about = "CLI workspace directory mangement.")]
pub struct Cli {
    #[command(subcommand)]
    pub cmd: Option<Command>,
}

/// Define the operation can be execute upon the workspace directory manaement system.
///
/// This include, adding, gettin, removing, and listing the workspace.
#[derive(Subcommand)]
pub enum Command {
    /// Register the directory with the given name as the unique ID.
    Add {
        /// The unique ID for the directory.
        name: String,
        /// The path to the directory.
        path: String,
    },

    /// Fetch the path based on the registered unique ID.
    Get {
        /// The registered unique ID of the directory.
        name: String,
    },

    /// Remove the path based on the registered unique ID.
    Remove {
        /// The registered unique ID of the directory.
        name: String,
    },

    /// List all registered ID together with its associated path.
    List,

    /// Rename existing usnique ID to a new one
    Rename {
        /// Old unique ID
        old_name: String,
        /// New ID to change into
        new_name: String,
    },
}
