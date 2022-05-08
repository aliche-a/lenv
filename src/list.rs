use clap::Args;
use std::{
    error,
    fmt::Write,
    fs::{self, DirEntry},
    os::unix::prelude::{OsStrExt, PermissionsExt},
    result,
};

const TEMPLATE: &str = r#"
{{- range . -}}
    {{ printf "%s\n" . }}
{{- end -}}
"#;

const RWX: char = '7';
const RW: char = '6';
const RX: char = '5';
const READ: char = '4';
const WX: char = '3';
const WRITE: char = '2';
const EXEC: char = '1';

/// Wrapper for errors from List command
type Result<T> = result::Result<T, Box<dyn error::Error>>;

#[derive(Args, Debug, Default)]
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
    /// The entry point into the command; runs the internal list method to perform the operation
    /// If any errors occur, they get returned immediately for run to handle/unwrap
    pub fn run(&self) {
        match self.list() {
            Ok(contents) => println!("{}", contents),
            Err(err) => eprintln!("Failed to list contents: {}", err),
        }
    }

    /// Private function that holds the logic for list
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
            self.remove_hidden_files(&mut entries)
        }

        let mut contents = self.format(&mut entries);

        // Sort the results so we have a consistent output
        contents.sort();

        // Now that we have a Vec<String>, which will work with template,
        // we can pass it to the function for rendering
        let result = gtmpl::template(TEMPLATE, contents)?;
        Ok(result)
    }

    /// Filters out the hidden entries from the result.
    /// Hidden files are determined by examining each entry's filename;
    /// if the first character is a '.', it is removed by retain().
    // The first byte of the file name is compared to the byte representation
    // of '.' since this how Rust deals with strings.
    // Comparing by bytes also ensures that converting the file name (OsString)
    // won't potentially error out as with to_string(), which fails if
    // the OsString doesn't contain valid UTF-8 codes.
    fn remove_hidden_files(&self, entries: &mut Vec<DirEntry>) {
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

    /// Transforms each DirEntry into a string, formatting it according to the options set
    fn format(&self, entries: &mut Vec<DirEntry>) -> Vec<String> {
        let mut contents = Vec::new();
        // Error: Vec<OsString> cannot be used with template.
        // So we convert each OsString into String and push all valid strings
        // into file_names, which gives us the Vec<String> we need for template.
        for entry in entries {
            if self.long {
                if let Ok(metadata) = entry.metadata() {
                    let file_type = {
                        if metadata.is_dir() {
                            "d"
                        } else {
                            "-"
                        }
                    };

                    let perms = metadata.permissions().mode() & 0o777;
                    let nums = perms.to_string();

                    let mut perm_set = String::new();
                    for n in nums.chars() {
                        match n {
                            RWX => write!(&mut perm_set, "rwx"),
                            RW => "rw-",
                            RX => "r-x",
                            READ => "r--",
                            WX => "-wx",
                            WRITE => "-w-",
                            EXEC => "--x",
                            _ => continue,
                        }
                    }

                    let long_entry = format!("{} {} {}", file_type, perms, file_name);
                    contents.push(long_entry);
                }
            } else if let Ok(name) = entry
                .file_name()
                .into_string()
            {
                contents.push(name);
            }
        }

        contents
    }
}

// fn symbolic_perms(perm: Permission) -> String {
//     let nums: Vec<char> = perm
//         .to_string()
//         .chars()
//         .map(|n| {});
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn list_dir() {
        let args = default();
        assert_eq!(
            "hello.txt\nworld.txt\n",
            args.list().unwrap()
        );
    }

    #[test]
    fn list_dir_hidden() {
        let mut args = default();
        args.all = true;
        assert_eq!(
            ".hidden\n.no\nhello.txt\nworld.txt\n",
            args.list().unwrap()
        );
    }

    #[test]
    fn list_dir_long() {
        let mut args = default();
        args.long = true;
        assert_eq!("here", args.list().unwrap())
    }

    // helper function to create default args that tests will use and
    // modify as needed
    fn default() -> ListArgs {
        ListArgs {
            dir: String::from("./resources/testing/ls-test-dir"),
            ..Default::default()
        }
    }
}
