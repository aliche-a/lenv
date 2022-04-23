use clap::{Parser, Subcommand};
use crate::list;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
#[clap(propagate_version = true)]
pub struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

/// Subcommands on the root Cli struct
#[derive(Subcommand)]
enum Commands {
    /// List contents of current directory
    List(list::ListArgs),
    /// Create a file
    Touch {
        /// Name or path of file to create
        name: String,
    },
}

impl Cli {
    /// Parse the args, commands, options, given to the app and
    /// execute the appropriate operation
    pub fn exec(&self) {
        match &self.command {
            Commands::List(args) => {
                println!("List called: {:?}", args);
            }
            Commands::Touch { name } => {
                println!("Touch command called with name: {:?}.", *name);
            }
        }
    }
}
