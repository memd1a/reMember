use proto95::{
    game::{
        npc::{NpcEnterFieldResp, NpcId, NpcInitData, NpcLeaveFieldResp},
        ObjectId,
    },
    shared::{FootholdId, Range2, Vec2},
};

use super::{PoolItem, next_id};

#[derive(Debug)]
pub struct Npc {
    pub tmpl_id: NpcId,
    pub pos: Vec2,
    pub fh: FootholdId,
    pub move_action: u8,
    pub range_horz: Range2,
    pub enabled: bool,
}

impl PoolItem for Npc {
    type Id = ObjectId;

    type EnterPacket = NpcEnterFieldResp;

    type LeavePacket = NpcLeaveFieldResp;

    type LeaveParam = ();

    fn get_id(&self) -> Self::Id {
        next_id()
    }

    fn get_enter_pkt(&self, id: Self::Id) -> Self::EnterPacket {
        NpcEnterFieldResp {
            id,
            template_id: self.tmpl_id,
            init: NpcInitData {
                pos: self.pos,
                move_action: self.move_action,
                fh: self.fh,
                range_horz: self.range_horz,
                enabled: self.enabled,
            },
        }
    }

    fn get_leave_pkt(&self, id: Self::Id, _param: Self::LeaveParam) -> Self::LeavePacket {
        NpcLeaveFieldResp { id }
    }
}
