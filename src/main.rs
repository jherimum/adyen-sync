use adyen_sync::{
    commands::{
        config::handlers::config_handler,
        database::handlers::database_handler,
        root::{CliArgs, Command},
    },
    settings::Settings,
};
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
