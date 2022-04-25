use std::{result, error};
use clap::Args;

/// Wrapper for errors from List command
type Result<T> = result::Result<T, Box<dyn error::Error>>;

#[derive(Args, Debug)]
pub struct ListArgs {
    /// Directory to print
    //  feature: print contents of multiple given dirs
    file: Option<String>,

    // "Content"-related args: changes the content displayed
    /// Include entries starting with .
    #[clap(short, long)]
    all: bool,

    /// list directories, not their contents
    #[clap(short, long)]
    dirs: bool,

    // "Format" flags: changes the way content is displayed
    /// Long listing
    #[clap(short)]
    long: bool,

    /// List entries by columns
    #[clap(short = 'C', long)]
    columns: bool,

    // Hidden options meant for internal use
    // io?
}

impl ListArgs {
    pub fn run(&self) {
        match self.default_list() {
            Ok(contents) => println!("{}", contents),
            Err(e) => eprintln!("Error printing contents: {}", e),
        };
    }

    /// Default behavior; builds the output for the contents of current or given directory
    fn default_list(&self) -> Result<String> {
        let contents = gtmpl::template("{{ . }}", "here")?;
        Ok(contents)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn list_curr_dir() {
        let args = default_args();
        assert_eq!("here", args.default_list().unwrap());
    }

    // helper function to create default args that tests will use and
    // modify as needed
    fn default_args() -> ListArgs {
        ListArgs {
            file: None,
            all: false,
            dirs: false,
            long: false,
            columns: false,
        }
    }
}
