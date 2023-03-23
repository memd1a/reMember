use crate::ha_xml::{HaXmlValue, Vec2};
use std::collections::BTreeMap;
use serde::{Serialize, Deserialize};

pub type Seat = Vec2;
#[derive(Debug,Serialize,Deserialize)]
pub struct Tile {
    pub no: i64,
    pub u: String,
    pub y: i64,
    pub z_m: i64,
    pub x: i64,
}
impl TryFrom<&HaXmlValue> for Tile {
    type Error = anyhow::Error;
    fn try_from(value: &HaXmlValue) -> Result<Self, Self::Error> {
        let dir = value.as_dir()?;
        Ok(Self {
            no: dir.get_key_mapped("no")?,
            u: dir.get_key_mapped("u")?,
            y: dir.get_key_mapped("y")?,
            z_m: dir.get_key_mapped("zM")?,
            x: dir.get_key_mapped("x")?,
        })
    }
}
#[derive(Debug,Serialize,Deserialize)]
pub struct Portal {
    pub pn: String,
    pub tm: i64,
    pub x: i64,
    pub horizontal_impact: Option<i64>,
    pub only_once: Option<i64>,
    pub hide_tooltip: Option<i64>,
    pub tn: String,
    pub y: i64,
    pub script: Option<String>,
    pub delay: Option<i64>,
    pub pt: i64,
}
impl TryFrom<&HaXmlValue> for Portal {
    type Error = anyhow::Error;
    fn try_from(value: &HaXmlValue) -> Result<Self, Self::Error> {
        let dir = value.as_dir()?;
        Ok(Self {
            pn: dir.get_key_mapped("pn")?,
            tm: dir.get_key_mapped("tm")?,
            x: dir.get_key_mapped("x")?,
            horizontal_impact: dir.get_opt_key_mapped("horizontalImpact")?,
            only_once: dir.get_opt_key_mapped("onlyOnce")?,
            hide_tooltip: dir.get_opt_key_mapped("hideTooltip")?,
            tn: dir.get_key_mapped("tn")?,
            y: dir.get_key_mapped("y")?,
            script: dir.get_opt_key_mapped("script")?,
            delay: dir.get_opt_key_mapped("delay")?,
            pt: dir.get_key_mapped("pt")?,
        })
    }
}
#[derive(Debug,Serialize,Deserialize)]
pub struct Foothold {
    pub y_1: i64,
    pub y_2: i64,
    pub x_1: i64,
    pub next: i64,
    pub piece: Option<i64>,
    pub forbid_fall_down: Option<i64>,
    pub x_2: i64,
    pub prev: i64,
}
impl TryFrom<&HaXmlValue> for Foothold {
    type Error = anyhow::Error;
    fn try_from(value: &HaXmlValue) -> Result<Self, Self::Error> {
        let dir = value.as_dir()?;
        Ok(Self {
            y_1: dir.get_key_mapped("y1")?,
            y_2: dir.get_key_mapped("y2")?,
            x_1: dir.get_key_mapped("x1")?,
            next: dir.get_key_mapped("next")?,
            piece: dir.get_opt_key_mapped("piece")?,
            forbid_fall_down: dir.get_opt_key_mapped("forbidFallDown")?,
            x_2: dir.get_key_mapped("x2")?,
            prev: dir.get_key_mapped("prev")?,
        })
    }
}
#[derive(Debug,Serialize,Deserialize)]
pub struct Reactor {
    pub id: String,
    pub f: i64,
    pub reactor_time: i64,
    pub x: i64,
    pub y: i64,
    pub name: String,
}
impl TryFrom<&HaXmlValue> for Reactor {
    type Error = anyhow::Error;
    fn try_from(value: &HaXmlValue) -> Result<Self, Self::Error> {
        let dir = value.as_dir()?;
        Ok(Self {
            id: dir.get_key_mapped("id")?,
            f: dir.get_key_mapped("f")?,
            reactor_time: dir.get_key_mapped("reactorTime")?,
            x: dir.get_key_mapped("x")?,
            y: dir.get_key_mapped("y")?,
            name: dir.get_key_mapped("name")?,
        })
    }
}
#[derive(Debug,Serialize,Deserialize)]
pub struct LadderRope {
    pub l: i64,
    pub x: i64,
    pub uf: i64,
    pub y_2: i64,
    pub page: i64,
    pub y_1: i64,
}
impl TryFrom<&HaXmlValue> for LadderRope {
    type Error = anyhow::Error;
    fn try_from(value: &HaXmlValue) -> Result<Self, Self::Error> {
        let dir = value.as_dir()?;
        Ok(Self {
            l: dir.get_key_mapped("l")?,
            x: dir.get_key_mapped("x")?,
            uf: dir.get_key_mapped("uf")?,
            y_2: dir.get_key_mapped("y2")?,
            page: dir.get_key_mapped("page")?,
            y_1: dir.get_key_mapped("y1")?,
        })
    }
}
#[derive(Debug,Serialize,Deserialize)]
pub struct Obj {
    pub x: i64,
    pub z_m: i64,
    pub y: i64,
    pub l_2: String,
    pub f: i64,
    pub o_s: String,
    pub z: i64,
    pub l_0: String,
    pub l_1: String,
    pub r: Option<i64>,
}
impl TryFrom<&HaXmlValue> for Obj {
    type Error = anyhow::Error;
    fn try_from(value: &HaXmlValue) -> Result<Self, Self::Error> {
        let dir = value.as_dir()?;
        Ok(Self {
            x: dir.get_key_mapped("x")?,
            z_m: dir.get_key_mapped("zM")?,
            y: dir.get_key_mapped("y")?,
            l_2: dir.get_key_mapped("l2")?,
            f: dir.get_key_mapped("f")?,
            o_s: dir.get_key_mapped("oS")?,
            z: dir.get_key_mapped("z")?,
            l_0: dir.get_key_mapped("l0")?,
            l_1: dir.get_key_mapped("l1")?,
            r: dir.get_opt_key_mapped("r")?,
        })
    }
}
#[derive(Debug,Serialize,Deserialize)]
pub struct MiniMap {
    pub center_x: i64,
    pub mag: i64,
    pub center_y: i64,
    pub height: i64,
    pub width: i64,
}
impl TryFrom<&HaXmlValue> for MiniMap {
    type Error = anyhow::Error;
    fn try_from(value: &HaXmlValue) -> Result<Self, Self::Error> {
        let dir = value.as_dir()?;
        Ok(Self {
            center_x: dir.get_key_mapped("centerX")?,
            mag: dir.get_key_mapped("mag")?,
            center_y: dir.get_key_mapped("centerY")?,
            height: dir.get_key_mapped("height")?,
            width: dir.get_key_mapped("width")?,
        })
    }
}
#[derive(Debug,Serialize,Deserialize)]
pub struct Extra {
    pub obj: BTreeMap<i64, Obj>,
    pub tile: BTreeMap<i64, Tile>,
    pub info: BTreeMap<i64, Info>,
}
impl TryFrom<&HaXmlValue> for Extra {
    type Error = anyhow::Error;
    fn try_from(value: &HaXmlValue) -> Result<Self, Self::Error> {
        let dir = value.as_dir()?;
        Ok(Self {
            obj: dir.get_key_mapped("obj")?,
            tile: dir.get_key_mapped("tile")?,
            info: dir.get_key_mapped("info")?,
        })
    }
}
#[derive(Debug,Serialize,Deserialize)]
pub struct Map {
    pub portal: BTreeMap<i64, Portal>,
    pub mini_map: Option<MiniMap>,
    pub foothold: BTreeMap<i64, BTreeMap<i64, BTreeMap<i64,Foothold>>>,
    //pub extra: BTreeMap<i64, Extra>,
    pub back: BTreeMap<i64, Back>,
    pub info: Info,
    pub ladder_rope: BTreeMap<i64, LadderRope>,
    pub life: BTreeMap<i64, Life>,
    pub reactor: BTreeMap<i64, Reactor>,
    pub seat: Option<BTreeMap<i64, Seat>>,
}
impl TryFrom<&HaXmlValue> for Map {
    type Error = anyhow::Error;
    fn try_from(value: &HaXmlValue) -> Result<Self, Self::Error> {
        let dir = value.as_dir()?;
        Ok(Self {
            portal: dir.get_key_mapped("portal")?,
            mini_map: dir.get_opt_key_mapped("miniMap")?,
            foothold: dir.get_key_mapped("foothold")?,
            //extra: dir.get_key_mapped("extra")?,
            back: dir.get_key_mapped("back")?,
            info: dir.get_key_mapped("info")?,
            ladder_rope: dir.get_key_mapped("ladderRope")?,
            life: dir.get_key_mapped("life")?,
            reactor: dir.get_key_mapped("reactor")?,
            seat: dir.get_opt_key_mapped("seat")?,
        })
    }
}
#[derive(Debug,Serialize,Deserialize)]
pub struct Back {
    pub rx: i64,
    pub _type: i64,
    pub f: i64,
    pub x: i64,
    pub ry: i64,
    pub a: i64,
    pub y: i64,
    pub cy: i64,
    pub cx: i64,
    pub b_s: String,
    pub front: i64,
    pub ani: i64,
    pub no: i64,
}
impl TryFrom<&HaXmlValue> for Back {
    type Error = anyhow::Error;
    fn try_from(value: &HaXmlValue) -> Result<Self, Self::Error> {
        let dir = value.as_dir()?;
        Ok(Self {
            rx: dir.get_key_mapped("rx")?,
            _type: dir.get_key_mapped("type")?,
            f: dir.get_key_mapped("f")?,
            x: dir.get_key_mapped("x")?,
            ry: dir.get_key_mapped("ry")?,
            a: dir.get_key_mapped("a")?,
            y: dir.get_key_mapped("y")?,
            cy: dir.get_key_mapped("cy")?,
            cx: dir.get_key_mapped("cx")?,
            b_s: dir.get_key_mapped("bS")?,
            front: dir.get_key_mapped("front")?,
            ani: dir.get_key_mapped("ani")?,
            no: dir.get_key_mapped("no")?,
        })
    }
}
#[derive(Debug,Serialize,Deserialize)]
pub struct Info {
    pub vr_left: Option<i64>,
    pub move_limit: Option<i64>,
    pub vr_right: Option<i64>,
    pub return_map: Option<i64>,
    pub t_s: Option<String>,
    pub lb_bottom: Option<i64>,
    pub no_map_cmd: Option<i64>,
    pub lb_top: Option<i64>,
    pub on_first_user_enter: Option<String>,
    pub vr_bottom: Option<i64>,
    pub cloud: Option<i64>,
    pub forbid_fall_down: Option<i64>,
    pub mob_rate: Option<f32>,
    pub fly: Option<i64>,
    pub forced_return: Option<i64>,
    pub version: Option<i64>,
    pub bgm: Option<String>,
    pub map_desc: Option<String>,
    pub hide_minimap: Option<i64>,
    pub on_user_enter: Option<String>,
    pub map_mark: Option<String>,
    pub vr_top: Option<i64>,
    pub swim: Option<i64>,
    pub field_limit: Option<i64>,
    pub town: Option<i64>,
    pub t_s_mag: Option<i64>,
    pub lb_side: Option<i64>,
}
impl TryFrom<&HaXmlValue> for Info {
    type Error = anyhow::Error;
    fn try_from(value: &HaXmlValue) -> Result<Self, Self::Error> {
        let dir = value.as_dir()?;
        Ok(Self {
            vr_left: dir.get_opt_key_mapped("VRLeft")?,
            move_limit: dir.get_opt_key_mapped("moveLimit")?,
            vr_right: dir.get_opt_key_mapped("VRRight")?,
            return_map: dir.get_opt_key_mapped("returnMap")?,
            t_s: dir.get_opt_key_mapped("tS")?,
            lb_bottom: dir.get_opt_key_mapped("LBBottom")?,
            no_map_cmd: dir.get_opt_key_mapped("noMapCmd")?,
            lb_top: dir.get_opt_key_mapped("LBTop")?,
            on_first_user_enter: dir.get_opt_key_mapped("onFirstUserEnter")?,
            vr_bottom: dir.get_opt_key_mapped("VRBottom")?,
            cloud: dir.get_opt_key_mapped("cloud")?,
            forbid_fall_down: dir.get_opt_key_mapped("forbidFallDown")?,
            mob_rate: dir.get_opt_key_mapped("mobRate")?,
            fly: dir.get_opt_key_mapped("fly")?,
            forced_return: dir.get_opt_key_mapped("forcedReturn")?,
            version: dir.get_opt_key_mapped("version")?,
            bgm: dir.get_opt_key_mapped("bgm")?,
            map_desc: dir.get_opt_key_mapped("mapDesc")?,
            hide_minimap: dir.get_opt_key_mapped("hideMinimap")?,
            on_user_enter: dir.get_opt_key_mapped("onUserEnter")?,
            map_mark: dir.get_opt_key_mapped("mapMark")?,
            vr_top: dir.get_opt_key_mapped("VRTop")?,
            swim: dir.get_opt_key_mapped("swim")?,
            field_limit: dir.get_opt_key_mapped("fieldLimit")?,
            town: dir.get_opt_key_mapped("town")?,
            t_s_mag: dir.get_opt_key_mapped("tSMag")?,
            lb_side: dir.get_opt_key_mapped("LBSide")?,
        })
    }
}
#[derive(Debug,Serialize,Deserialize)]
pub struct Life {
    pub id: String,
    pub hide: Option<i64>,
    pub fh: i64,
    pub rx_0: i64,
    pub _type: String,
    pub x: i64,
    pub cy: i64,
    pub mob_time: Option<i64>,
    pub rx_1: i64,
    pub f: Option<i64>,
    pub y: i64,
}
impl TryFrom<&HaXmlValue> for Life {
    type Error = anyhow::Error;
    fn try_from(value: &HaXmlValue) -> Result<Self, Self::Error> {
        let dir = value.as_dir()?;
        Ok(Self {
            id: dir.get_key_mapped("id")?,
            hide: dir.get_opt_key_mapped("hide")?,
            fh: dir.get_key_mapped("fh")?,
            rx_0: dir.get_key_mapped("rx0")?,
            _type: dir.get_key_mapped("type")?,
            x: dir.get_key_mapped("x")?,
            cy: dir.get_key_mapped("cy")?,
            mob_time: dir.get_opt_key_mapped("mobTime")?,
            rx_1: dir.get_key_mapped("rx1")?,
            f: dir.get_opt_key_mapped("f")?,
            y: dir.get_key_mapped("y")?,
        })
    }
}
