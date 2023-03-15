use bytes::BufMut;
use moople_derive::MooplePacket;
use moople_packet::{
    maple_packet_enum,
    proto::{time::MapleTime, DecodePacket, EncodePacket, PacketLen, option::MapleOption8, CondOption},
    NetResult, mark_maple_bit_flags,
};

use crate::id::ItemId;

use super::NameStr;

bitflags::bitflags! {
    #[derive(Copy, Clone, Debug, PartialEq, Eq)]
    pub struct ItemFlags : u16 {
        const Protected = 0x01;
        const PreventSlipping = 0x02;
        const PreventColdness = 0x04;
        const Untradeable = 0x08;
        const ScissorsApplied = 0x10;
        const Sandbox = 0x40;
        const PetCome = 0x80;
        const AccountSharing = 0x100;
        const MergeUntradeable = 0x200;
    }
}
mark_maple_bit_flags!(ItemFlags);


bitflags::bitflags! {
    #[derive(Copy, Clone, Debug, PartialEq, Eq)]
    pub struct ItemBundleFlags : u16 {
        const Protected = 0x01;
        const TradingPossible = 0x02;
    }
}
mark_maple_bit_flags!(ItemBundleFlags);

bitflags::bitflags! {
    #[derive(Copy, Clone, Debug, PartialEq, Eq)]
    pub struct ItemPetFlags : u16 {
        const Protected = 0x01;
        const TradingPossible = 0x02;
    }
}
mark_maple_bit_flags!(ItemPetFlags);

bitflags::bitflags! {
    #[derive(Copy, Clone, Debug, PartialEq, Eq)]
    pub struct ItemEquipFlags : u16 {    
        const Protected = 0x01;
        const PreventSlipping = 0x02;
        const SupportWarm = 0x04;
        const Binded = 0x08;
        //const TradingPossible = 0x1;
    }
}
mark_maple_bit_flags!(ItemEquipFlags);

#[derive(Debug, MooplePacket)]
pub struct PetItemInfo {
    pub name: NameStr,
    pub level: u8,
    pub tameness: u16,
    pub fullness: u8, /* repleteness */
    pub expiration_time: MapleTime, /* dateDead */
    pub attribute1: u16, /* PetAttribute  seems to be only hasStats 2^0*/
    pub skill: u16,
    pub remain_life: u32,
    pub attribute2: u16, /* Attribute  Only IsPossibleTrading 2^0 */
}
#[derive(Debug, MooplePacket)]
pub struct EquipStats {
    pub str: u16,
    pub dex: u16,
    pub int: u16,
    pub luk: u16,
    pub hp: u16,
    pub mp: u16,
    pub watk: u16,
    pub matk: u16,
    pub wdef: u16,
    pub mdef: u16,
    pub accuracy: u16,
    pub avoid: u16,
    pub craft: u16,
    pub speed: u16,
    pub jump: u16,
}

#[derive(Debug, MooplePacket)]
pub struct EquipAllStats {
    pub remaining_upgrade_slots: u8,
    pub upgrade_count: u8,
    pub stats: EquipStats,
    pub title: String, /* stitle */
    pub flags: ItemFlags,
}

#[derive(Debug, MooplePacket)]
pub struct ItemInfo {
    pub item_id: ItemId,
    pub cash_id: MapleOption8<u64>,
    pub expiration: MapleTime,
}

impl ItemInfo {
    pub fn is_rechargable(&self) -> bool {
        self.item_id.is_rechargable()
    }
}

#[derive(Debug, MooplePacket)]
pub struct ItemPetData {
    pub info: ItemInfo,
    pub name: NameStr,
    pub level: u8,
    pub tameness: u16,
    pub fullness: u8,
    pub expiration_time: MapleTime,
    pub attribute1: u16,
    pub skill: u16,
    pub remain_life: u32,
    pub attribute2: u16,
}

#[derive(Debug, MooplePacket)]
pub struct ItemStackData {
    pub info: ItemInfo,
    pub quantity: u16, /* nNumber */
    pub title: String,
    pub flag: ItemFlags,
    #[pkt(if(field = "info", cond = "ItemInfo::is_rechargable"))]
    pub serial_number: CondOption<u64>/* liSN */
}

#[derive(Debug)]
pub struct OptionalLevelInfo(pub Option<ItemLevelInfo>);

impl<'de> DecodePacket<'de> for OptionalLevelInfo {
    fn decode_packet(pr: &mut moople_packet::MaplePacketReader<'de>) -> NetResult<Self> {
        let ty = pr.read_u8()?;
        let item_info = match ty {
            0x40 => {
                //In total 10 0x40 bytes
                pr.read_array::<9>()?;
                None
            }
            _ => Some(ItemLevelInfo::decode_packet(pr)?),
        };

        Ok(Self(item_info))
    }
}

impl EncodePacket for OptionalLevelInfo {
    fn encode_packet<B: BufMut>(
        &self,
        pw: &mut moople_packet::MaplePacketWriter<B>,
    ) -> NetResult<()> {
        match self.0 {
            Some(ref info) => {
                pw.write_u8(0x00);
                info.encode_packet(pw)?;
            }
            None => pw.write_array(&[0x40; 10]),
        };

        Ok(())
    }
}

impl PacketLen for OptionalLevelInfo {
    const SIZE_HINT: Option<usize> = None;

    fn packet_len(&self) -> usize {
        self.0
            .as_ref()
            .map(|info| info.packet_len() + 1)
            .unwrap_or(10)
    }
}

#[derive(Debug, MooplePacket)]
pub struct ItemLevelInfo {
    pub level: u8,
    pub exp: u32,
    pub vicious: u32,
    pub unknown2: u64, /* nIUC(4) + Durability(4) */
}

#[derive(Debug, MooplePacket)]
pub struct EquipItemInfo {
    pub info: ItemInfo,
    pub stats: EquipAllStats,

    pub lvl_up_ty: u8,
    pub lvl: u8,
    pub exp: u32,
    pub durability: i32,
    pub hammer_count: u32,
    pub grade: u8,
    pub stars: u8,
    pub options: [u16; 3],
    pub sockets: [u16; 2],

    pub sn: u64,

    /*
      if ((*(uint *)&this->field_0x18 | *(uint *)&this->field_0x1c) == 0) {
    COutPacket::EncodeBuffer(param_1,&this->liSN,8);
  } */


  pub time_stamp: MapleTime, // ftEquipped
  pub prev_bonus_exp_rate: i32, // nPrevBonusExpRate ?


}

maple_packet_enum!(
    Item,
    u8,
    Equip(EquipItemInfo) => 1,
    Stack(ItemStackData) => 2,
    Pet(ItemPetData) => 3,
    Equipped(()) => 255
);


