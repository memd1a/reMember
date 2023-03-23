use std::{collections::BTreeMap, path::Path};

use anyhow::{anyhow, Context};
use either::Either;
use serde::de::Error;
use serde::{de::DeserializeOwned, Deserialize, Deserializer, Serialize};

#[derive(serde::Serialize, serde::Deserialize, Debug)]
#[serde(transparent)]
struct IntOrString<Int: DeserializeOwned + Serialize> {
    #[serde(with = "either::serde_untagged")]
    inner: Either<String, Int>,
}

fn deserialize_num<'de, D>(deserializer: D) -> Result<u32, D::Error>
where
    D: Deserializer<'de>,
{
    let v = IntOrString::deserialize(deserializer)?;
    Ok(match v.inner {
        Either::Left(s) => s.parse().map_err(D::Error::custom)?,
        Either::Right(n) => n,
    })
}

fn deserialize_inum<'de, D>(deserializer: D) -> Result<i32, D::Error>
where
    D: Deserializer<'de>,
{
    let v = IntOrString::deserialize(deserializer)?;
    Ok(match v.inner {
        Either::Left(s) => s.parse().map_err(D::Error::custom)?,
        Either::Right(n) => n,
    })
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Mob {
    #[serde(default, deserialize_with = "deserialize_num")]
    pub level: u32,
    #[serde(rename = "maxHP", deserialize_with = "deserialize_num")]
    pub max_hp: u32,
    #[serde(rename = "maxMP", default, deserialize_with = "deserialize_num")]
    pub max_mp: u32,
    #[serde(default, deserialize_with = "deserialize_num")]
    pub exp: u32,
    #[serde(default)]
    pub boss: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ItemSummons {
    #[serde(rename = "id", default, deserialize_with = "deserialize_num")]
    pub id: u32,
    #[serde(rename = "prob", default, deserialize_with = "deserialize_num")]
    pub prob: u32
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Item {
    #[serde(rename = "eva", default, deserialize_with = "deserialize_inum")]
    pub eva: i32,
    #[serde(rename = "acc", default, deserialize_with = "deserialize_inum")]
    pub acc: i32,
    #[serde(rename = "hp", default, deserialize_with = "deserialize_inum")]
    pub hp: i32,
    #[serde(rename = "mp", default, deserialize_with = "deserialize_inum")]
    pub mp: i32,
    #[serde(rename = "pad", default, deserialize_with = "deserialize_inum")]
    pub pad: i32,
    #[serde(rename = "mad", default, deserialize_with = "deserialize_inum")]
    pub mad: i32,
    #[serde(rename = "hpR", default, deserialize_with = "deserialize_inum")]
    pub hp_r: i32,
    #[serde(rename = "mpR", default, deserialize_with = "deserialize_inum")]
    pub mp_r: i32,
    #[serde(rename = "pdd", default, deserialize_with = "deserialize_inum")]
    pub pdd: i32,
    #[serde(rename = "mdd", default, deserialize_with = "deserialize_inum")]
    pub mdd: i32,
    #[serde(rename = "speed", default, deserialize_with = "deserialize_inum")]
    pub speed: i32,
    #[serde(rename = "jump", default, deserialize_with = "deserialize_inum")]
    pub jump: i32,

    #[serde(rename = "enchantCategory", default, deserialize_with = "deserialize_num")]
    pub enchant_category: u32,
    #[serde(rename = "slotMax", default, deserialize_with = "deserialize_num")]
    pub slot_max: u32,
    #[serde(rename = "morph", default, deserialize_with = "deserialize_num")]
    pub morph: u32,
    #[serde(rename = "success", default, deserialize_with = "deserialize_num")]
    pub success: u32,
    #[serde(rename = "moveTo", default, deserialize_with = "deserialize_num")]
    pub move_to: u32,
    #[serde(rename = "price", default, deserialize_with = "deserialize_num")]
    pub price: u32,
    #[serde(rename = "unitPrice")]
    pub unit_price: Option<f32>,

    #[serde(rename = "incEVA", default, deserialize_with = "deserialize_num")]
    pub inc_eva: u32,
    #[serde(rename = "incACC", default, deserialize_with = "deserialize_num")]
    pub inc_acc: u32,
    #[serde(rename = "incSTR", default, deserialize_with = "deserialize_num")]
    pub inc_str: u32,
    #[serde(rename = "incDEX", default, deserialize_with = "deserialize_num")]
    pub inc_dex: u32,
    #[serde(rename = "incLUK", default, deserialize_with = "deserialize_num")]
    pub inc_luk: u32,
    #[serde(rename = "incINT", default, deserialize_with = "deserialize_num")]
    pub inc_int: u32,
    #[serde(rename = "incJump", default, deserialize_with = "deserialize_num")]
    pub inc_jump: u32,
    #[serde(rename = "incSpeed", default, deserialize_with = "deserialize_num")]
    pub inc_speed: u32,
    #[serde(rename = "incPDD", default, deserialize_with = "deserialize_num")]
    pub inc_pdd: u32,
    #[serde(rename = "incMDD", default, deserialize_with = "deserialize_num")]
    pub inc_mdd: u32,
    #[serde(rename = "incMHP", default, deserialize_with = "deserialize_num")]
    pub inc_max_hp: u32,
    #[serde(rename = "incMMP", default, deserialize_with = "deserialize_num")]
    pub inc_max_mp: u32,
    #[serde(rename = "incPAD", default, deserialize_with = "deserialize_num")]
    pub inc_pad: u32,
    #[serde(rename = "incMAD", default, deserialize_with = "deserialize_num")]
    pub inc_mad: u32,
    #[serde(rename = "incCraft", default, deserialize_with = "deserialize_num")]
    pub inc_craft: u32,


    #[serde(rename = "cursed", default, deserialize_with = "deserialize_num")]
    pub cursed: u32,
    #[serde(rename = "time", default, deserialize_with = "deserialize_num")]
    pub time: u32,


    #[serde(rename = "summons", default)]
    pub summons: Vec<ItemSummons>,
    #[serde(rename = "notSale", default)]
    pub not_sale: bool,
    #[serde(rename = "accountSharable", default)]
    pub account_sharable: bool,
    #[serde(rename = "cash", default)]
    pub cash: bool,
    #[serde(rename = "expireOnLogout", default)]
    pub expire_on_logout: bool,
    #[serde(rename = "only", default)]
    pub only: bool,
    #[serde(rename = "pquest", default)]
    pub pquest: bool,
    #[serde(rename = "timeLimited", default)]
    pub time_limited: bool,
    #[serde(rename = "tradeBlock", default)]
    pub trade_block: bool,
    #[serde(rename = "bigSize", default)]
    pub big_size: bool,
}



pub fn load_all<T: DeserializeOwned>(
    base_path: impl AsRef<Path>,
) -> anyhow::Result<BTreeMap<u32, T>> {
    let mut res = BTreeMap::new();
    for file in base_path.as_ref().read_dir()? {
        let file = file?.path();
        let id: u32 = file
            .file_stem()
            .ok_or_else(|| anyhow!("Not a file"))?
            .to_string_lossy()
            .parse()?;
        let v = serde_json::from_reader(std::fs::File::open(file)?).context(id)?;
        res.insert(id, v);
    }

    Ok(res)
}
