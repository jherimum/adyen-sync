use crate::commands::database::commands::DatabaseOpts;
use crate::commands::root::GlobalOpts;
use crate::database::models::ToTarget;
use crate::database::repo;
use crate::database::repo::Pools;
use crate::settings::Settings;
use anyhow::Result;

pub async fn databse_sync(
    settings: &Settings,
    global_opts: &GlobalOpts,
    database_opts: &DatabaseOpts,
    batch_size: u8,
    target_client_id: String,
) -> Result<()> {
    let database_opts = database_opts.merge(settings);
    let pools: Pools = database_opts.try_into()?;

    let mut max_target_raw_uid = repo::get_max_raw_uidpk(&pools.target).await?;
    let mut raws_to_import =
        repo::find_raw_after_uidpk(&pools.source, &max_target_raw_uid, batch_size).await?;

    while !raws_to_import.is_empty() {
        let mut tx = pools.target.begin().await?;
        for r in raws_to_import.iter_mut() {
            r.to_target(&target_client_id);
            repo::insert_raw_notification(&mut tx, r).await?;
        }
        tx.commit().await?;

        max_target_raw_uid = repo::get_max_raw_uidpk(&pools.target).await?;
        raws_to_import =
            repo::find_raw_after_uidpk(&pools.source, &max_target_raw_uid, batch_size).await?;
    }

    Ok(())
}
