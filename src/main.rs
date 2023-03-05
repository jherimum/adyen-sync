use clap::{Args, Parser, Subcommand};

#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct AsyncAdyenArgs {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    Status(StatusCommand),
}

#[derive(Debug, Args)]
pub struct StatusCommand {
    source_connection: Option<String>,
    target_connection: Option<String>,
}

fn main() {
    let x = AsyncAdyenArgs::parse();
    println!("{:?}", x);
}
