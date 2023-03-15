use moople_packet::{EncodePacket, HasOpcode, MaplePacketWriter};
use proto95::{
    game::mob::{
        CarnivalTeam, LocalMobData, MobChangeControllerResp, MobEnterFieldResp, MobId, MobInitData,
        MobLeaveFieldResp, MobLeaveType, MobSummonType, MobTemporaryStatPartial,
        PartialMobTemporaryStat,
    },
    shared::{FootholdId, Vec2},
};

use crate::services::session::session_set::SharedSessionDataRef;

use super::{Pool, PoolItem};

#[derive(Debug)]
pub struct Mob {
    pub tmpl_id: MobId,
    pub pos: Vec2,
    pub fh: FootholdId,
    pub origin_fh: Option<FootholdId>,
}

impl PoolItem for Mob {
    type Id = u32;

    type EnterPacket = MobEnterFieldResp;

    type LeavePacket = MobLeaveFieldResp;

    type LeaveParam = MobLeaveType;

    fn get_enter_pkt(&self, id: Self::Id) -> Self::EnterPacket {
        let empty_stats = PartialMobTemporaryStat {
            hdr: (),
            data: MobTemporaryStatPartial {
                ..Default::default()
            },
        };

        MobEnterFieldResp {
            id,
            calc_dmg_index: 5,
            tmpl_id: self.tmpl_id,
            stats: empty_stats,
            init_data: MobInitData {
                pos: self.pos,
                move_action: 3,
                fh: self.fh,
                origin_fh: self.origin_fh.unwrap_or(0), //char_fh
                summon_type: MobSummonType::Normal(()),
                carnival_team: CarnivalTeam::None,
                effect_id: 0,
                phase: 0,
            },
        }
    }

    fn get_leave_pkt(&self, id: Self::Id, param: Self::LeaveParam) -> Self::LeavePacket {
        MobLeaveFieldResp {
            id,
            leave_type: param,
        }
    }
}

impl Pool<Mob> {
    pub async fn assign_controller(&self, session: SharedSessionDataRef) -> anyhow::Result<()> {
        //TODO move out loop
        for (id, mob) in self.items.read().await.iter() {
            let empty_stats = PartialMobTemporaryStat {
                hdr: (),
                data: MobTemporaryStatPartial {
                    ..Default::default()
                },
            };

            let mut pw = MaplePacketWriter::default();
            pw.write_opcode(MobChangeControllerResp::OPCODE);
            MobChangeControllerResp {
                level: 1,
                //seed: CrcSeed::default(),
                id: *id,
                local_mob_data: Some(LocalMobData {
                    calc_damage_index: 5,
                    tmpl_id: mob.tmpl_id,
                    stats: empty_stats,
                })
                .into(),
            }
            .encode_packet(&mut pw)?;
            session.broadcast_tx.send(pw.into_packet()).await?;
        }
        Ok(())
    }
}
