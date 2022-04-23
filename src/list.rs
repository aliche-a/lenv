use clap::Args;

#[derive(Args, Debug)]
pub struct ListArgs {
    // Format-related args
    /// Include entries starting with .
    #[clap(short, long)]
    all: Option<bool>,
    /// Long listing
    #[clap(short)]
    long: Option<bool>,
}


