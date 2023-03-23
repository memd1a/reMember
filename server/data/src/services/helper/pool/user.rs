use moople_packet::proto::list::{MapleIndexList8, MapleIndexListZ};
use proto95::{
    game::{
        user::remote::{
            GuildMarkData, TamingMobData, UserEnterFieldResp, UserLeaveFieldResp,
            UserRemoteInitData,
        },
        ObjectId,
    },
    id::{job_id::JobId, FaceId, HairId, ItemId, Skin},
    shared::{
        char::{AvatarData, AvatarEquips, CharacterId, PetIds, RemoteCharSecondaryStatPartial},
        Gender, Vec2,
    },
};

use super::PoolItem;

#[derive(Debug)]
pub struct User {
    pub char_id: CharacterId,
    pub pos: Vec2,
    pub fh: u16,
}

impl PoolItem for User {
    type Id = ObjectId;

    type EnterPacket = UserEnterFieldResp;

    type LeavePacket = UserLeaveFieldResp;

    type LeaveParam = ();

    fn get_enter_pkt(&self,  _id: Self::Id) -> Self::EnterPacket {
        let secondary_stat = RemoteCharSecondaryStatPartial {
            //shadowpartner: Some(4111002).into(),
            darksight: Some(()).into(),
            curse: Some(1000).into(),
            ..Default::default()
        };

        let avatar = AvatarData {
            gender: Gender::Female,
            skin: Skin::Normal,
            mega: true,
            face: FaceId::MOTIVATED_LOOK_F,
            hair: HairId::ZETA,
            equips: AvatarEquips {
                equips: vec![
                    (5, ItemId(1040006)),
                    (6, ItemId(1060006)),
                    (7, ItemId(1072005)),
                    (11, ItemId(1322005)),
                ]
                .into(),
                masked_equips: MapleIndexList8::default(),
                weapon_sticker_id: ItemId(0),
            },
            pets: PetIds::default(),
        };

        UserEnterFieldResp {
            char_id: self.char_id,
            user_init_data: UserRemoteInitData {
                level: 30,
                name: self.char_id.to_string(),
                guild_name: "Eden".to_string(),
                guild_mark: GuildMarkData::default(),
                secondary_stat: secondary_stat.into(),
                avatar,
                driver_id: 0,
                passenger_id: 0,
                choco_count: 0,
                active_effect_item: ItemId(0),
                completed_set_item_id: ItemId(0),
                portable_chair: ItemId(0),
                pos: self.pos,
                fh: self.fh,
                show_admin_effects: false,
                pet_infos: MapleIndexListZ::default(),
                taming_mob: TamingMobData::default(),
                mini_room: None.into(),
                ad_board: None.into(),
                couple: None.into(),
                friendship: None.into(),
                marriage: None.into(),
                load_flags: 0,
                new_year_cards: None.into(),
                phase: 0,
                defense_att: 0,
                defense_state: 0,
                job: JobId::Bandit,
                move_action: 0,
            },
        }
    }

    fn get_leave_pkt(&self, _id: Self::Id, _param: Self::LeaveParam) -> Self::LeavePacket {
        UserLeaveFieldResp {
            char_id: self.char_id,
        }
    }
}
