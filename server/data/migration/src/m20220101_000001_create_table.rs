use futures::future::try_join_all;

use sea_orm_migration::{prelude::*, sea_query::extension::postgres::Type};

use crate::{
    helper::{stats::with_equip_stats, *},
    moople::with_char_stats,
};

#[derive(Iden)]
enum Account {
    Table,
    Id,
    Username,
    PasswordHash,
    Gender,
    AcceptedTos,
    LastLoginAt,
    CreatedAt,
    Pin,
    Pic,
    Country,
    GmLevel,
    LastSelectedWorld,
    CharacterSlots,
    NxCredit,
    NxPrepaid,
    MaplePoints,
}

#[derive(Iden)]
enum Ban {
    Table,
    Id,
    BanReason,
    BanTime,
    AccId,
}

#[derive(Iden)]
enum Character {
    Table,
    Id,
    AccId,
    Name,
    CreatedAt,
    LastLoginAt,
    Gender,
}

#[derive(Iden)]
enum ItemStack {
    Table,
    Id,
    ItemId,
    CashId,
    ExpiresAt,
    Quantity,
    Flags,
}

#[derive(Iden)]
enum EquipItem {
    Table,
    Id,
    ItemId,
    CashId,
    ExpiresAt,
    Flags,
    OwnerTag,
    ItemLevel,
    ItemExp,
    ViciousHammers,
}

#[derive(Iden)]
enum PetItem {
    Table,
    Id,
    ItemId,
    CashId,
    ExpiresAt,
    Flags,
    Name,
    Level,
    Tameness,
    Fullness,
    Skill,
    RemainingLife,
    Summoned,
}

#[derive(Iden)]
enum InventorySlot {
    Table,
    Id,
    InvType,
    Slot,
    CharId,
    EquipItemId,
    StackItemId,
    PetItemId,
}

#[derive(DeriveMigrationName)]
pub struct Migration {
    acc_table: MoopleTbl,
    char_table: MoopleTbl,
    ban_table: MoopleTbl,
    eq_table: MoopleTbl,
    stack_item_table: MoopleTbl,
    pet_item_table: MoopleTbl,
    inv_slot_table: MoopleTbl,
}

impl Default for Migration {
    fn default() -> Self {
        let acc_table = MoopleTbl::new(
            Account::Table,
            Account::Id,
            [
                ColumnDef::new(Account::Username)
                    .string()
                    .not_null()
                    .unique_key()
                    .to_owned(),
                ColumnDef::new(Account::PasswordHash)
                    .string()
                    .not_null()
                    .to_owned(),
                moople_bool(Account::AcceptedTos),
                moople_gender_col(Account::Gender).null().to_owned(),
                date_time(Account::LastLoginAt),
                created_at(Account::CreatedAt),
                moople_small_str(Account::Pin),
                moople_small_str(Account::Pic),
                moople_id(Account::Country),
                moople_int(Account::GmLevel),
                moople_id(Account::LastSelectedWorld),
                moople_size(Account::CharacterSlots),
                moople_size(Account::NxCredit),
                moople_size(Account::NxPrepaid),
                moople_size(Account::MaplePoints),
            ],
            [],
        );

        let char_table = MoopleTbl::new(
            Character::Table,
            Character::Id,
            with_char_stats([
                moople_name(Character::Name),
                created_at(Character::CreatedAt),
                date_time(Character::LastLoginAt),
                moople_gender_col(Character::Gender).not_null().to_owned(),
            ]),
            [Ref::ownership(Character::AccId, &acc_table)],
        );

        let ban_table = MoopleTbl::new(
            Ban::Table,
            Ban::Id,
            [moople_str(Ban::BanReason), date_time(Ban::BanTime)],
            [Ref::ownership(Ban::AccId, &acc_table)],
        );

        let item_stack_table = MoopleTbl::new(
            ItemStack::Table,
            ItemStack::Id,
            [
                date_time(ItemStack::ExpiresAt),
                mopple_cash_id(ItemStack::CashId),
                moople_id(ItemStack::ItemId),
                moople_int(ItemStack::Flags),
                moople_size(ItemStack::Quantity),
            ],
            [],
        );

        let item_equip_table = MoopleTbl::new(
            EquipItem::Table,
            EquipItem::Id,
            with_equip_stats([
                date_time(EquipItem::ExpiresAt),
                mopple_cash_id(EquipItem::CashId),
                moople_id(EquipItem::ItemId),
                moople_int(EquipItem::Flags),
                moople_size(EquipItem::ItemLevel),
                moople_size(EquipItem::ItemExp),
                moople_size(EquipItem::ViciousHammers),
                moople_name(EquipItem::OwnerTag),
            ]),
            [],
        );

        let item_pet_table = MoopleTbl::new(
            PetItem::Table,
            PetItem::Id,
            [
                date_time(PetItem::ExpiresAt),
                mopple_cash_id(PetItem::CashId),
                moople_id(PetItem::ItemId),
                moople_int(PetItem::Flags),
                moople_name(PetItem::Name),
                moople_stat(PetItem::Level),
                moople_stat(PetItem::Tameness),
                moople_stat(PetItem::Fullness),
                moople_stat(PetItem::Skill),
                moople_stat(PetItem::RemainingLife),
                moople_bool(PetItem::Summoned),
            ],
            [],
        );

        let inv_slot_table = MoopleTbl::new(
            InventorySlot::Table,
            InventorySlot::Id,
            [
                moople_int(InventorySlot::InvType),
                moople_int(InventorySlot::Slot),
            ],
            [
                Ref::ownership(InventorySlot::CharId, &char_table),
                Ref::opt(InventorySlot::EquipItemId, &item_equip_table),
                Ref::opt(InventorySlot::StackItemId, &item_stack_table),
                Ref::opt(InventorySlot::PetItemId, &item_pet_table),
            ],
        );
        Self {
            acc_table,
            char_table,
            ban_table,
            eq_table: item_equip_table,
            stack_item_table: item_stack_table,
            pet_item_table: item_pet_table,
            inv_slot_table,
        }
    }
}

impl Migration {
    fn table_iter(&self) -> impl Iterator<Item = &'_ MoopleTbl> {
        [
            &self.acc_table,
            &self.char_table,
            &self.ban_table,
            &self.eq_table,
            &self.pet_item_table,
            &self.stack_item_table,
            &self.inv_slot_table,
        ]
        .into_iter()
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.create_type(moople_gender_ty()).await?;

        for tbl in self.table_iter() {
            tbl.create_table(manager).await?;
        }

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        try_join_all(self.table_iter().map(|tbl| tbl.drop_fk(manager))).await?;
        for tbl in self.table_iter() {
            tbl.drop_table(manager).await?;
        }

        manager
            .drop_type(Type::drop().name(Gender::GenderTy).to_owned())
            .await?;

        Ok(())
    }
}
