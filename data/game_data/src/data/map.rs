use std::collections::{HashMap, BTreeMap};

use serde::{Serialize, Deserialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Map {
    pub seat: BTreeMap<usize, Seat>,
    pub mini_map: MiniMap,
    pub reactor: Reactor,
    pub portal: Portal,
    pub info: Info,
    pub ladder_rope: BTreeMap<usize, LadderRope>,
    pub life: BTreeMap<usize, Life>,
    #[serde(rename = "ToolTip")]
    pub tool_tip: BTreeMap<usize, ToolTip>,
    pub foothold: BTreeMap<usize, Foothold>,
    pub extra: BTreeMap<usize, Extra>,
    pub back: BTreeMap<usize, Back>,
}

pub type FootholdGroup = BTreeMap<usize, Foothold>;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ToolTip {
    pub y1: i64,
    pub y2: i64,
    pub x1: i64,
    pub x2: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Life {
    pub y: i64,
    pub mob_time: i64,
    pub rx0: i64,
    #[serde(rename = "type")]
    pub type_field: String,
    pub id: String,
    pub rx1: i64,
    pub cy: i64,
    pub x: i64,
    pub f: i64,
    pub hide: i64,
    pub fh: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Portal {
    pub pt: i64,
    pub script: String,
    pub pn: String,
    pub y: i64,
    pub horizontal_impact: i64,
    pub x: i64,
    pub hide_tooltip: i64,
    pub only_once: i64,
    pub tn: String,
    pub tm: i64,
    pub delay: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LadderRope {
    pub y1: i64,
    pub x: i64,
    pub l: i64,
    pub page: i64,
    pub uf: i64,
    pub y2: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Info {
    #[serde(rename = "VRRight")]
    pub vrright: i64,
    pub move_limit: i64,
    pub no_map_cmd: i64,
    pub map_desc: String,
    #[serde(rename = "VRTop")]
    pub vrtop: i64,
    #[serde(rename = "VRLeft")]
    pub vrleft: i64,
    pub forced_return: i64,
    pub mob_rate: f64,
    pub hide_minimap: i64,
    pub town: i64,
    pub version: i64,
    pub cloud: i64,
    pub return_map: i64,
    #[serde(rename = "VRBottom")]
    pub vrbottom: i64,
    pub on_user_enter: String,
    pub map_mark: String,
    pub field_limit: i64,
    pub bgm: String,
    pub swim: i64,
    pub fly: i64,
    pub on_first_user_enter: String,
}
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MiniMap {
    pub width: i64,
    pub height: i64,
    pub center_x: i64,
    pub mag: i64,
    pub center_y: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Seat {
    pub x: i64,
    pub y: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Reactor {
}