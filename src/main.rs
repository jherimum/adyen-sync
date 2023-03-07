use adyen_sync::{commands::*, handlers::*, settings::Settings};
use clap::Parser;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let mut config = Settings::load()?;
    let args = CliArgs::parse();

    match args.command {
        Command::Config(command) => config_handler(&mut config, &args.global_opts, &command).await,
        Command::Database(command) => database_handler(&config, &args.global_opts, &command).await,
    }
}
