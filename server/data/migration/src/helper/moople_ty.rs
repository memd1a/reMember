use sea_orm_migration::{prelude::*, sea_query::extension::postgres::{TypeCreateStatement, Type}};


#[derive(Iden)]
pub enum Gender {
    GenderTy,
    Female,
    Male,
}

pub fn moople_gender_ty() -> TypeCreateStatement {
    Type::create()
        .as_enum(Gender::GenderTy)
        .values([Gender::Male, Gender::Female])
        .to_owned()
}

pub fn moople_gender_col(id: impl IntoIden) -> ColumnDef {
    ColumnDef::new(id)
        .enumeration(Gender::GenderTy, [Gender::Male, Gender::Female])
        .to_owned()
}

pub fn moople_id(name: impl IntoIden) -> ColumnDef {
    ColumnDef::new(name).integer().not_null().to_owned()
}

pub fn moople_opt_id(name: impl IntoIden) -> ColumnDef {
    ColumnDef::new(name).integer().null().to_owned()
}

pub fn moople_id_pkey(name: impl IntoIden) -> ColumnDef {
    moople_id(name).auto_increment().primary_key().to_owned()
}

pub fn moople_int(id: impl IntoIden) -> ColumnDef {
    ColumnDef::new(id).integer().default(0).not_null().to_owned()
}

pub fn moople_size(id: impl IntoIden) -> ColumnDef {
    ColumnDef::new(id).integer().default(0).not_null().to_owned()
}

pub fn moople_bool(id: impl IntoIden) -> ColumnDef {
    ColumnDef::new(id).boolean().not_null().default(false).to_owned()
}

pub fn moople_str(id: impl IntoIden) -> ColumnDef {
    ColumnDef::new(id).string().to_owned()
}

pub fn moople_small_str(id: impl IntoIden) -> ColumnDef {
    ColumnDef::new(id).string().string_len(16).to_owned()
}

pub fn date_time(id: impl IntoIden) -> ColumnDef {
    ColumnDef::new(id).timestamp().to_owned()
}

pub fn mopple_cash_id(id: impl IntoIden) -> ColumnDef {
    ColumnDef::new(id).big_integer().null().to_owned()
}

pub fn created_at(id: impl IntoIden) -> ColumnDef {
    date_time(id)
        .default(Expr::current_timestamp())
        .not_null()
        .to_owned()
}

pub fn moople_name(id: impl IntoIden) -> ColumnDef {
    ColumnDef::new(id).string_len(13).not_null().to_owned()
}

pub fn moople_stat(id: impl IntoIden) -> ColumnDef {
    ColumnDef::new(id).integer().not_null().default(0).to_owned()
}

pub fn char_stat(id: impl IntoIden) -> ColumnDef {
    ColumnDef::new(id).integer().not_null().default(0).to_owned()
}