pub type WzKey = [u8; 16];
pub type WzAesKey = [u8; 32];

pub const WZ_KEY_LEN: usize = 16;

pub const GMS_WZ_IV: &WzKey =  include_bytes!("../../../keys/data/gms_iv.bin");
pub const SEA_WZ_IV: &WzKey = include_bytes!("../../../keys/data/sea_iv.bin");
pub const DEFAULT_WZ_IV: &WzKey = include_bytes!("../../../keys/data/default_iv.bin");
pub const WZ_AES_KEY: &WzAesKey = include_bytes!("../../../keys/data/aes.bin");
pub const WZ_OFFSET_MAGIC: u32 = u32::from_le_bytes(*include_bytes!("../../../keys/data/wz_magic.bin"));
