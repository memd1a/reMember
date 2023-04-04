use std::{arch::asm, ffi::c_void, sync::LazyLock};

use detour::GenericDetour;

use crate::{
    config::addr,
    packet_struct::{PacketStructElem, RECV_PACKET_CTX, SEND_PACKET_CTX},
    ret_addr, return_address, static_ms_fn_hook,
    ztl::{zarr::ZArray, zxstr::ZXString8},
};

pub trait CPacket {
    const DATA_OFFSET: usize;

    fn get_data_arr(&self) -> &ZArray<u8>;

    fn get_len(&self) -> usize;

    fn get_data(&self) -> &[u8] {
        &self.get_data_arr().get_data()[Self::DATA_OFFSET..Self::DATA_OFFSET + self.get_len()]
    }

    fn get_opcode(&self) -> u16 {
        let data = self.get_data();
        u16::from_le_bytes(data[..2].try_into().unwrap())
    }

    fn get_offset(&self) -> usize;
}

#[derive(Debug)]
#[repr(C, packed)]
pub struct COutPacket {
    pub is_loopback: i32,
    pub send_buf: ZArray<u8>,
    pub offset: u32,
    pub is_encrypted_by_shanda: bool,
}

impl CPacket for COutPacket {
    const DATA_OFFSET: usize = 0;
    fn get_len(&self) -> usize {
        self.offset as usize
    }

    fn get_data_arr(&self) -> &ZArray<u8> {
        &self.send_buf
    }

    fn get_offset(&self) -> usize {
        self.offset as usize
    }
}

#[derive(Debug)]
#[repr(C)]
pub struct CInPacket {
    pub is_loopback: i32,
    pub state: i32,
    pub recv_buf: ZArray<u8>,
    pub len: u16,
    pub raw_seq: u16,
    pub data_len: u16,
    pub offset: u32,
}

impl CPacket for CInPacket {
    const DATA_OFFSET: usize = 4;
    fn get_len(&self) -> usize {
        self.data_len as usize
    }

    fn get_data_arr(&self) -> &ZArray<u8> {
        &self.recv_buf
    }

    fn get_offset(&self) -> usize {
        self.offset as usize - Self::DATA_OFFSET
    }
}

type PClientSocket = *mut c_void;

static_ms_fn_hook!(
    SEND_PACKET_HOOK,
    addr::SOCKET_SEND_PACKET,
    send_packet_detour,
    type FClientSocketSendPacket = unsafe extern "thiscall" fn(PClientSocket, *mut COutPacket)
);

static_ms_fn_hook!(
    PROCESS_PACKET_HOOK,
    addr::SOCKET_PROCESS_PACKET,
    process_packet_detour,
    type FClientSocketProcessPacket = unsafe extern "thiscall" fn(PClientSocket, *mut CInPacket)
);

/*
   Hacky way to spoof there return address,
   by basically abusing the fact that fastcall is the same as thiscall
   but passes an extra parameter via edx
   so we move the packet from edx onto the stack
   and then fake the return addy
*/
#[naked]
unsafe extern "fastcall" fn send_packet_trampoline(this: PClientSocket, pkt: *mut COutPacket) {
    unsafe {
        asm!(
            // Push packet param
            "push edx",
            // Push fake return address -> fake ret addy
            "push {1}",
            // Patched bytes for detour jump
            "push ebp",
            "mov ebp, esp",
            "push 0xffffffff",
            // Load address for jump
            "mov eax, {0}",
            "jmp eax",
            const addr::SOCKET_SEND_PACKET_TRAMPOLINE_ENTRY,
            const addr::SOCKET_SINGLETON_SEND_PACKET_RET,
            options(noreturn)
        );
    }
}

extern "thiscall" fn send_packet_detour(this: PClientSocket, ppkt: *mut COutPacket) {
    let ret = ret_addr!();
    let pkt = unsafe { ppkt.as_ref() }.unwrap();

    SEND_PACKET_CTX.finish(ppkt.addr(), ret, pkt.get_data());

    unsafe { send_packet_trampoline(this, ppkt) }
}
extern "thiscall" fn process_packet_detour(this: PClientSocket, ppkt: *mut CInPacket) {
    RECV_PACKET_CTX.clear();
    RECV_PACKET_CTX.store_ptr(ppkt);
    let ret = ret_addr!();
    let pkt = unsafe { ppkt.as_ref() }.unwrap();

    unsafe { PROCESS_PACKET_HOOK.call(this, ppkt) }

    RECV_PACKET_CTX.finish(ppkt.addr(), ret, pkt.get_data());
}

static_ms_fn_hook!(
    ENCODE_1_HOOK,
    addr::PACKET_ENCODE1,
    encode1_detour,
    type FPacketEncode1 = unsafe extern "thiscall" fn(*mut COutPacket, u8)
);

extern "thiscall" fn encode1_detour(this: *mut COutPacket, v: u8) {
    let ret = ret_addr!();
    let off = unsafe { this.as_ref().unwrap() }.get_offset();
    SEND_PACKET_CTX.add_elem(this.addr(), PacketStructElem::i8(off, ret));

    unsafe { ENCODE_1_HOOK.call(this, v) }
}

static_ms_fn_hook!(
    ENCODE_2_HOOK,
    addr::PACKET_ENCODE2,
    encode2_detour,
    type FPacketEncode2 = unsafe extern "thiscall" fn(*mut COutPacket, u16)
);

extern "thiscall" fn encode2_detour(this: *mut COutPacket, v: u16) {
    let ret = ret_addr!();
    let off = unsafe { this.as_ref().unwrap() }.get_offset();
    SEND_PACKET_CTX.add_elem(this.addr(), PacketStructElem::i16(off, ret));

    unsafe { ENCODE_2_HOOK.call(this, v) }
}

static_ms_fn_hook!(
    ENCODE_4_HOOK,
    addr::PACKET_ENCODE4,
    encode4_detour,
    type FPacketEncode4 = unsafe extern "thiscall" fn(*mut COutPacket, u32)
);

extern "thiscall" fn encode4_detour(this: *mut COutPacket, v: u32) {
    let ret = ret_addr!();
    let off = unsafe { this.as_ref().unwrap() }.get_offset();
    SEND_PACKET_CTX.add_elem(this.addr(), PacketStructElem::i32(off, ret));

    unsafe { ENCODE_4_HOOK.call(this, v) }
}

static_ms_fn_hook!(
    ENCODE_STR_HOOK,
    addr::PACKET_ENCODE_STR,
    encode_str_detour,
    type FPacketEncodeStr = unsafe extern "thiscall" fn(*mut COutPacket, ZXString8)
);

extern "thiscall" fn encode_str_detour(this: *mut COutPacket, v: ZXString8) {
    let ret = ret_addr!();
    let off = unsafe { this.as_ref().unwrap() }.get_offset();
    SEND_PACKET_CTX.add_elem(this.addr(), PacketStructElem::str(off, ret, v.len()));

    unsafe { ENCODE_STR_HOOK.call(this, v) }
}

static_ms_fn_hook!(
    ENCODE_BUF_HOOK,
    addr::PACKET_ENCODE_BUF,
    encode_buf_detour,
    type FPacketEncodeBuf = unsafe extern "thiscall" fn(*mut COutPacket, *const u8, u32)
);

extern "thiscall" fn encode_buf_detour(this: *mut COutPacket, v: *const u8, n: u32) {
    let ret = ret_addr!();
    let off = unsafe { this.as_ref().unwrap() }.get_offset();
    SEND_PACKET_CTX.add_elem(this.addr(), PacketStructElem::buf(off, ret, n as usize));

    unsafe { ENCODE_BUF_HOOK.call(this, v, n) }
}

// Begin decode

static_ms_fn_hook!(
    DECODE_1_HOOK,
    addr::PACKET_DECODE1,
    decode1_detour,
    type FPacketDecode1 = unsafe extern "thiscall" fn(*mut CInPacket) -> u8
);

extern "thiscall" fn decode1_detour(this: *mut CInPacket) -> u8 {
    let ret = ret_addr!();
    let off = unsafe { this.as_ref().unwrap() }.get_offset();
    RECV_PACKET_CTX.add_elem(this.addr(), PacketStructElem::i8(off, ret));

    unsafe { DECODE_1_HOOK.call(this) }
}

static_ms_fn_hook!(
    DECODE_2_HOOK,
    addr::PACKET_DECODE2,
    decode2_detour,
    type FPacketDecode2 = unsafe extern "thiscall" fn(*mut CInPacket) -> u16
);

extern "thiscall" fn decode2_detour(this: *mut CInPacket) -> u16 {
    let ret = ret_addr!();
    let off = unsafe { this.as_ref().unwrap() }.get_offset();
    RECV_PACKET_CTX.add_elem(this.addr(), PacketStructElem::i16(off, ret));

    unsafe { DECODE_2_HOOK.call(this) }
}

static_ms_fn_hook!(
    DECODE_4_HOOK,
    addr::PACKET_DECODE4,
    decode4_detour,
    type FPacketDecode4 = unsafe extern "thiscall" fn(*mut CInPacket) -> u32
);

extern "thiscall" fn decode4_detour(this: *mut CInPacket) -> u32 {
    let ret = ret_addr!();
    let off = unsafe { this.as_ref().unwrap() }.get_offset();
    RECV_PACKET_CTX.add_elem(this.addr(), PacketStructElem::i32(off, ret));

    unsafe { DECODE_4_HOOK.call(this) }
}

static_ms_fn_hook!(
    DECODE_STR_HOOK,
    addr::PACKET_DECODE_STR,
    decode_str_detour,
    type FPacketDecodeStr = unsafe extern "thiscall" fn(*mut CInPacket, *mut ZXString8) -> *mut ZXString8
);

extern "thiscall" fn decode_str_detour(this: *mut CInPacket, v: *mut ZXString8) -> *mut ZXString8 {
    let ret = ret_addr!();
    let off = unsafe { this.as_ref().unwrap() }.get_offset();

    let s = unsafe { DECODE_STR_HOOK.call(this, v) };
    let str = unsafe { s.as_ref().unwrap() };
    RECV_PACKET_CTX.add_elem(
        this.addr(),
        PacketStructElem::str(off, ret, str.len()),
    );
    s
}

static_ms_fn_hook!(
    DECODE_BUF_HOOK,
    addr::PACKET_DECODE_BUF,
    decode_buf_detour,
    type FPacketDecodeBuf = unsafe extern "thiscall" fn(*mut CInPacket, *mut u8, u32)
);

extern "thiscall" fn decode_buf_detour(this: *mut CInPacket, v: *mut u8, n: u32) {
    let ret = ret_addr!();
    let off = unsafe { this.as_ref().unwrap() }.get_offset();
    RECV_PACKET_CTX.add_elem(this.addr(), PacketStructElem::buf(off, ret, n as usize));

    unsafe { DECODE_BUF_HOOK.call(this, v, n) }
}

// End decode

pub unsafe fn init_hooks() -> anyhow::Result<()> {
    log::info!("Hooking socket...");

    SEND_PACKET_HOOK.enable()?;
    PROCESS_PACKET_HOOK.enable()?;

    ENCODE_1_HOOK.enable()?;
    ENCODE_2_HOOK.enable()?;
    ENCODE_4_HOOK.enable()?;
    ENCODE_BUF_HOOK.enable()?;
    ENCODE_STR_HOOK.enable()?;

    DECODE_1_HOOK.enable()?;
    DECODE_2_HOOK.enable()?;
    DECODE_4_HOOK.enable()?;
    DECODE_BUF_HOOK.enable()?;
    DECODE_STR_HOOK.enable()?;

    log::info!("Socket hooked");
    Ok(())
}
