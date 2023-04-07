pub mod addr95 {
    pub const CXX_EXCEPTION: usize = 0x00a307a1;

    // Socket

    pub const SOCKET_SEND_PACKET: usize = 0x004af9f0;
    pub const SOCKET_PROCESS_PACKET: usize = 0x004b00f0;
    /*
        Any address which calls `CClientSocket::SendPacket` followed by a retn(0xC3) works here
        Usually there's a function which just call SendPacket on the Singleton instance
        of `CClientSocket`
     */
    pub const SOCKET_SINGLETON_SEND_PACKET_RET: usize = 0x00429b8b + 5;
    // The offsets depends how many bytes are overwritten by the hook, usually 5
    pub const SOCKET_SEND_PACKET_TRAMPOLINE_ENTRY: usize = SOCKET_SEND_PACKET + 5;

    pub const PACKET_ENCODE1: usize = 0x00415360;
    pub const PACKET_ENCODE2: usize = 0x0042ca10;
    pub const PACKET_ENCODE4: usize = 0x004153b0;
    pub const PACKET_ENCODE_STR: usize = 0x004841f0;
    pub const PACKET_ENCODE_BUF: usize = 0x00482200;

    pub const PACKET_DECODE1: usize = 0x4097d0;
    pub const PACKET_DECODE2: usize = 0x42a2a0;
    pub const PACKET_DECODE4: usize = 0x409870;
    pub const PACKET_DECODE_STR: usize = 0x484140;
    pub const PACKET_DECODE_BUF: usize = 0x4336a0;


    // String Pool
    pub const STRING_POOL_GET_INSTANCE: usize = 0x007466a0;
    pub const STRING_POOL_GET_STR: usize = 0x00403b30;
    pub const STRING_POOL_GET_STRW: usize = 0x00403b60;


    // Logo skipper
    pub const LOGO_BRANCHES: usize = 0x60e2db;

    // Keys
    pub const AES_BASIC_KEY: usize = 0xc560a0;
    pub const AES_USER_KEY: usize = 0xc560c0;
    pub const IG_SHUFFLE_KEY: usize = 0xc61a70;
    pub const IG_CIPHER_SEED: usize = 0xa1bf35 + 3;
}

pub mod addr83 {
    pub const CXX_EXCEPTION: usize = 0x00a60bb7;

    // Socket

    pub const SOCKET_SEND_PACKET: usize = 0x0049637b;
    pub const SOCKET_PROCESS_PACKET: usize = 0x004965f1;
    /*
        Any address which calls `CClientSocket::SendPacket` followed by a retn(0xC3) works here
        Usually there's a function which just call SendPacket on the Singleton instance
        of `CClientSocket`
     */
    pub const SOCKET_SINGLETON_SEND_PACKET_RET: usize = 0x00a60d5e;
    // The offsets depends how many bytes are overwritten by the hook, usually 5
    pub const SOCKET_SEND_PACKET_TRAMPOLINE_ENTRY: usize = SOCKET_SEND_PACKET + 5;

    pub const PACKET_ENCODE1: usize = 0x00406549;
    pub const PACKET_ENCODE2: usize = 0x00427f74;
    pub const PACKET_ENCODE4: usize = 0x004065a6;
    pub const PACKET_ENCODE_STR: usize = 0x0046f3cf;
    pub const PACKET_ENCODE_BUF: usize = 0x0046c00c;

    pub const PACKET_DECODE1: usize = 0x004065f3;
    pub const PACKET_DECODE2: usize = 0x0042470c;
    pub const PACKET_DECODE4: usize = 0x00406629;
    pub const PACKET_DECODE_STR: usize = 0x0046f30c;
    pub const PACKET_DECODE_BUF: usize = 0x00432257;


    // Not updated ...

    // String Pool
    pub const STRING_POOL_GET_INSTANCE: usize = 0x007466a0;
    pub const STRING_POOL_GET_STR: usize = 0x00403b30;
    pub const STRING_POOL_GET_STRW: usize = 0x00403b60;


    // Logo skipper
    pub const LOGO_BRANCHES: usize = 0x60e2db;

    // Keys
    pub const AES_BASIC_KEY: usize = 0xc560a0;
    pub const AES_USER_KEY: usize = 0xc560c0;
    pub const IG_SHUFFLE_KEY: usize = 0xc61a70;
    pub const IG_CIPHER_SEED: usize = 0xa1bf35 + 3;
    
}

pub use addr95 as addr;

// TODO load some settings from a config file

pub const NAME: &str = "reMember - hook";
pub const VERSION: &str = "1.2";

// Used for dumping, maximum entry id
pub const MAX_STR_POOL_LEN: usize = 6883;

pub const DATA_DIR: &str = "data";

pub const STR_POOL_FILE: &str = "data/str_pool.json";
pub const STR_POOL_UTF16_FILE: &str = "data/str_pool_utf16.json";

pub const PACKET_OUT_FILE: &str = "packets_out";
pub const PACKET_IN_FILE: &str = "packets_in";

pub fn packet_out_file() -> String {
    let pid = std::process::id();
    format!("{DATA_DIR}/{PACKET_OUT_FILE}_{pid}.json")
}

pub fn packet_in_file() -> String {
    let pid = std::process::id();
    format!("{DATA_DIR}/{PACKET_IN_FILE}_{pid}.json")
}

pub const PACKET_TRACING: bool = true;
pub const DUMP_STR_POOL: bool = false;
pub const DUMP_KEYS: bool = false;
pub const SKIP_LOGO: bool = false;