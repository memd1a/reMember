use moople_derive::MooplePacket;
use moople_packet::{packet_opcode, proto::time::Ticks};

use crate::{recv_opcodes::RecvOpcodes, shared::Vec2};

use super::ObjectId;

pub type PetLockerId = u64;
pub type PetId = u32;
pub type PetIx = u8;

#[derive(MooplePacket, Debug)]
pub struct PetDropPickUpReq {
    pub locker_id: PetLockerId,
    pub u1: u8,// Pet id?
    pub ticks: Ticks,
    pub point: Vec2,
    pub drop_id: ObjectId,
    pub drop_crc: u32,
    pub pickup_others: bool,
    pub sweep_for_drop: bool,
    pub long_range: bool,
    // TOdo: drop_id / 0xd * 0xd == drop_id, figure this out
    pub drop_pos: Vec2,
    pub pos_crc: u32,
    pub rect_crc: u32


    
}
packet_opcode!(PetDropPickUpReq, RecvOpcodes::PetDropPickUpRequest);