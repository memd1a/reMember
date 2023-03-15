use std::net::IpAddr;

use constant_time_eq::constant_time_eq;
use rand::{thread_rng, RngCore};
use sea_orm::{ActiveModelTrait, DbErr, TryIntoModel};
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};
use thiserror::Error;

use crate::created_at;
use crate::entities::account::{Model, Entity, ActiveModel, Column};
use crate::entities::ban;
use crate::entities::sea_orm_active_enums::GenderTy;

pub type AccountId = i32;
pub type HardwareInfo = u32;

#[derive(Debug)]
#[repr(u8)]
pub enum Region {
    Asia = 1,
    America = 2,
    Europe = 3,
}

#[derive(Debug, Error)]
pub enum AccountServiceError {
    #[error("Account with the username already exists")]
    UsernameAlreadyExists,
    #[error("No account for this username was found")]
    UsernameNotFound,
    #[error("Password mismatch")]
    PasswordMismatch,
    #[error("Password size is wrong")]
    PasswordWrongSize,
    #[error("Password is only supposed to contain ASCII characters")]
    PasswordWrongChar,
    #[error("Password size is wrong")]
    UsernameWrongSize,
    #[error("Password is only supposed to contain ASCII characters")]
    UsernameWrongChar,
    #[error("Account is banned")]
    AccountIsBanned,
    #[error("database")]
    Disconnect(#[from] DbErr),
}

//MAybe use passwords crate

pub type AccResult<T> = std::result::Result<T, AccountServiceError>;

#[derive(Debug, Clone)]
pub struct AccountService {
    db: DatabaseConnection,
}

type PasswordSalt = [u8; 16];

const HASH_COST: u32 = 9;

fn gen_salt() -> PasswordSalt {
    let mut salt = PasswordSalt::default();
    thread_rng().fill_bytes(&mut salt);
    salt
}

fn hash_password(pw: &str) -> String {
    let salt = gen_salt();
    bcrypt::hash_with_salt(pw, HASH_COST, salt)
        .unwrap()
        .to_string()
}

impl AccountService {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn get(&self, id: AccountId) -> anyhow::Result<Option<Model>> {
        Ok(Entity::find_by_id(id).one(&self.db).await?)
    }

    pub async fn create(
        &self,
        username: impl ToString,
        password: &str,
        region: Region,
        accepted_tos: bool,
        gender: Option<GenderTy>,
    ) -> anyhow::Result<AccountId> {
        //TODO check username + password
        let hash = hash_password(password);

        let acc = ActiveModel {
            username: Set(username.to_string()),
            password_hash: Set(hash),
            accepted_tos: Set(accepted_tos),
            created_at: created_at(&self.db),
            country: Set(region as u8 as i32),
            gm_level: Set(0),
            last_selected_world: Set(0),
            character_slots: Set(0),
            nx_credit: Set(0),
            maple_points: Set(0),
            nx_prepaid: Set(0),
            gender: Set(gender),
            ..Default::default()
        };

        let res = Entity::insert(acc).exec(&self.db).await?;
        Ok(res.last_insert_id)
    }

    pub async fn find_account_by_username(&self, username: &str) -> anyhow::Result<Option<Model>> {
        Ok(Entity::find()
            .filter(Column::Username.eq(username))
            .one(&self.db)
            .await?)
    }

    pub async fn try_login(&self, username: &str, password: &str) -> AccResult<Model> {
        let res = Entity::find()
            .filter(Column::Username.eq(username))
            //TODO find latest ban
            .find_also_related(ban::Entity)
            .one(&self.db)
            .await?;

        let Some((acc, last_ban)) = res else {
            return Err(AccountServiceError::UsernameNotFound)
        };

        if let Some(_last_ban) = last_ban {
            return Err(AccountServiceError::AccountIsBanned);
        }

        let verfiy_password = self.verify_password(password, &acc.password_hash).unwrap();
        if !verfiy_password {
            return Err(AccountServiceError::PasswordMismatch);
        }

        //TODO add some locking logic

        Ok(acc)
    }

    pub async fn update(
        &self,
        acc: Model,
        update: impl FnOnce(&mut ActiveModel),
    ) -> anyhow::Result<Model> {
        let mut acc: ActiveModel = acc.into();
        update(&mut acc);

        Ok(acc.save(&self.db).await?.try_into_model().unwrap())
    }

    pub async fn set_gender(&self, acc: Model, gender: GenderTy) -> anyhow::Result<Model> {
        self.update(acc, |acc| {
            acc.gender = Set(Some(gender));
        })
        .await
    }

    pub async fn set_pic(&self, acc: Model, pic: String) -> anyhow::Result<Model> {
        self.update(acc, |acc| {
            acc.pic = Set(Some(pic));
        })
        .await
    }

    pub async fn set_pin(&self, acc: Model, pin: String) -> anyhow::Result<Model> {
        self.update(acc, |acc| {
            acc.pin = Set(Some(pin));
        })
        .await
    }

    pub async fn accept_tos(&self, acc: Model) -> anyhow::Result<Model> {
        self.update(acc, |acc| {
            acc.accepted_tos = Set(true);
        })
        .await
    }

    pub async fn delete_acc(&self, _id: AccountId) -> anyhow::Result<()> {
        todo!()
    }

    pub fn verify_password(&self, pw: &str, pw_hash: &str) -> anyhow::Result<bool> {
        Ok(bcrypt::verify(pw, pw_hash)?)
    }

    pub fn check_pin(&self, acc: &Model, pin: &str) -> anyhow::Result<bool> {
        let Some(acc_pin) = acc.pin.as_ref() else {
            anyhow::bail!("Pin not set")
        };

        Ok(constant_time_eq(acc_pin.as_bytes(), pin.as_bytes()))
    }

    pub fn check_pic(&self, acc: &Model, pic: &str) -> anyhow::Result<bool> {
        let Some(acc_pic) = acc.pic.as_ref() else {
            anyhow::bail!("Pic not set")
        };

        Ok(constant_time_eq(acc_pic.as_bytes(), pic.as_bytes()))
    }

    pub fn check_hardware_info(
        &self,
        _acc: &Model,
        _hw_info: &HardwareInfo,
        _ip: IpAddr,
    ) -> anyhow::Result<()> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use sea_orm::DatabaseConnection;

    use crate::{services::account::Region, entities::sea_orm_active_enums::GenderTy};

    use super::AccountService;

    pub(crate) async fn get_test_db() -> anyhow::Result<DatabaseConnection> {
        let db = crate::gen_sqlite(crate::SQL_OPT_MEMORY).await?;
        Ok(db)
    }

    async fn get_test_svc() -> anyhow::Result<AccountService> {
        let acc_svc = AccountService::new(get_test_db().await?);
        Ok(acc_svc)
    }

    #[tokio::test]
    async fn account_insert() -> anyhow::Result<()> {
        const USERNAME: &str = "test1";
        const PW: &str = "abc123";

        let svc = get_test_svc().await?;
        let acc_id = svc.create(USERNAME, PW, Region::Europe, true, None).await?;

        let acc = svc.get(acc_id).await?.unwrap();

        let acc = svc.set_gender(acc, GenderTy::Female).await?;
        assert_eq!(acc.gender, Some(GenderTy::Female));

        let acc = svc.accept_tos(acc).await?;
        assert_eq!(acc.accepted_tos, true);

        //Login must work
        svc.try_login(USERNAME, PW).await?;

        Ok(())
    }
}
