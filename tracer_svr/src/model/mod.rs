use core::sea_orm::ConnectionTrait;
use entity::sea_orm::Schema;
use entity::proj_list::Entity;

fn sync_schema(state: &core::typedefs::AppState) {
    
    let conn = &state.conn;
    let builder = conn.get_database_backend();
    let schema = Schema::new(builder);

    let mut s_builder = schema.create_table_from_entity(Entity);
    s_builder.if_not_exists();
    let s = builder.build(&s_builder);
    tracing::info!("sql: {}", s.sql);
}

pub fn setup(state: &core::typedefs::AppState) {
    sync_schema(state);
}