use adyen_sync::{commands::*, handlers::*, settings::Settings};
use clap::Parser;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let config = Settings::load()?;
    let args = CliArgs::parse();

    match args.command {
        Command::Config(config_c) => match config_c.command {
            ConfigSubCommand::Show(c) => config_show(&config, args.global_opts, &c).await,
            ConfigSubCommand::Set(c) => config_set(&config, &c).await,
        },
        Command::Sync(sync_c) => match sync_c.commands {
            SyncSubCommand::Status(command) => sync_status(&config, &command).await,
            SyncSubCommand::Update(command) => sync_update(&config, &command).await,
            SyncSubCommand::Watch(command) => sync_watch(&config, &command).await,
        },
    }
}
