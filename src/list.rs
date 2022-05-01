use clap::Args;
use std::{error, fs, result};

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
    pub fn run(&self) {
        match self.default_list() {
            Ok(contents) => println!("{}", contents),
            Err(e) => eprintln!("Error printing contents: {}", e),
        };
    }

    /// Default behavior; builds the output for the contents of current or given directory.
    /// If any errors occur, immediately return it to run so run can handle/wrap it
    fn default_list(&self) -> Result<String> {
        // Error: after processing in filter_map, we tried to use the result,
        // which was owned by a temporary variable created in the scope.
        // So create a new vector to hold the file names after processing.
        let mut file_names = Vec::new();

        // Read the contents of the given directory.
        // Borrow the value from self, so it doesn't get moved into read_dir
        let files = fs::read_dir(&self.dir)
            // Unwrap to get the ReadDir iterator from the result
            .unwrap()
            // Filter and process each entry in the iterator,
            // returning only file name of each Ok variant.
            // The return needs to be wrapped in an Option because of the iterator(?)
            .filter_map(|path| match path {
                Ok(p) => Some(p.file_name()),
                Err(_) => None,
            });

        // Error: Vec<OsString> cannot be used with template.
        // So we convert each OsString into String and push all valid strings
        // into file_names, which gives us the Vec<String> we need for template.
        for file in files {
            if let Ok(name) = file.into_string() {
                file_names.push(name)
            }
        }

        // Sort the results so we have a consistent output
        file_names.sort();

        // Now that we have a Vec<String>, which will work with template,
        // we can pass it to the function for rendering
        let contents = gtmpl::template(TEMPLATE, file_names)?;
        Ok(contents)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn list_curr_dir() {
        let args = default_args();
        assert_eq!("hello.txt\nworld.txt\n", args.default_list().unwrap());
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
