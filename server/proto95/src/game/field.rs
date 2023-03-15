use moople_derive::MooplePacket;
use moople_packet::{
    maple_packet_enum, packet_opcode,
    proto::{
        list::{MapleList, MapleListLen},
        option::MapleOption8,
        time::MapleTime,
        MapleList16,
    },
};

use crate::{
    id::MapId,
    send_opcodes::SendOpcodes,
    shared::{char::{CharacterId, CharDataAll, CharDataFlagsAll, CharDataHeader}, TagPoint},
};

#[derive(MooplePacket, Debug)]
pub struct ClientOption {
    pub key: u32,
    pub value: u32,
}

#[derive(MooplePacket, Debug, Default)]
pub struct CrcSeed {
    pub s1: u32,
    pub s2: u32,
    pub s3: u32,
}

#[derive(MooplePacket, Debug)]
pub struct LogoutGiftConfig {
    pub predict_quit: u32,
    pub gift_commodity_id: [u32; 3],
}

#[derive(MooplePacket, Debug, Clone, Copy, PartialEq, Eq)]
pub struct PlusOneListIndex(pub u16);

impl MapleListLen for PlusOneListIndex {
    fn to_len(&self) -> usize {
        match self.0 {
            0 => 0,
            n => (n + 1) as usize,
        }
    }

    fn from_len(ix: usize) -> Self {
        PlusOneListIndex(ix as u16)
    }
}

pub type NotificationList = MapleList<PlusOneListIndex, String>;

#[derive(MooplePacket, Debug)]
pub struct SetFieldCharData {
    pub notifications: NotificationList,
    pub seed: CrcSeed,
    pub char_data_flags: CharDataFlagsAll,
    pub char_data_hdr: CharDataHeader,
    pub char_data: CharDataAll,
    pub logout_gift_config: LogoutGiftConfig,
}

#[derive(MooplePacket, Debug)]
pub struct SetFieldOtherData {
    pub notifications: NotificationList,
    pub map: MapId,
    pub portal: u8,
    pub hp: u32,
    pub chase_target_pos: MapleOption8<TagPoint>,
}

impl SetFieldOtherData {
    pub fn is_chase_enabled(&self) -> bool {
        self.chase_target_pos.opt.is_some()
    }
}

maple_packet_enum!(
    SetFieldResult,
    u8,
    TransferField(SetFieldOtherData) => 0,
    CharData(SetFieldCharData) => 1,
);

#[derive(MooplePacket, Debug)]
pub struct SetFieldResp {
    pub client_option: MapleList16<ClientOption>,
    pub channel_id: u32,
    pub old_driver_id: CharacterId,
    pub unknown_flag_1: u8,
    pub set_field_result: SetFieldResult,
    pub timestamp: MapleTime,
    pub extra: u32,
}
packet_opcode!(SetFieldResp, SendOpcodes::SetField);
