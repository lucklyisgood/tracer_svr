use core::sea_orm::{ConnectionTrait, DbErr, Schema};
use core::sea_orm_migration::SchemaManager;

use entity::proj_list::Entity;

async fn sync_schema(state: &core::typedefs::AppState) -> Result<(), DbErr> {
    let db_conn = &state.conn;
    let manager = SchemaManager::new(db_conn);

    let builder = db_conn.get_database_backend();
    let schema = Schema::new(builder);
    let tcs_entity = schema.create_table_from_entity(Entity);

    manager.create_table(tcs_entity).await?;
    Ok(())
}

pub async fn setup(state: &core::typedefs::AppState) {
    let res: Result<(), DbErr> = sync_schema(state).await;
    if res.is_err() {
        tracing::debug!("sync table fail {:?}", res);
    }
}
