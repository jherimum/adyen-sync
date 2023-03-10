use adyen_sync::{
    commands::{
        config::handlers::config_handler,
        database::handlers::database_handler,
        root::{Cli, Command},
    },
    settings::Settings,
};
use clap::Parser;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let mut config = Settings::load()?;
    let cli = Cli::parse();

    match cli.command {
        Command::Config(command) => config_handler(&mut config, &cli.global_opts, command).await,
        Command::Database(command) => database_handler(&config, &cli.global_opts, command).await,
    }
}
