pub mod char;
use moople_packet::proto::time::MapleTime;
use proto95::{login::account::AccountInfo, shared::Gender};

use crate::entities::{account, sea_orm_active_enums::GenderTy};

impl From<&GenderTy> for Gender {
    fn from(value: &GenderTy) -> Self {
        match value {
            GenderTy::Female => Gender::Female,
            GenderTy::Male => Gender::Male,
        }
    }
}

impl From<Gender> for GenderTy {
    fn from(value: Gender) -> Self {
        match value {
            Gender::Female => GenderTy::Female,
            Gender::Male => GenderTy::Male,
        }
    }
}

impl From<&account::Model> for AccountInfo {
    fn from(model: &account::Model) -> Self {
        let gm = model.gm_level as u8;
        AccountInfo {
            id: model.id as u32,
            gender: model.gender.as_ref().into(),
            grade_code: gm,
            sub_grade_code: gm,
            is_test_acc: model.tester,
            country_id: model.country as u8,
            name: model.username.clone(),
            purchase_exp: 0,
            chat_block_reason: 0,
            chat_block_date: MapleTime::zero(),
            registration_date: MapleTime::try_from(model.created_at).unwrap(),
            num_chars: model.character_slots as u32,
        }
    }
}
