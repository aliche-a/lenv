use clap::Parser;

mod cli;
mod list;

fn main() {
    let cli = cli::Cli::parse();

    cli.exec();
}