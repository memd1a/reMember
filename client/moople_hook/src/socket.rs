use std::{arch::asm, ffi::c_void};

use detour::static_detour;

use crate::{
    fn_ref, fn_ref_hook,
    packet_struct::{PacketStructElem, RECV_PACKET_CTX, SEND_PACKET_CTX},
    return_address,
    ztl::{zarr::ZArray, zxstr::ZXString8},
};

pub trait CPacket {
    fn get_data_arr(&self) -> &ZArray<u8>;

    fn get_len(&self) -> usize;

    fn get_data(&self) -> &[u8] {
        &self.get_data_arr().get_data()[..self.get_len()]
    }

    fn get_opcode(&self) -> u16 {
        let data = self.get_data();
        u16::from_le_bytes(data[..2].try_into().unwrap())
    }
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
    fn get_len(&self) -> usize {
        self.offset as usize
    }

    fn get_data_arr(&self) -> &ZArray<u8> {
        &self.send_buf
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
    fn get_len(&self) -> usize {
        self.data_len as usize
    }

    fn get_data_arr(&self) -> &ZArray<u8> {
        &self.recv_buf
    }

    fn get_data(&self) -> &[u8] {
        &self.get_data_arr().get_data()[4..4 + self.get_len()]
    }
}

type PClientSocket = *const c_void;

fn_ref_hook!(
    client_socket_send_packet,
    FClientSocketSendPacket,
    send_packet_addr,
    0x004af9f0,
    SendPacketHook,
    unsafe extern "thiscall" fn(PClientSocket, *mut COutPacket)
);

fn_ref_hook!(
    client_socket_process_packet,
    FClientSocketProcessPacket,
    process_packet_addr,
    0x004b00f0,
    ProcessPacketHook,
    unsafe extern "thiscall" fn(PClientSocket, *mut CInPacket)
);

const TRAMPOLINE_RET: usize = 0x00429b90;
const TRAMPOLINE_ENTRY: usize = 0x4af9f0 + 5;

// Hacky way abusing the fact that first two params are passed in ecx and edx for fastcall
#[naked]
unsafe extern "fastcall" fn send_packet_trampoline(this: PClientSocket, pkt: *mut COutPacket) {
    unsafe {
        asm!(
            // Push packet param
            "push edx",
            // Push fake return address
            "push {1}",
            // Patched bytes for detour jump
            "push ebp",
            "mov ebp, esp",
            "push 0xffffffff",
            // Load address for jump
            "mov eax, {0}",
            "jmp eax",
            const TRAMPOLINE_ENTRY,
            const TRAMPOLINE_RET,
            options(noreturn)
        );
    }
}

fn send_packet_detour(this: PClientSocket, ppkt: *mut COutPacket) {
    let ret = unsafe { return_address(1) as usize };
    let pkt = unsafe { ppkt.as_ref() }.unwrap();

    let op = pkt.get_opcode();
    log::info!("Send packet: {op}, {ret:X}");
    SEND_PACKET_CTX.finish(ppkt.addr(), ret, pkt.get_data());

    unsafe { send_packet_trampoline(this, ppkt) }

    /*  Overwrite global ret addr so process doesnt crash
    let g_ret_addr = 0x00c68e08 as *mut u32;
    unsafe { g_ret_addr.write_unaligned(0x4afb60) };*/
}
fn process_packet_detour(this: PClientSocket, ppkt: *mut CInPacket) {
    RECV_PACKET_CTX.clear();
    RECV_PACKET_CTX.store_ptr(ppkt);
    let ret = unsafe { return_address(1) as usize };
    let pkt = unsafe { ppkt.as_ref() }.unwrap();

    let op = pkt.get_opcode();
    log::info!("Process packet: {op}");

    unsafe { ProcessPacketHook.call(this, ppkt) }

    RECV_PACKET_CTX.finish(ppkt.addr(), ret, pkt.get_data());
}

fn_ref_hook!(
    out_packet_encode1,
    FOutPacketEncode1,
    encode1_addr,
    0x415360,
    Encode1Hook,
    unsafe extern "thiscall" fn(*const COutPacket, u8)
);

fn encode1_detour(this: *const COutPacket, v: u8) {
    let ret = unsafe { return_address(1) as usize };
    let off = unsafe { this.as_ref().unwrap() }.offset;
    SEND_PACKET_CTX.add_elem(this.addr(), PacketStructElem::i8(off, ret));

    unsafe { Encode1Hook.call(this, v) }
}

fn_ref_hook!(
    out_packet_encode2,
    FOutPacketEncode2,
    encode2_addr,
    0x42ca10,
    Encode2Hook,
    unsafe extern "thiscall" fn(*const COutPacket, u16)
);

fn encode2_detour(this: *const COutPacket, v: u16) {
    let ret = unsafe { return_address(1) as usize };
    let off = unsafe { this.as_ref().unwrap() }.offset;
    SEND_PACKET_CTX.add_elem(this.addr(), PacketStructElem::i16(off, ret));

    unsafe { Encode2Hook.call(this, v) }
}

fn_ref_hook!(
    out_packet_encode4,
    FOutPacketEncode4,
    encode4_addr,
    0x4153b0,
    Encode4Hook,
    unsafe extern "thiscall" fn(*const COutPacket, u32)
);

fn encode4_detour(this: *const COutPacket, v: u32) {
    let ret = unsafe { return_address(1) as usize };
    let off = unsafe { this.as_ref().unwrap() }.offset;
    SEND_PACKET_CTX.add_elem(this.addr(), PacketStructElem::i32(off, ret));

    unsafe { Encode4Hook.call(this, v) }
}

fn_ref_hook!(
    out_packet_encode_str,
    FOutPacketEncodeStr,
    encode_str_addr,
    0x4841f0,
    EncodeStrHook,
    unsafe extern "thiscall" fn(*const COutPacket, ZXString8)
);

fn encode_str_detour(this: *const COutPacket, v: ZXString8) {
    let ret = unsafe { return_address(1) as usize };
    let off = unsafe { this.as_ref().unwrap() }.offset;
    SEND_PACKET_CTX.add_elem(this.addr(), PacketStructElem::str(off, ret, v.len() as u32));

    unsafe { EncodeStrHook.call(this, v) }
}

fn_ref_hook!(
    out_packet_encode_buf,
    FOutPacketEncodeBuf,
    encode_buf_addr,
    0x00482200,
    EncodeBufHook,
    unsafe extern "thiscall" fn(*const COutPacket, *const u8, u32)
);

fn encode_buf_detour(this: *const COutPacket, v: *const u8, n: u32) {
    let ret = unsafe { return_address(1) as usize };
    let off = unsafe { this.as_ref().unwrap() }.offset;
    SEND_PACKET_CTX.add_elem(this.addr(), PacketStructElem::buf(off, ret, n));

    unsafe { EncodeBufHook.call(this, v, n) }
}

// Begin decode

fn_ref_hook!(
    in_packet_decode1,
    FOutPacketDecode1,
    decode1_addr,
    0x4097d0,
    Decode1Hook,
    unsafe extern "thiscall" fn(*const CInPacket) -> u8
);

fn decode1_detour(this: *const CInPacket) -> u8 {
    let ret = unsafe { return_address(1) as usize };
    let off = unsafe { this.as_ref().unwrap() }.offset;
    RECV_PACKET_CTX.add_elem(this.addr(), PacketStructElem::i8(off, ret));

    unsafe { Decode1Hook.call(this) }
}

fn_ref_hook!(
    in_packet_decode2,
    FOutPacketDecode2,
    decode2_addr,
    0x42a2a0,
    Decode2Hook,
    unsafe extern "thiscall" fn(*const CInPacket) -> u16
);

fn decode2_detour(this: *const CInPacket) -> u16 {
    let ret = unsafe { return_address(1) as usize };
    let off = dbg!(unsafe { this.as_ref().unwrap() }).offset;
    RECV_PACKET_CTX.add_elem(this.addr(), PacketStructElem::i16(off, ret));

    unsafe { Decode2Hook.call(this) }
}

fn_ref_hook!(
    in_packet_decode4,
    FOutPacketDecode4,
    decode4_addr,
    0x409870,
    Decode4Hook,
    unsafe extern "thiscall" fn(*const CInPacket) -> u32
);

fn decode4_detour(this: *const CInPacket) -> u32 {
    let ret = unsafe { return_address(1) as usize };
    let off = unsafe { this.as_ref().unwrap() }.offset;
    RECV_PACKET_CTX.add_elem(this.addr(), PacketStructElem::i32(off, ret));

    unsafe { Decode4Hook.call(this) }
}

fn_ref_hook!(
    in_packet_decode_str,
    FOutPacketDecodeStr,
    decode_str_addr,
    0x484140,
    DecodeStrHook,
    unsafe extern "thiscall" fn(*const CInPacket, *mut ZXString8) -> *const ZXString8
);

fn decode_str_detour(this: *const CInPacket, v: *mut ZXString8) -> *const ZXString8 {
    let ret = unsafe { return_address(1) as usize };
    let off = unsafe { this.as_ref().unwrap() }.offset;

    let s = unsafe { DecodeStrHook.call(this, v) };
    let str = unsafe { s.as_ref().unwrap() };
    RECV_PACKET_CTX.add_elem(
        this.addr(),
        PacketStructElem::str(off, ret, str.len() as u32),
    );
    s
}

fn_ref_hook!(
    in_packet_decode_buf,
    FOutPacketDecodeBuf,
    decode_buf_addr,
    0x4336a0,
    DecodeBufHook,
    unsafe extern "thiscall" fn(*const CInPacket, *mut u8, u32)
);

fn decode_buf_detour(this: *const CInPacket, v: *mut u8, n: u32) {
    let ret = unsafe { return_address(1) as usize };
    let off = unsafe { this.as_ref().unwrap() }.offset;
    RECV_PACKET_CTX.add_elem(this.addr(), PacketStructElem::buf(off, ret, n));

    unsafe { DecodeBufHook.call(this, v, n) }
}

// End decode

pub unsafe fn init_hooks() -> anyhow::Result<()> {
    log::info!("Hooking socket");
    SendPacketHook
        .initialize(*client_socket_send_packet, send_packet_detour)?
        .enable()?;

    ProcessPacketHook
        .initialize(*client_socket_process_packet, process_packet_detour)?
        .enable()?;

    Encode1Hook
        .initialize(*out_packet_encode1, encode1_detour)?
        .enable()?;

    Encode2Hook
        .initialize(*out_packet_encode2, encode2_detour)?
        .enable()?;

    Encode4Hook
        .initialize(*out_packet_encode4, encode4_detour)?
        .enable()?;

    EncodeStrHook
        .initialize(*out_packet_encode_str, encode_str_detour)?
        .enable()?;

    EncodeBufHook
        .initialize(*out_packet_encode_buf, encode_buf_detour)?
        .enable()?;

    Decode1Hook
        .initialize(*in_packet_decode1, decode1_detour)?
        .enable()?;

    Decode2Hook
        .initialize(*in_packet_decode2, decode2_detour)?
        .enable()?;

    Decode4Hook
        .initialize(*in_packet_decode4, decode4_detour)?
        .enable()?;

    DecodeStrHook
        .initialize(*in_packet_decode_str, decode_str_detour)?
        .enable()?;

    DecodeBufHook
        .initialize(*in_packet_decode_buf, decode_buf_detour)?
        .enable()?;

    /*
    Checks for return addr there are two options for the encode hook:

    1. Patch all return addr checks
    2. Use the special trampoline call

    // TODO: maybe detours crate should be fixed to spoof ret addy
    // or investigate why naked asm blocks break msvc
    // Top range checks
    nop(0x004afafd as *mut u8, 2);
    nop(0x004afb22 as *mut u8, 2);

    // Inner call range checks
    // xor eax,eax
    // call eax => call 0
    nop(0x004afb8f as *mut u8, 4);
    nop(0x004AFBDD as *mut u8, 4);
    nop(0x004afc15 as *mut u8, 4);
    nop(0x004afc3e as *mut u8, 4);
    nop(0x004afc5f as *mut u8, 4);*/

    log::info!("Socket hooked");
    Ok(())
}
