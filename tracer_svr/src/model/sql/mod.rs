pub mod projlist_sql;
use once_cell::sync::Lazy;
use std::collections::HashMap;
use super::exec_sql;

static SQL_LIST: Lazy<HashMap<&str, &str>> = Lazy::new(||HashMap::from([
    ("proj_list", projlist_sql::PROJ_LIST_TABLE_SQL),
]));

pub async fn sync_table() {
    for (key, val) in SQL_LIST.iter() {
        let result = exec_sql(key, val).await;
        match result {
            Err(e) => {
                panic!("sync table err {}", e)
            }
            _ => {}
        }
    }
}