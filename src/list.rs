use clap::Args;
use std::{
    error,
    fs::{self, DirEntry},
    os::unix::prelude::OsStrExt,
    result,
};

const TEMPLATE: &str = r#"
{{- range . -}}
    {{ printf "%s\n" . }}
{{- end -}}
"#;

/// Wrapper for errors from List command
type Result<T> = result::Result<T, Box<dyn error::Error>>;

#[derive(Args, Debug)]
pub struct ListArgs {
    /// Directory to print
    //  feature: print contents of multiple given dirs
    #[clap(default_value("."))]
    dir: String,

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
    /// run is the entry point into the command; runs the internal list method to perform the operation
    /// If any errors occur, they get returned immediately for run to handle/unwrap
    pub fn run(&self) {
        match self.list() {
            Ok(contents) => println!("{}", contents),
            Err(err) => eprintln!("Failed to list contents: {}", err),
        }
    }

    fn list(&self) -> Result<String> {
        // Read the contents of the given directory.
        // Borrow the value from self, so it doesn't get moved into read_dir
        let mut entries: Vec<DirEntry> = fs::read_dir(&self.dir)
            // Expect to get the ReadDir iterator from the result,
            // printing the error if we get one with the msg for context
            .expect("Error reading directory")
            // Filter and process each entry in the iterator,
            // returning only file name of each Ok variant.
            // The return needs to be wrapped in an Option because of the iterator(?)
            .filter_map(|path| path.ok())
            .collect();

        if !self.all {
            self.no_hidden_files(&mut entries)
        }

        let mut contents = Vec::new();
        // Error: Vec<OsString> cannot be used with template.
        // So we convert each OsString into String and push all valid strings
        // into file_names, which gives us the Vec<String> we need for template.
        for entry in entries {
            if let Ok(name) = entry
                .file_name()
                .into_string()
            {
                contents.push(name)
            }
        }

        // Sort the results so we have a consistent output
        contents.sort();

        // Now that we have a Vec<String>, which will work with template,
        // we can pass it to the function for rendering
        let result = gtmpl::template(TEMPLATE, contents)?;
        Ok(result)
    }

    fn no_hidden_files(&self, entries: &mut Vec<DirEntry>) {
        entries.retain(|entry| {
            match entry
                .file_name()
                .as_bytes()
                .first()
            {
                Some(byte) => *byte != b'.',
                None => false,
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn list_curr_dir() {
        let args = default_args();
        assert_eq!(
            "hello.txt\nworld.txt\n",
            args.list().unwrap()
        );
    }

    #[test]
    fn list_curr_dir_hidden() {
        let mut args = default_args();
        args.all = true;
        assert_eq!(
            ".hidden\n.no\nhello.txt\nworld.txt\n",
            args.list().unwrap()
        );
    }

    // helper function to create default args that tests will use and
    // modify as needed
    fn default_args() -> ListArgs {
        ListArgs {
            dir: String::from("./resources/testing/ls-test-dir"),
            all: false,
            dirs: false,
            long: false,
            columns: false,
        }
    }
}
