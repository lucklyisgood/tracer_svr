use tracing;

#[cfg(feature="sqlite")]
use core::database::{init_sqlite, DB};

use rbatis::{table_sync::{SqliteTableSync, TableSync}, rbdc::datetime::FastDateTime};
use rbs::to_value;

pub mod human;

#[cfg(feature="sqlite")]
async fn init_db() {
    init_sqlite().await.unwrap();
}

async fn sync_table() {
    let mut s = SqliteTableSync::default();
    s.sql_id = " PRIMARY KEY AUTOINCREMENT NOT NULL ".to_string();
    s.sync(
        DB.acquire().await.unwrap(),
        to_value!(human::Human {
            id: Some(0),
            create_time: Some(FastDateTime::now()),
            name: Some("".into())
        }),
        "human"
    ).await.unwrap();
}

pub async fn setup() {
    tracing::info!("model setup");
    init_db().await;
    sync_table().await;
}
