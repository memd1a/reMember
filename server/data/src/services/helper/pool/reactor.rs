use proto95::{shared::{char::CharacterId, Vec2}, game::{ObjectId, reactor::{ReactorEnterFieldResp, ReactorLeaveFieldResp, ReactorId}}};

use super::PoolItem;

#[derive(Debug)]
pub struct Reactor {
    pub pos: Vec2,
    pub tmpl_id: ReactorId,
    pub state: u8
}

impl PoolItem for Reactor {
    type Id = ObjectId;

    type EnterPacket = ReactorEnterFieldResp;

    type LeavePacket = ReactorLeaveFieldResp;

    type LeaveParam = ();

    fn get_enter_pkt(&self, id: Self::Id) -> Self::EnterPacket {
        ReactorEnterFieldResp {
            id,
            tmpl_id: self.tmpl_id,
            state: self.state,
            pos: self.pos,
            flipped: false,
            name: String::new()
        }
    }

    fn get_leave_pkt(&self, id: Self::Id, _param: Self::LeaveParam) -> Self::LeavePacket {
        ReactorLeaveFieldResp {
            id: id,
            state: self.state,
            pos: self.pos,
        }
    }
}