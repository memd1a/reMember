use proto95::{
    id::{job_id::JobGroup, FaceId, HairId, ItemId, MapId, Skin},
    login::char::{DeleteCharResult, SelectCharResultCode},
    shared::Gender,
};
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, QuerySelect, Set, ActiveModelTrait};

use crate::{
    created_at,
    entities::{
        account,
        character::{ActiveModel, Column, Entity, Model, self},
        skill,
    },
};

use super::{account::AccountService, item::ItemService};

#[derive(Debug, Clone)]
pub struct ItemStarterSet {
    pub bottom: ItemId,
    pub shoes: ItemId,
    pub top: ItemId,
    pub weapon: ItemId,
    pub guide: ItemId,
}

impl ItemStarterSet {
    pub fn validate(&self, job: JobGroup) -> anyhow::Result<()> {
        //TODO: update to v95
        let _bottom = check_contains(job.get_starter_bottoms(), self.bottom, "Bottom ID")?;
        let _shoes = check_contains(job.get_starter_shoes(), self.shoes, "Shoes ID")?;
        let _top = check_contains(job.get_starter_tops(), self.top, "Top ID")?;
        let _weapon = check_contains(job.get_starter_weapons(), self.weapon, "Weapon ID")?;
        if self.guide != job.get_guide_item() {
            anyhow::bail!("Invalid starter guide");
        }

        Ok(())
    }

    pub fn default_starter_set(job: JobGroup) -> Self {
        Self {
            shoes: ItemId::LEATHER_SANDALS,
            bottom: ItemId::BLUE_JEAN_SHORTS,
            top: ItemId::WHITE_UNDERSHIRT,
            weapon: ItemId::SWORD,
            guide: job.get_guide_item(),
        }
    }
}

pub type CharacterID = i32;

#[derive(Debug, Clone)]
pub struct CharacterCreateDTO {
    pub name: String,
    pub job_group: JobGroup,
    pub face: FaceId,
    pub skin: Skin,
    pub hair: HairId,
    pub starter_set: ItemStarterSet,
    pub gender: Gender,
}

impl CharacterCreateDTO {
    pub fn get_starter_set(&self) -> ItemStarterSet {
        self.starter_set.clone()
    }
    pub fn validate(&self) -> anyhow::Result<()> {
        Ok(())
        /*  de-uglify and test this
        let job = self.job_group;
        let _face = check_contains(job.get_starter_face(), self.face, "Face ID")?;
        let _hair = check_contains(job.get_starter_hair(), self.hair, "Hair")?;
        self.starter_set.validate(job)?;

        Ok(())*/
    }
}

fn is_valid_char_name(name: &str) -> bool {
    //TODO error messages
    if !(3..13).contains(&name.len()) {
        return false;
    }

    if !name.chars().all(|c| c.is_ascii_alphanumeric()) {
        return false;
    }

    true
}

pub fn check_contains<T: PartialEq + std::fmt::Debug>(
    mut iter: impl Iterator<Item = T>,
    check_id: T,
    name: &str,
) -> anyhow::Result<T> {
    if !iter.any(|id| id == check_id) {
        anyhow::bail!("Invalid {name} ({check_id:?}) for char creation ")
    }

    Ok(check_id)
}

#[derive(Debug, Clone)]
pub struct CharacterService {
    db: DatabaseConnection,
    account: AccountService,
}

impl CharacterService {
    pub fn new(db: DatabaseConnection) -> Self {
        Self {
            db: db.clone(),
            account: AccountService::new(db),
        }
    }

    pub async fn check_name(&self, name: &str) -> anyhow::Result<bool> {
        if !is_valid_char_name(name) {
            return Ok(false);
        }

        let other_id = Entity::find()
            .select_only()
            .column(Column::Id)
            .filter(Column::Name.eq(name))
            .one(&self.db)
            .await?;

        Ok(other_id.is_none())
    }

    pub async fn get_characters_for_account(&self, acc_id: i32) -> anyhow::Result<Vec<Model>> {
        Ok(Entity::find()
            .filter(Column::AccId.eq(acc_id))
            .all(&self.db)
            .await?)
    }

    pub async fn get(&self, char_id: CharacterID) -> anyhow::Result<Option<Model>> {
        Ok(Entity::find_by_id(char_id).one(&self.db).await?)
    }

    pub async fn must_get(&self, char_id: CharacterID) -> anyhow::Result<Model> {
        self.get(char_id)
            .await?
            .ok_or_else(|| anyhow::format_err!("No char for id: {char_id}"))
    }

    pub async fn create_character(
        &self,
        acc_id: i32,
        create: CharacterCreateDTO,
        item_svc: &ItemService,
    ) -> anyhow::Result<CharacterID> {
        create.validate()?;

        if !self.check_name(&create.name).await? {
            anyhow::bail!("Name is not valid");
        }

        let job = create.job_group;
        let map_id = MapId::AMHERST.0 as i32; //job.get_start_map().0 as i32;
        let job = job.get_noob_job_id() as u32;

        let char = ActiveModel {
            acc_id: Set(acc_id),
            created_at: created_at(&self.db),
            gender: Set((create.gender).into()),
            name: Set(create.name),
            map_id: Set(map_id),
            job: Set(job as i32),
            level: Set(1),
            str: Set(13),
            dex: Set(4),
            int: Set(4),
            luk: Set(4),
            hp: Set(5),
            max_hp: Set(50),
            mp: Set(5),
            max_mp: Set(50),
            equip_slots: Set(24),
            use_slots: Set(24),
            setup_slots: Set(24),
            etc_slots: Set(24),
            buddy_capacity: Set(20),
            skin: Set(create.skin as u8 as i32),
            face: Set(create.face.0 as i32),
            hair: Set(create.hair.0 as i32),
            exp: Set(0),
            gacha_exp: Set(0),
            mesos: Set(50_000),
            fame: Set(0),
            ap: Set(0),
            sp: Set(10),
            spawn_point: Set(0),
            skill_points: Set(vec![0; 20]),
            play_time: Set(0),
            ..Default::default()
        };

        let char_id = Entity::insert(char).exec(&self.db).await?.last_insert_id;
        item_svc
            .create_starter_set(char_id, create.starter_set)
            .await?;

        Ok(char_id)
    }

    pub async fn delete_character(
        &self,
        acc: &account::Model,
        char_id: CharacterID,
        pic: &str,
    ) -> anyhow::Result<DeleteCharResult> {
        if !self.account.check_pic(acc, pic)? {
            return Ok(DeleteCharResult::InvalidPic);
        }

        let char = self.must_get(char_id).await?;
        if char.acc_id != acc.id {
            return Ok(DeleteCharResult::UnknownErr);
        }

        /* Check:
        - world transfer
        - family
        - guild
        */

        Ok(DeleteCharResult::Success)
    }

    pub async fn select_char_with_pic(
        &self,
        acc: &account::Model,
        char_id: CharacterID,
        pic: &str,
    ) -> anyhow::Result<SelectCharResultCode> {
        if !self.account.check_pic(acc, pic)? {
            return Ok(SelectCharResultCode::InvalidPic);
        }

        self.select_char(acc, char_id).await
    }

    pub async fn select_char(
        &self,
        acc: &account::Model,
        char_id: CharacterID,
    ) -> anyhow::Result<SelectCharResultCode> {
        let char = self.must_get(char_id).await?;
        if char.acc_id != acc.id {
            return Ok(SelectCharResultCode::UnknownErr);
        }
        Ok(SelectCharResultCode::Success)
    }

    pub async fn load_skills(&self, id: CharacterID) -> anyhow::Result<Vec<skill::Model>> {
        Ok(skill::Entity::find()
            .filter(skill::Column::CharId.eq(id))
            .all(&self.db)
            .await?)
    }

    pub async fn save_char(&self, char: character::ActiveModel) -> anyhow::Result<()> {
        char.save(&self.db).await?;
        Ok(())
    }
}
