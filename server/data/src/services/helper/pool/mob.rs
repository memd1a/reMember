use futures::SinkExt;
use moople_net::service::{packet_buffer::PacketBuffer, session_svc::SharedSessionHandle};
use moople_packet::{EncodePacket, HasOpcode, MaplePacketWriter};
use proto95::{
    game::{
        mob::{
            CarnivalTeam, LocalMobData, MobChangeControllerResp, MobDamagedResp, MobEnterFieldResp,
            MobHPIndicatorResp, MobId, MobInitData, MobLeaveFieldResp, MobLeaveType, MobMoveReq,
            MobMoveResp, MobSummonType, MobTemporaryStatPartial, PartialMobTemporaryStat,
        },
        ObjectId,
    },
    shared::{FootholdId, Vec2},
};

use crate::services::{
    data::character::CharacterID,
    meta::meta_service::MobMeta,
    session::session_set::{SessionSet, SharedSessionDataRef},
};

use super::{next_id, Pool, PoolItem};

#[derive(Debug)]
pub struct Mob {
    pub meta: MobMeta,
    pub tmpl_id: MobId,
    pub pos: Vec2,
    pub fh: FootholdId,
    pub origin_fh: Option<FootholdId>,
    pub hp: u32,
    pub perc: u8,
}

impl Mob {
    pub fn damage(&mut self, dmg: u32) {
        self.hp = self.hp.saturating_sub(dmg);
        self.perc = ((self.hp * 100) / self.meta.max_hp) as u8;
    }

    pub fn is_dead(&self) -> bool {
        self.hp == 0
    }
}

impl PoolItem for Mob {
    type Id = u32;

    type EnterPacket = MobEnterFieldResp;

    type LeavePacket = MobLeaveFieldResp;

    type LeaveParam = MobLeaveType;

    fn get_id(&self) -> Self::Id {
        next_id()
    }

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
    pub async fn assign_controller(&self, mut session: SharedSessionHandle) -> anyhow::Result<()> {
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
            session.tx.send(pw.into_packet().data).await?;
        }
        Ok(())
    }

    pub async fn attack_mob(
        &self,
        id: ObjectId,
        dmg: u32,
        buf: &mut PacketBuffer,
    ) -> anyhow::Result<bool> {
        // TODO: Locking the whole pool to update a single mob is not right
        let mut mobs = self.items.write().await;
        let mob = mobs
            .get_mut(&id)
            .ok_or(anyhow::format_err!("Invalid mob"))?;
        mob.damage(dmg);

        buf.write_packet(MobDamagedResp {
            id,
            ty: 0,
            dec_hp: dmg,
            hp: mob.hp,
            max_hp: mob.meta.max_hp,
        })?;

        buf.write_packet(MobHPIndicatorResp {
            id,
            hp_perc: mob.perc,
        })?;

        Ok(mob.is_dead())
    }

    pub fn mob_move(
        &self,
        id: ObjectId,
        req: MobMoveReq,
        controller: CharacterID,
        sessions: &SessionSet,
    ) -> anyhow::Result<()> {
        let pkt = MobMoveResp {
            id,
            not_force_landing: false,
            not_change_action: false,
            next_attack_possible: false,
            action_dir: req.action_dir,
            data: req.data,
            multi_target: req.multi_target,
            rand_time: req.rand_time,
            move_path: req.move_path.path,
        };

        sessions.broadcast_pkt(pkt, controller)?;
        Ok(())
    }
}
