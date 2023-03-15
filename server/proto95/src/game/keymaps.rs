use moople_derive::MooplePacket;
use moople_packet::{packet_opcode, proto::option::MapleOptionR8};

use crate::send_opcodes::SendOpcodes;

#[derive(Debug, MooplePacket, Default, Clone, Copy)]
pub struct KeyBinding {
    pub ty: u8,
    pub action_id: u32,
}

#[derive(Debug, MooplePacket)]
pub struct FuncKeyMapInitResp {
    // Reversed option, if set to none the default key map is used
    pub key_bindings: MapleOptionR8<[KeyBinding; 90]>,
}

impl FuncKeyMapInitResp {
    pub fn default_map() -> Self {
        Self { key_bindings: None.into() }
    }
}

packet_opcode!(FuncKeyMapInitResp, SendOpcodes::FuncKeyMappedInit);
