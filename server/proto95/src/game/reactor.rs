use moople_derive::MooplePacket;
use moople_packet::{packet_opcode, proto::time::MapleDurationMs16};

use crate::{send_opcodes::SendOpcodes, shared::Vec2, recv_opcodes::RecvOpcodes};

use super::ObjectId;

pub type ReactorId = u32;

#[derive(MooplePacket, Debug)]
pub struct ReactorEnterFieldResp {
    pub id: ObjectId,
    pub tmpl_id: ReactorId,
    pub state: u8,
    pub pos: Vec2,
    pub flipped: bool,
    pub name: String
}
packet_opcode!(ReactorEnterFieldResp, SendOpcodes::ReactorEnterField);

#[derive(MooplePacket, Debug)]
pub struct ReactorLeaveFieldResp {
    pub id: ObjectId,
    pub state: u8,
    pub pos: Vec2
}
packet_opcode!(ReactorLeaveFieldResp, SendOpcodes::ReactorLeaveField);

#[derive(MooplePacket, Debug)]
pub struct ReactorMoveResp {
    pub id: ObjectId,
    pub pos: Vec2
}
packet_opcode!(ReactorMoveResp, SendOpcodes::ReactorMove);

#[derive(MooplePacket, Debug)]
pub struct ReactorChangeStateResp {
    pub id: ObjectId,
    pub state: u8,
    pub pos: Vec2,
    pub animation_delay: MapleDurationMs16,
    pub proper_event_id: u8,
    pub end_state: u8
}
packet_opcode!(ReactorChangeStateResp, SendOpcodes::ReactorChangeState);

#[derive(MooplePacket, Debug)]
pub struct ReactorHitReq {
    pub id: ObjectId,
    pub skill_reactor: u32,
    pub hit_option: u32,
    pub action_delay: MapleDurationMs16,
    pub skill_id: u32

}
packet_opcode!(ReactorHitReq, RecvOpcodes::ReactorHit);

#[derive(MooplePacket, Debug)]
pub struct ReactorTouchReq {
    pub id: ObjectId,
    pub has_reactor: bool, // TODO

}
packet_opcode!(ReactorTouchReq, RecvOpcodes::ReactorTouch);