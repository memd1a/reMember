pub mod char;
use moople_packet::proto::time::MapleTime;
use proto95::{shared::Gender, login::account::{LoginAccountInfo, LoginAccountExtraInfo}};

use crate::entities::{sea_orm_active_enums::GenderTy, account};

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


impl From<&account::Model> for LoginAccountInfo {
    fn from(model: &account::Model) -> Self {
        let gm = model.gm_level as u8;
        //TODO: pin and such could come from another source so this from needs more data
        let extra_info = model.gender.is_some().then_some(LoginAccountExtraInfo {
            skip_pin: false,
            login_opt: proto95::login::LoginOpt::EnableSecondPassword,
            client_key: [0; 8]
        }).into();
    
    
        LoginAccountInfo {
            id: model.id as u32,
            gender: model.gender.as_ref().into(),
            grade_code: gm,
            sub_grade_code: gm,
            is_test_acc: false,
            country_id: model.country as u8,
            name: model.username.clone(),
            purchase_exp: 0,
            chat_block_reason: 0,
            chat_block_date: MapleTime::zero(),
            registration_date: MapleTime::try_from(model.created_at).unwrap(),
            num_chars: 3,
            extra_info
        }
    }
}