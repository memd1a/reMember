pub mod account;
pub mod character;
pub mod item;

pub use account::AccountService;
pub use character::CharacterService;
pub use item::ItemService;
use sea_orm::DatabaseConnection;

use super::meta::meta_service::MetaService;

#[derive(Debug)]
pub struct DataServices {
    pub account: AccountService,
    pub char: CharacterService,
    pub item: ItemService,
}

impl DataServices {
    pub fn new(db: DatabaseConnection, meta: &'static MetaService) -> Self {
        DataServices {
            account: AccountService::new(db.clone()),
            char: CharacterService::new(db.clone()),
            item: ItemService::new(db, meta),
        }
    }
}
