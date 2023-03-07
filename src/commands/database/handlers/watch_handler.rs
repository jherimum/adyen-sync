use anyhow::Result;

use crate::{
    commands::{database::commands::DatabaseOpts, root::GlobalOpts},
    settings::Settings,
};

pub async fn database_watch(_: &Settings, _: &GlobalOpts, _: &DatabaseOpts) -> Result<()> {
    todo!()
}
