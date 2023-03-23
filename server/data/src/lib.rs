pub mod entity_ext;
pub mod services;
pub mod proto_mapper;
pub mod util;
pub mod entities;

use chrono::{NaiveDateTime, Utc};
use entities::{account, ban, character, equip_item, pet_item, item_stack, inventory_slot};
use sea_orm::{
    ActiveValue, ConnectOptions, ConnectionTrait, Database, DatabaseConnection,
    DbBackend, DbErr, Schema,
};
pub const SQL_OPT_MEMORY: &str = "sqlite::memory:";
pub const SQL_OPT_TEST_FILE: &str = "sqlite://test.db?mode=rwc";

pub fn created_at(db: &DatabaseConnection) -> ActiveValue<NaiveDateTime> {
    match db {
        DatabaseConnection::SqlxSqlitePoolConnection(_) => ActiveValue::Set(Utc::now().naive_utc()),
        _ => ActiveValue::NotSet,
    }
}

pub async fn gen_sqlite(opt: &str) -> Result<DatabaseConnection, DbErr> {
    let mut opt = ConnectOptions::new(opt.to_owned());
    opt.sqlx_logging(true);
    let db = Database::connect(opt).await?;

    let schema = Schema::new(DbBackend::Sqlite);

    db.execute(
        db.get_database_backend()
            .build(&schema.create_table_from_entity(account::Entity)),
    )
    .await?;
    db.execute(
        db.get_database_backend()
            .build(&schema.create_table_from_entity(character::Entity)),
    )
    .await?;
    db.execute(
        db.get_database_backend()
            .build(&schema.create_table_from_entity(ban::Entity)),
    )
    .await?;


    db.execute(
        db.get_database_backend()
            .build(&schema.create_table_from_entity(equip_item::Entity)),
    )
    .await?;

    db.execute(
        db.get_database_backend()
            .build(&schema.create_table_from_entity(pet_item::Entity)),
    )
    .await?;

    db.execute(
        db.get_database_backend()
            .build(&schema.create_table_from_entity(item_stack::Entity)),
    )
    .await?;

    db.execute(
        db.get_database_backend()
            .build(&schema.create_table_from_entity(inventory_slot::Entity)),
    )
    .await?;

    Ok(db)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn sqlite_build() {
        gen_sqlite(SQL_OPT_MEMORY).await.unwrap();
    }
}
