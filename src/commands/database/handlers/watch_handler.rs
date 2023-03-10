use std::{
    io::{stdout, Write},
    thread,
    time::Duration,
};

use anyhow::{Context, Result};
use chrono::{Duration as ChronoDuration, Utc};

use crate::{
    commands::{
        database::{
            commands::DatabaseWatchArgs,
            handlers::import::{self, Import},
        },
        root::GlobalOpts,
    },
    database::repo::{self, Pools},
    settings::{MergeSettings, Settings},
};

pub async fn database_watch(
    settings: &Settings,
    _: &GlobalOpts,
    args: DatabaseWatchArgs,
) -> Result<()> {
    println!("start watching");
    let args = args.merge(settings);
    let pools = Pools::try_from(&args).context("Error creating connection pools.")?;
    let mut ref_time = Utc::now().naive_utc() - ChronoDuration::days(100);
    loop {
        let raws = repo::find_raw_guid_after_created_date(&pools.source, &ref_time, 200).await?;

        let imported = Import::new(
            &pools,
            args.target_client_id
                .as_ref()
                .context("deveria ter passado")?,
            1,
        )
        .execute(&raws)
        .await;

        //let max_impoted_created_date = imported.iter().map(|r| r.created_date).max();
        // if let Some(date) = max_impoted_created_date {
        //     ref_time = date;
        // }

        thread::sleep(Duration::from_secs(args.delay as u64));
    }
}
