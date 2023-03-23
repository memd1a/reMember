use serde::{Deserialize, Serialize};

use crate::ha_xml::{HaXmlValue, Vec2};
use std::collections::BTreeMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct MultiRect {
    pub lt: Vec2,
    pub rb: Vec2,
}
impl TryFrom<&HaXmlValue> for MultiRect {
    type Error = anyhow::Error;
    fn try_from(value: &HaXmlValue) -> Result<Self, Self::Error> {
        let dir = value.as_dir()?;
        Ok(Self {
            lt: dir.get_key_mapped("lt")?,
            rb: dir.get_key_mapped("rb")?,
        })
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Skill6 {
    pub delay: i64,
    pub head: Vec2,
    pub origin: Vec2,
}
impl TryFrom<&HaXmlValue> for Skill6 {
    type Error = anyhow::Error;
    fn try_from(value: &HaXmlValue) -> Result<Self, Self::Error> {
        let dir = value.as_dir()?;
        Ok(Self {
            delay: dir.get_key_mapped("delay")?,
            head: dir.get_key_mapped("head")?,
            origin: dir.get_key_mapped("origin")?,
        })
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Die1 {
    pub speak: Option<Speak>,
    pub _0: Option<i64>,
    pub lt: Option<Vec2>,
    pub z_1: Option<i64>,
    pub z_0: Option<i64>,
    pub a_1: Option<i64>,
    pub a_0: Option<i64>,
    pub rb: Option<Vec2>,
    pub rect: Option<BTreeMap<i64, Rect>>,
    pub dealy: Option<i64>,
    pub head: Option<Vec2>,
    pub delay: Option<i64>,
    pub origin: Vec2,
    pub z: Option<i64>,
}
impl TryFrom<&HaXmlValue> for Die1 {
    type Error = anyhow::Error;
    fn try_from(value: &HaXmlValue) -> Result<Self, Self::Error> {
        let dir = value.as_dir()?;
        Ok(Self {
            speak: dir.get_opt_key_mapped("speak")?,
            _0: dir.get_opt_key_mapped("0")?,
            lt: dir.get_opt_key_mapped("lt")?,
            z_1: dir.get_opt_key_mapped("z1")?,
            z_0: dir.get_opt_key_mapped("z0")?,
            a_1: dir.get_opt_key_mapped("a1")?,
            a_0: dir.get_opt_key_mapped("a0")?,
            rb: dir.get_opt_key_mapped("rb")?,
            rect: dir.get_opt_key_mapped("rect")?,
            dealy: dir.get_opt_key_mapped("dealy")?,
            head: dir.get_opt_key_mapped("head")?,
            delay: dir.get_opt_key_mapped("delay")?,
            origin: dir.get_key_mapped("origin")?,
            z: dir.get_opt_key_mapped("z")?,
        })
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Skill8 {
    pub delay: i64,
    pub origin: Vec2,
    pub head: Vec2,
}
impl TryFrom<&HaXmlValue> for Skill8 {
    type Error = anyhow::Error;
    fn try_from(value: &HaXmlValue) -> Result<Self, Self::Error> {
        let dir = value.as_dir()?;
        Ok(Self {
            delay: dir.get_key_mapped("delay")?,
            origin: dir.get_key_mapped("origin")?,
            head: dir.get_key_mapped("head")?,
        })
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Skill4 {
    pub rect: Option<BTreeMap<i64, Rect>>,
    pub rb: Option<Vec2>,
    pub delay: Option<i64>,
    pub lt: Option<Vec2>,
    pub head: Vec2,
    pub z: Option<i64>,
    pub origin: Vec2,
}
impl TryFrom<&HaXmlValue> for Skill4 {
    type Error = anyhow::Error;
    fn try_from(value: &HaXmlValue) -> Result<Self, Self::Error> {
        let dir = value.as_dir()?;
        Ok(Self {
            rect: dir.get_opt_key_mapped("rect")?,
            rb: dir.get_opt_key_mapped("rb")?,
            delay: dir.get_opt_key_mapped("delay")?,
            lt: dir.get_opt_key_mapped("lt")?,
            head: dir.get_key_mapped("head")?,
            z: dir.get_opt_key_mapped("z")?,
            origin: dir.get_key_mapped("origin")?,
        })
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Hit {
    pub attach: Option<i64>,
    pub attachfacing: Option<i64>,
    pub head: Option<Vec2>,
    pub delya: Option<i64>,
    pub a_0: Option<i64>,
    pub lt: Option<Vec2>,
    pub rb: Option<Vec2>,
    pub a_1: Option<i64>,
    pub origin: Vec2,
    pub delay: Option<i64>,
    pub hit_after: Option<i64>,
    pub z: Option<i64>,
    pub pos: Option<i64>,
}
impl TryFrom<&HaXmlValue> for Hit {
    type Error = anyhow::Error;
    fn try_from(value: &HaXmlValue) -> Result<Self, Self::Error> {
        let dir = value.as_dir()?;
        Ok(Self {
            attach: dir.get_opt_key_mapped("attach")?,
            attachfacing: dir.get_opt_key_mapped("attachfacing")?,
            head: dir.get_opt_key_mapped("head")?,
            delya: dir.get_opt_key_mapped("delya")?,
            a_0: dir.get_opt_key_mapped("a0")?,
            lt: dir.get_opt_key_mapped("lt")?,
            rb: dir.get_opt_key_mapped("rb")?,
            a_1: dir.get_opt_key_mapped("a1")?,
            origin: dir.get_key_mapped("origin")?,
            delay: dir.get_opt_key_mapped("delay")?,
            hit_after: dir.get_opt_key_mapped("hitAfter")?,
            z: dir.get_opt_key_mapped("z")?,
            pos: dir.get_opt_key_mapped("pos")?,
        })
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Hit1 {
    pub lt: Option<Vec2>,
    pub origin: Vec2,
    pub z: Option<i64>,
    pub a_0: Option<i64>,
    pub rb: Option<Vec2>,
    pub speak: Option<Speak>,
    pub delay: Option<i64>,
    pub rect: Option<BTreeMap<i64, Rect>>,
    pub head: Option<Vec2>,
}
impl TryFrom<&HaXmlValue> for Hit1 {
    type Error = anyhow::Error;
    fn try_from(value: &HaXmlValue) -> Result<Self, Self::Error> {
        let dir = value.as_dir()?;
        Ok(Self {
            lt: dir.get_opt_key_mapped("lt")?,
            origin: dir.get_key_mapped("origin")?,
            z: dir.get_opt_key_mapped("z")?,
            a_0: dir.get_opt_key_mapped("a0")?,
            rb: dir.get_opt_key_mapped("rb")?,
            speak: dir.get_opt_key_mapped("speak")?,
            delay: dir.get_opt_key_mapped("delay")?,
            rect: dir.get_opt_key_mapped("rect")?,
            head: dir.get_opt_key_mapped("head")?,
        })
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Jump {
    pub head: Option<Vec2>,
    pub lt: Option<Vec2>,
    pub delay: Option<i64>,
    pub z: Option<i64>,
    pub rb: Option<Vec2>,
    pub origin: Vec2,
    pub rect: Option<BTreeMap<i64, Rect>>,
}
impl TryFrom<&HaXmlValue> for Jump {
    type Error = anyhow::Error;
    fn try_from(value: &HaXmlValue) -> Result<Self, Self::Error> {
        let dir = value.as_dir()?;
        Ok(Self {
            head: dir.get_opt_key_mapped("head")?,
            lt: dir.get_opt_key_mapped("lt")?,
            delay: dir.get_opt_key_mapped("delay")?,
            z: dir.get_opt_key_mapped("z")?,
            rb: dir.get_opt_key_mapped("rb")?,
            origin: dir.get_key_mapped("origin")?,
            rect: dir.get_opt_key_mapped("rect")?,
        })
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Miss {
    pub delay: Option<i64>,
    pub a_0: i64,
    pub origin: Vec2,
    pub a_1: i64,
    pub head: Vec2,
}
impl TryFrom<&HaXmlValue> for Miss {
    type Error = anyhow::Error;
    fn try_from(value: &HaXmlValue) -> Result<Self, Self::Error> {
        let dir = value.as_dir()?;
        Ok(Self {
            delay: dir.get_opt_key_mapped("delay")?,
            a_0: dir.get_key_mapped("a0")?,
            origin: dir.get_key_mapped("origin")?,
            a_1: dir.get_key_mapped("a1")?,
            head: dir.get_key_mapped("head")?,
        })
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Info {
    pub do_not_remove: Option<i64>,
    pub escort: Option<i64>,
    pub rush: Option<i64>,
    pub remove_quest: Option<i64>,
    pub fs: Option<f32>,
    pub disable: Option<i64>,
    pub bodyattack: Option<i64>,
    pub hide_hp: Option<i64>,
    pub bullet_speed: Option<i64>,
    pub com_mp: Option<i64>,
    pub pushed: Option<i64>,
    pub ma_damage: Option<i64>,
    pub default: Option<Default>,
    pub self_destruction: Option<SelfDestruction>,
    pub rare_item_drop_level: Option<i64>,
    pub body_attack: Option<i64>,
    pub pd_rate: Option<i64>,
    pub pd_damage: Option<i64>,
    pub hidename: Option<i64>,
    pub knockback_ex: Option<i64>,
    pub knockback_ex_impact_y: Option<i64>,
    pub anger_attack: Option<i64>,
    pub rand_delay_attack: Option<i64>,
    pub tremble: Option<i64>,
    pub flyspeed: Option<i64>,
    pub first_attack: Option<i64>,
    pub dual_gauge: Option<i64>,
    pub invincible: Option<i64>,
    pub cool_damage_prob: Option<i64>,
    pub hp_tag_color: Option<i64>,
    pub damaged_by_mob: Option<i64>,
    pub public_reward: Option<i64>,
    pub speed: Option<i64>,
    pub do_first: Option<i64>,
    pub jump_attack: Option<i64>,
    pub eva: Option<i64>,
    pub remove_after: Option<i64>,
    pub only_normal_attack: Option<i64>,
    pub mp_burn: Option<i64>,
    pub point: Option<i64>,
    pub no_doom: Option<i64>,
    pub cool_damage: Option<i64>,
    pub skill: Option<BTreeMap<i64, Skill>>,
    pub md_damage: Option<i64>,
    pub fly_speed: Option<i64>,
    pub remove_on_miss: Option<i64>,
    pub pa_damge: Option<i64>,
    pub buff: Option<String>,
    pub heal_on_destroy: Option<HealOnDestroy>,
    pub category: Option<i64>,
    pub elem_attr: Option<String>,
    pub area_warning: Option<AreaWarning>,
    pub get_cp: Option<i64>,
    pub hide_name: Option<i64>,
    pub _0: Option<i64>,
    pub default_hp: Option<String>,
    pub lose_item: Option<BTreeMap<i64, LoseItem>>,
    pub summon_type: Option<i64>,
    pub hit: Option<Hit>,
    pub h_pgauge_hide: Option<i64>,
    pub no_flip: Option<i64>,
    pub weapon: Option<i64>,
    pub party_reward: Option<String>,
    pub desease: Option<i64>,
    pub attachfacing: Option<i64>,
    pub ban: Option<Ban>,
    pub hp_tag_bgcolor: Option<i64>,
    pub default_mp: Option<String>,
    pub magic: Option<i64>,
    pub delay: Option<i64>,
    pub attach: Option<i64>,
    pub phase: Option<i64>,
    pub bullet_number: Option<i64>,
    pub cant_pass_by_teleport: Option<i64>,
    pub attack_after: Option<i64>,
    pub acc: Option<i64>,
    pub chase_speed: Option<i64>,
    pub knockback_ex_impact_x: Option<i64>,
    pub anger_gauge: Option<i64>,
    pub boss: Option<i64>,
    pub max_mp: Option<i64>,
    pub upper_most_layer: Option<i64>,
    pub range: Option<Range>,
    pub fixed_damage: Option<i64>,
    pub explosive_reward: Option<i64>,
    pub special_attack: Option<i64>,
    pub exp: Option<i64>,
    pub link: Option<String>,
    pub effect_after: Option<i64>,
    pub not_attack: Option<i64>,
    pub fix_damage: Option<i64>,
    pub hp_recovery: Option<i64>,
    pub ball: Option<Ball>,
    pub mob_type: Option<String>,
    pub disease: Option<i64>,
    pub ma_damge: Option<i64>,
    pub con_mp: Option<i64>,
    pub undead: Option<i64>,
    pub _type: Option<i64>,
    pub ignore_field_out: Option<i64>,
    pub cannot_evade: Option<i64>,
    pub pa_damage: Option<i64>,
    pub md_rate: Option<i64>,
    pub mp_recovery: Option<i64>,
    pub speak: Option<Speak>,
    pub deadly_attack: Option<i64>,
    pub knockback: Option<i64>,
    pub drop_item_period: Option<i64>,
    pub charge_count: Option<i64>,
    pub mbook_id: Option<i64>,
    pub firstattack: Option<i64>,
    pub max_hp: Option<i64>,
    pub noregen: Option<i64>,
    pub level: Option<i64>,
}
impl TryFrom<&HaXmlValue> for Info {
    type Error = anyhow::Error;
    fn try_from(value: &HaXmlValue) -> Result<Self, Self::Error> {
        let dir = value.as_dir()?;
        Ok(Self {
            do_not_remove: dir.get_opt_key_mapped("doNotRemove")?,
            escort: dir.get_opt_key_mapped("escort")?,
            rush: dir.get_opt_key_mapped("rush")?,
            remove_quest: dir.get_opt_key_mapped("removeQuest")?,
            fs: dir.get_opt_key_mapped("fs")?,
            disable: dir.get_opt_key_mapped("disable")?,
            bodyattack: dir.get_opt_key_mapped("bodyattack")?,
            hide_hp: dir.get_opt_key_mapped("hideHP")?,
            bullet_speed: dir.get_opt_key_mapped("bulletSpeed")?,
            com_mp: dir.get_opt_key_mapped("comMP")?,
            pushed: dir.get_opt_key_mapped("pushed")?,
            ma_damage: dir.get_opt_key_mapped("MADamage")?,
            default: dir.get_opt_key_mapped("default")?,
            self_destruction: dir.get_opt_key_mapped("selfDestruction")?,
            rare_item_drop_level: dir.get_opt_key_mapped("rareItemDropLevel")?,
            body_attack: dir.get_opt_key_mapped("bodyAttack")?,
            pd_rate: dir.get_opt_key_mapped("PDRate")?,
            pd_damage: dir.get_opt_key_mapped("PDDamage")?,
            hidename: dir.get_opt_key_mapped("hidename")?,
            knockback_ex: dir.get_opt_key_mapped("knockbackEx")?,
            knockback_ex_impact_y: dir.get_opt_key_mapped("knockbackExImpactY")?,
            anger_attack: dir.get_opt_key_mapped("AngerAttack")?,
            rand_delay_attack: dir.get_opt_key_mapped("randDelayAttack")?,
            tremble: dir.get_opt_key_mapped("tremble")?,
            flyspeed: dir.get_opt_key_mapped("flyspeed")?,
            first_attack: dir.get_opt_key_mapped("firstAttack")?,
            dual_gauge: dir.get_opt_key_mapped("dualGauge")?,
            invincible: dir.get_opt_key_mapped("invincible")?,
            cool_damage_prob: dir.get_opt_key_mapped("coolDamageProb")?,
            hp_tag_color: dir.get_opt_key_mapped("hpTagColor")?,
            damaged_by_mob: dir.get_opt_key_mapped("damagedByMob")?,
            public_reward: dir.get_opt_key_mapped("publicReward")?,
            speed: dir.get_opt_key_mapped("speed")?,
            do_first: dir.get_opt_key_mapped("doFirst")?,
            jump_attack: dir.get_opt_key_mapped("jumpAttack")?,
            eva: dir.get_opt_key_mapped("eva")?,
            remove_after: dir.get_opt_key_mapped("removeAfter")?,
            only_normal_attack: dir.get_opt_key_mapped("onlyNormalAttack")?,
            mp_burn: dir.get_opt_key_mapped("mpBurn")?,
            point: dir.get_opt_key_mapped("point")?,
            no_doom: dir.get_opt_key_mapped("noDoom")?,
            cool_damage: dir.get_opt_key_mapped("coolDamage")?,
            skill: dir.get_opt_key_mapped("skill")?,
            md_damage: dir.get_opt_key_mapped("MDDamage")?,
            fly_speed: dir.get_opt_key_mapped("FlySpeed")?,
            remove_on_miss: dir.get_opt_key_mapped("removeOnMiss")?,
            pa_damge: dir.get_opt_key_mapped("PADamge")?,
            buff: dir.get_opt_key_mapped("buff")?,
            heal_on_destroy: dir.get_opt_key_mapped("healOnDestroy")?,
            category: dir.get_opt_key_mapped("category")?,
            elem_attr: dir.get_opt_key_mapped("elemAttr")?,
            area_warning: dir.get_opt_key_mapped("areaWarning")?,
            get_cp: dir.get_opt_key_mapped("getCP")?,
            hide_name: dir.get_opt_key_mapped("hideName")?,
            _0: dir.get_opt_key_mapped("0")?,
            default_hp: dir.get_opt_key_mapped("defaultHP")?,
            lose_item: dir.get_opt_key_mapped("loseItem")?,
            summon_type: dir.get_opt_key_mapped("summonType")?,
            hit: dir.get_opt_key_mapped("hit")?,
            h_pgauge_hide: dir.get_opt_key_mapped("HPgaugeHide")?,
            no_flip: dir.get_opt_key_mapped("noFlip")?,
            weapon: dir.get_opt_key_mapped("weapon")?,
            party_reward: dir.get_opt_key_mapped("PartyReward")?,
            desease: dir.get_opt_key_mapped("desease")?,
            attachfacing: dir.get_opt_key_mapped("attachfacing")?,
            ban: dir.get_opt_key_mapped("ban")?,
            hp_tag_bgcolor: dir.get_opt_key_mapped("hpTagBgcolor")?,
            default_mp: dir.get_opt_key_mapped("defaultMP")?,
            magic: dir.get_opt_key_mapped("magic")?,
            delay: dir.get_opt_key_mapped("delay")?,
            attach: dir.get_opt_key_mapped("attach")?,
            phase: dir.get_opt_key_mapped("phase")?,
            bullet_number: dir.get_opt_key_mapped("bulletNumber")?,
            cant_pass_by_teleport: dir.get_opt_key_mapped("cantPassByTeleport")?,
            attack_after: dir.get_opt_key_mapped("attackAfter")?,
            acc: dir.get_opt_key_mapped("acc")?,
            chase_speed: dir.get_opt_key_mapped("chaseSpeed")?,
            knockback_ex_impact_x: dir.get_opt_key_mapped("knockbackExImpactX")?,
            anger_gauge: dir.get_opt_key_mapped("AngerGauge")?,
            boss: dir.get_opt_key_mapped("boss")?,
            max_mp: dir.get_opt_key_mapped("maxMP")?,
            upper_most_layer: dir.get_opt_key_mapped("upperMostLayer")?,
            range: dir.get_opt_key_mapped("range")?,
            fixed_damage: dir.get_opt_key_mapped("fixedDamage")?,
            explosive_reward: dir.get_opt_key_mapped("explosiveReward")?,
            special_attack: dir.get_opt_key_mapped("specialAttack")?,
            exp: dir.get_opt_key_mapped("exp")?,
            link: dir.get_opt_key_mapped("link")?,
            effect_after: dir.get_opt_key_mapped("effectAfter")?,
            not_attack: dir.get_opt_key_mapped("notAttack")?,
            fix_damage: dir.get_opt_key_mapped("fixDamage")?,
            hp_recovery: dir.get_opt_key_mapped("hpRecovery")?,
            ball: dir.get_opt_key_mapped("ball")?,
            mob_type: dir.get_opt_key_mapped("mobType")?,
            disease: dir.get_opt_key_mapped("disease")?,
            ma_damge: dir.get_opt_key_mapped("MADamge")?,
            con_mp: dir.get_opt_key_mapped("conMP")?,
            undead: dir.get_opt_key_mapped("undead")?,
            _type: dir.get_opt_key_mapped("type")?,
            ignore_field_out: dir.get_opt_key_mapped("ignoreFieldOut")?,
            cannot_evade: dir.get_opt_key_mapped("cannotEvade")?,
            pa_damage: dir.get_opt_key_mapped("PADamage")?,
            md_rate: dir.get_opt_key_mapped("MDRate")?,
            mp_recovery: dir.get_opt_key_mapped("mpRecovery")?,
            speak: dir.get_opt_key_mapped("speak")?,
            deadly_attack: dir.get_opt_key_mapped("deadlyAttack")?,
            knockback: dir.get_opt_key_mapped("knockback")?,
            drop_item_period: dir.get_opt_key_mapped("dropItemPeriod")?,
            charge_count: dir.get_opt_key_mapped("ChargeCount")?,
            mbook_id: dir.get_opt_key_mapped("mbookID")?,
            firstattack: dir.get_opt_key_mapped("firstattack")?,
            max_hp: dir.get_opt_key_mapped("maxHP")?,
            noregen: dir.get_opt_key_mapped("noregen")?,
            level: dir.get_opt_key_mapped("level")?,
        })
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Skill2 {
    pub head: Option<Vec2>,
    pub speak: Option<Speak>,
    pub origin: Vec2,
    pub lt: Option<Vec2>,
    pub rect: Option<BTreeMap<i64, Rect>>,
    pub delay: Option<i64>,
    pub rb: Option<Vec2>,
    pub z: Option<i64>,
}
impl TryFrom<&HaXmlValue> for Skill2 {
    type Error = anyhow::Error;
    fn try_from(value: &HaXmlValue) -> Result<Self, Self::Error> {
        let dir = value.as_dir()?;
        Ok(Self {
            head: dir.get_opt_key_mapped("head")?,
            speak: dir.get_opt_key_mapped("speak")?,
            origin: dir.get_key_mapped("origin")?,
            lt: dir.get_opt_key_mapped("lt")?,
            rect: dir.get_opt_key_mapped("rect")?,
            delay: dir.get_opt_key_mapped("delay")?,
            rb: dir.get_opt_key_mapped("rb")?,
            z: dir.get_opt_key_mapped("z")?,
        })
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Chase {
    pub lt: Vec2,
    pub delay: Option<i64>,
    pub origin: Vec2,
    pub rb: Vec2,
    pub head: Vec2,
    pub z: Option<i64>,
}
impl TryFrom<&HaXmlValue> for Chase {
    type Error = anyhow::Error;
    fn try_from(value: &HaXmlValue) -> Result<Self, Self::Error> {
        let dir = value.as_dir()?;
        Ok(Self {
            lt: dir.get_key_mapped("lt")?,
            delay: dir.get_opt_key_mapped("delay")?,
            origin: dir.get_key_mapped("origin")?,
            rb: dir.get_key_mapped("rb")?,
            head: dir.get_key_mapped("head")?,
            z: dir.get_opt_key_mapped("z")?,
        })
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Skill16 {
    pub origin: Vec2,
    pub speak: Option<Speak>,
    pub delay: Option<i64>,
    pub z: i64,
}
impl TryFrom<&HaXmlValue> for Skill16 {
    type Error = anyhow::Error;
    fn try_from(value: &HaXmlValue) -> Result<Self, Self::Error> {
        let dir = value.as_dir()?;
        Ok(Self {
            origin: dir.get_key_mapped("origin")?,
            speak: dir.get_opt_key_mapped("speak")?,
            delay: dir.get_opt_key_mapped("delay")?,
            z: dir.get_key_mapped("z")?,
        })
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Skill {
    pub action: i64,
    pub skill_after: Option<i64>,
    pub info: Option<String>,
    pub effect_after: Option<i64>,
    pub skill: i64,
    pub level: i64,
}
impl TryFrom<&HaXmlValue> for Skill {
    type Error = anyhow::Error;
    fn try_from(value: &HaXmlValue) -> Result<Self, Self::Error> {
        let dir = value.as_dir()?;
        Ok(Self {
            action: dir.get_key_mapped("action")?,
            skill_after: dir.get_opt_key_mapped("skillAfter")?,
            info: dir.get_opt_key_mapped("info")?,
            effect_after: dir.get_opt_key_mapped("effectAfter")?,
            skill: dir.get_key_mapped("skill")?,
            level: dir.get_key_mapped("level")?,
        })
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct AreaWarning {
    pub a_0: Option<i64>,
    pub attach: Option<i64>,
    pub a_1: Option<i64>,
    pub origin: Vec2,
    pub z: Option<i64>,
    pub attachfacing: Option<i64>,
    pub delay: Option<i64>,
}
impl TryFrom<&HaXmlValue> for AreaWarning {
    type Error = anyhow::Error;
    fn try_from(value: &HaXmlValue) -> Result<Self, Self::Error> {
        let dir = value.as_dir()?;
        Ok(Self {
            a_0: dir.get_opt_key_mapped("a0")?,
            attach: dir.get_opt_key_mapped("attach")?,
            a_1: dir.get_opt_key_mapped("a1")?,
            origin: dir.get_key_mapped("origin")?,
            z: dir.get_opt_key_mapped("z")?,
            attachfacing: dir.get_opt_key_mapped("attachfacing")?,
            delay: dir.get_opt_key_mapped("delay")?,
        })
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Mob {
    pub regen: Option<Regen>,
    pub anger_gauge_effect: Option<AngerGaugeEffect>,
    pub die_f: Option<DieF>,
    pub _move: Option<Move>,
    pub die: Option<Die>,
    pub stand: Option<Stand>,
    pub die_2: Option<Die2>,
    pub chase: Option<Chase>,
    pub attack_6: Option<Attack6>,
    pub hit_1: Option<Hit1>,
    pub attack_4: Option<Attack4>,
    pub rope: Option<Rope>,
    pub fly: Option<Fly>,
    pub attack_3: Option<Attack3>,
    pub jump: Option<Jump>,
    pub hit: Option<Hit>,
    pub anger_gauge_animation: Option<BTreeMap<i64, AngerGaugeAnimation>>,
    pub info: Info,
    pub eye: Option<Eye>,
    pub attack_2: Option<Attack2>,
    pub attack_5: Option<Attack5>,
    pub die_1: Option<Die1>,
    pub stop: Option<Stop>,
    pub say: Option<Say>,
    pub miss: Option<Miss>,
    pub attack_1: Option<Attack1>,
    pub ladder: Option<Ladder>,
}
impl TryFrom<&HaXmlValue> for Mob {
    type Error = anyhow::Error;
    fn try_from(value: &HaXmlValue) -> Result<Self, Self::Error> {
        let dir = value.as_dir()?;
        Ok(Self {
            regen: dir.get_opt_key_mapped("regen")?,
            anger_gauge_effect: dir.get_opt_key_mapped("AngerGaugeEffect")?,
            die_f: dir.get_opt_key_mapped("dieF")?,
            _move: dir.get_opt_key_mapped("move")?,
            die: dir.get_opt_key_mapped("die")?,
            stand: dir.get_opt_key_mapped("stand")?,
            die_2: dir.get_opt_key_mapped("die2")?,
            chase: dir.get_opt_key_mapped("chase")?,
            attack_6: dir.get_opt_key_mapped("attack6")?,
            hit_1: dir.get_opt_key_mapped("hit1")?,
            attack_4: dir.get_opt_key_mapped("attack4")?,
            rope: dir.get_opt_key_mapped("rope")?,
            fly: dir.get_opt_key_mapped("fly")?,
            attack_3: dir.get_opt_key_mapped("attack3")?,
            jump: dir.get_opt_key_mapped("jump")?,
            hit: dir.get_opt_key_mapped("hit")?,
            anger_gauge_animation: dir.get_opt_key_mapped("AngerGaugeAnimation")?,
            info: dir.get_key_mapped("info")?,
            eye: dir.get_opt_key_mapped("eye")?,
            attack_2: dir.get_opt_key_mapped("attack2")?,
            attack_5: dir.get_opt_key_mapped("attack5")?,
            die_1: dir.get_opt_key_mapped("die1")?,
            stop: dir.get_opt_key_mapped("stop")?,
            say: dir.get_opt_key_mapped("say")?,
            miss: dir.get_opt_key_mapped("miss")?,
            attack_1: dir.get_opt_key_mapped("attack1")?,
            ladder: dir.get_opt_key_mapped("ladder")?,
        })
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Attack6 {
    pub lt: Vec2,
    pub delay: i64,
    pub info: Info,
    pub origin: Vec2,
    pub head: Vec2,
    pub rb: Vec2,
}
impl TryFrom<&HaXmlValue> for Attack6 {
    type Error = anyhow::Error;
    fn try_from(value: &HaXmlValue) -> Result<Self, Self::Error> {
        let dir = value.as_dir()?;
        Ok(Self {
            lt: dir.get_key_mapped("lt")?,
            delay: dir.get_key_mapped("delay")?,
            info: dir.get_key_mapped("info")?,
            origin: dir.get_key_mapped("origin")?,
            head: dir.get_key_mapped("head")?,
            rb: dir.get_key_mapped("rb")?,
        })
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Regen {
    pub head: Option<Vec2>,
    pub z: Option<i64>,
    pub delay: Option<i64>,
    pub rb: Option<Vec2>,
    pub origin: Vec2,
    pub lt: Option<Vec2>,
}
impl TryFrom<&HaXmlValue> for Regen {
    type Error = anyhow::Error;
    fn try_from(value: &HaXmlValue) -> Result<Self, Self::Error> {
        let dir = value.as_dir()?;
        Ok(Self {
            head: dir.get_opt_key_mapped("head")?,
            z: dir.get_opt_key_mapped("z")?,
            delay: dir.get_opt_key_mapped("delay")?,
            rb: dir.get_opt_key_mapped("rb")?,
            origin: dir.get_key_mapped("origin")?,
            lt: dir.get_opt_key_mapped("lt")?,
        })
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct _1 {
    pub prob: i64,
    pub _0: String,
    pub hp: i64,
}
impl TryFrom<&HaXmlValue> for _1 {
    type Error = anyhow::Error;
    fn try_from(value: &HaXmlValue) -> Result<Self, Self::Error> {
        let dir = value.as_dir()?;
        Ok(Self {
            prob: dir.get_key_mapped("prob")?,
            _0: dir.get_key_mapped("0")?,
            hp: dir.get_key_mapped("hp")?,
        })
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Index {
    pub hp: i64,
    pub _0: String,
    pub prob: i64,
}
impl TryFrom<&HaXmlValue> for Index {
    type Error = anyhow::Error;
    fn try_from(value: &HaXmlValue) -> Result<Self, Self::Error> {
        let dir = value.as_dir()?;
        Ok(Self {
            hp: dir.get_key_mapped("hp")?,
            _0: dir.get_key_mapped("0")?,
            prob: dir.get_key_mapped("prob")?,
        })
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Skill7 {
    pub delay: i64,
    pub origin: Vec2,
    pub head: Vec2,
}
impl TryFrom<&HaXmlValue> for Skill7 {
    type Error = anyhow::Error;
    fn try_from(value: &HaXmlValue) -> Result<Self, Self::Error> {
        let dir = value.as_dir()?;
        Ok(Self {
            delay: dir.get_key_mapped("delay")?,
            origin: dir.get_key_mapped("origin")?,
            head: dir.get_key_mapped("head")?,
        })
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Con {
    pub pet: Option<i64>,
    pub quest: Option<Quest>,
}
impl TryFrom<&HaXmlValue> for Con {
    type Error = anyhow::Error;
    fn try_from(value: &HaXmlValue) -> Result<Self, Self::Error> {
        let dir = value.as_dir()?;
        Ok(Self {
            pet: dir.get_opt_key_mapped("pet")?,
            quest: dir.get_opt_key_mapped("quest")?,
        })
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Default {
    pub origin: Vec2,
    pub lt: Option<Vec2>,
    pub z: Option<i64>,
    pub head: Option<Vec2>,
    pub rb: Option<Vec2>,
    pub delay: Option<i64>,
}
impl TryFrom<&HaXmlValue> for Default {
    type Error = anyhow::Error;
    fn try_from(value: &HaXmlValue) -> Result<Self, Self::Error> {
        let dir = value.as_dir()?;
        Ok(Self {
            origin: dir.get_key_mapped("origin")?,
            lt: dir.get_opt_key_mapped("lt")?,
            z: dir.get_opt_key_mapped("z")?,
            head: dir.get_opt_key_mapped("head")?,
            rb: dir.get_opt_key_mapped("rb")?,
            delay: dir.get_opt_key_mapped("delay")?,
        })
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct LoseItem {
    pub prop: i64,
    pub lose_msg_type: Option<i64>,
    pub lose_msg: Option<String>,
    pub not_drop: Option<i64>,
    pub id: i64,
    pub x: i64,
}
impl TryFrom<&HaXmlValue> for LoseItem {
    type Error = anyhow::Error;
    fn try_from(value: &HaXmlValue) -> Result<Self, Self::Error> {
        let dir = value.as_dir()?;
        Ok(Self {
            prop: dir.get_key_mapped("prop")?,
            lose_msg_type: dir.get_opt_key_mapped("loseMsgType")?,
            lose_msg: dir.get_opt_key_mapped("loseMsg")?,
            not_drop: dir.get_opt_key_mapped("notDrop")?,
            id: dir.get_key_mapped("id")?,
            x: dir.get_key_mapped("x")?,
        })
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Range {
    pub start: Option<i64>,
    pub lt: Option<Vec2>,
    pub area_count: Option<i64>,
    pub attack_count: Option<i64>,
    pub rb: Option<Vec2>,
    pub r: Option<i64>,
    pub sp: Option<Vec2>,
    pub delay: Option<i64>,
}
impl TryFrom<&HaXmlValue> for Range {
    type Error = anyhow::Error;
    fn try_from(value: &HaXmlValue) -> Result<Self, Self::Error> {
        let dir = value.as_dir()?;
        Ok(Self {
            start: dir.get_opt_key_mapped("start")?,
            lt: dir.get_opt_key_mapped("lt")?,
            area_count: dir.get_opt_key_mapped("areaCount")?,
            attack_count: dir.get_opt_key_mapped("attackCount")?,
            rb: dir.get_opt_key_mapped("rb")?,
            r: dir.get_opt_key_mapped("r")?,
            sp: dir.get_opt_key_mapped("sp")?,
            delay: dir.get_opt_key_mapped("delay")?,
        })
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Stand {
    pub rb: Option<Vec2>,
    pub lt: Option<Vec2>,
    pub a_0: Option<i64>,
    pub z: Option<i64>,
    pub head: Option<Vec2>,
    pub delay: Option<i64>,
    pub zigzag: Option<i64>,
    pub rect: Option<BTreeMap<i64, Rect>>,
    pub multi_rect: Option<BTreeMap<i64, MultiRect>>,
    pub info: Option<Info>,
    pub speak: Option<Speak>,
    pub origin: Vec2,
}
impl TryFrom<&HaXmlValue> for Stand {
    type Error = anyhow::Error;
    fn try_from(value: &HaXmlValue) -> Result<Self, Self::Error> {
        let dir = value.as_dir()?;
        Ok(Self {
            rb: dir.get_opt_key_mapped("rb")?,
            lt: dir.get_opt_key_mapped("lt")?,
            a_0: dir.get_opt_key_mapped("a0")?,
            z: dir.get_opt_key_mapped("z")?,
            head: dir.get_opt_key_mapped("head")?,
            delay: dir.get_opt_key_mapped("delay")?,
            zigzag: dir.get_opt_key_mapped("zigzag")?,
            rect: dir.get_opt_key_mapped("rect")?,
            multi_rect: dir.get_opt_key_mapped("multiRect")?,
            info: dir.get_opt_key_mapped("info")?,
            speak: dir.get_opt_key_mapped("speak")?,
            origin: dir.get_key_mapped("origin")?,
        })
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct HealOnDestroy {
    pub _type: i64,
    pub amount: i64,
}
impl TryFrom<&HaXmlValue> for HealOnDestroy {
    type Error = anyhow::Error;
    fn try_from(value: &HaXmlValue) -> Result<Self, Self::Error> {
        let dir = value.as_dir()?;
        Ok(Self {
            _type: dir.get_key_mapped("type")?,
            amount: dir.get_key_mapped("amount")?,
        })
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Rope {
    pub head: Vec2,
    pub origin: Vec2,
    pub lt: Option<Vec2>,
    pub delay: i64,
    pub rb: Option<Vec2>,
}
impl TryFrom<&HaXmlValue> for Rope {
    type Error = anyhow::Error;
    fn try_from(value: &HaXmlValue) -> Result<Self, Self::Error> {
        let dir = value.as_dir()?;
        Ok(Self {
            head: dir.get_key_mapped("head")?,
            origin: dir.get_key_mapped("origin")?,
            lt: dir.get_opt_key_mapped("lt")?,
            delay: dir.get_key_mapped("delay")?,
            rb: dir.get_opt_key_mapped("rb")?,
        })
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Fly {
    pub delay: Option<i64>,
    pub rect: Option<BTreeMap<i64, Rect>>,
    pub head: Option<Vec2>,
    pub rb: Option<Vec2>,
    pub origin: Vec2,
    pub z: Option<i64>,
    pub lt: Option<Vec2>,
    pub zigzag: Option<i64>,
}
impl TryFrom<&HaXmlValue> for Fly {
    type Error = anyhow::Error;
    fn try_from(value: &HaXmlValue) -> Result<Self, Self::Error> {
        let dir = value.as_dir()?;
        Ok(Self {
            delay: dir.get_opt_key_mapped("delay")?,
            rect: dir.get_opt_key_mapped("rect")?,
            head: dir.get_opt_key_mapped("head")?,
            rb: dir.get_opt_key_mapped("rb")?,
            origin: dir.get_key_mapped("origin")?,
            z: dir.get_opt_key_mapped("z")?,
            lt: dir.get_opt_key_mapped("lt")?,
            zigzag: dir.get_opt_key_mapped("zigzag")?,
        })
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Ball {
    pub delay: Option<i64>,
    pub rotate_period: Option<i64>,
    pub origin: Vec2,
    pub z: Option<i64>,
}
impl TryFrom<&HaXmlValue> for Ball {
    type Error = anyhow::Error;
    fn try_from(value: &HaXmlValue) -> Result<Self, Self::Error> {
        let dir = value.as_dir()?;
        Ok(Self {
            delay: dir.get_opt_key_mapped("delay")?,
            rotate_period: dir.get_opt_key_mapped("rotatePeriod")?,
            origin: dir.get_key_mapped("origin")?,
            z: dir.get_opt_key_mapped("z")?,
        })
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct AngerGaugeEffect {
    pub origin: Vec2,
    pub delay: i64,
}
impl TryFrom<&HaXmlValue> for AngerGaugeEffect {
    type Error = anyhow::Error;
    fn try_from(value: &HaXmlValue) -> Result<Self, Self::Error> {
        let dir = value.as_dir()?;
        Ok(Self {
            origin: dir.get_key_mapped("origin")?,
            delay: dir.get_key_mapped("delay")?,
        })
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Die {
    pub head: Option<Vec2>,
    pub delay: i64,
    pub origin: Vec2,
}
impl TryFrom<&HaXmlValue> for Die {
    type Error = anyhow::Error;
    fn try_from(value: &HaXmlValue) -> Result<Self, Self::Error> {
        let dir = value.as_dir()?;
        Ok(Self {
            head: dir.get_opt_key_mapped("head")?,
            delay: dir.get_key_mapped("delay")?,
            origin: dir.get_key_mapped("origin")?,
        })
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Speak {
    pub prob: Option<i64>,
    pub prop: Option<i64>,
    pub con: Option<BTreeMap<i64, Con>>,
    pub chat_balloon: Option<i64>,
    pub index: Option<Index>,
    pub _2: Option<_2>,
    pub _1: Option<_1>,
    pub chata_balloon: Option<i64>,
    pub width_size: Option<i64>,
    pub _0: Option<String>,
}
impl TryFrom<&HaXmlValue> for Speak {
    type Error = anyhow::Error;
    fn try_from(value: &HaXmlValue) -> Result<Self, Self::Error> {
        let dir = value.as_dir()?;
        Ok(Self {
            prob: dir.get_opt_key_mapped("prob")?,
            prop: dir.get_opt_key_mapped("prop")?,
            con: dir.get_opt_key_mapped("con")?,
            chat_balloon: dir.get_opt_key_mapped("chatBalloon")?,
            index: dir.get_opt_key_mapped("Index")?,
            _2: dir.get_opt_key_mapped("2")?,
            _1: dir.get_opt_key_mapped("1")?,
            chata_balloon: dir.get_opt_key_mapped("chataBalloon")?,
            width_size: dir.get_opt_key_mapped("widthSize")?,
            _0: dir.get_opt_key_mapped("0")?,
        })
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Rect {
    pub rb: Vec2,
    pub lt: Vec2,
}
impl TryFrom<&HaXmlValue> for Rect {
    type Error = anyhow::Error;
    fn try_from(value: &HaXmlValue) -> Result<Self, Self::Error> {
        let dir = value.as_dir()?;
        Ok(Self {
            rb: dir.get_key_mapped("rb")?,
            lt: dir.get_key_mapped("lt")?,
        })
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Stop {
    pub rb: Vec2,
    pub delay: i64,
    pub rect: Option<BTreeMap<i64, Rect>>,
    pub lt: Vec2,
    pub origin: Vec2,
    pub head: Vec2,
}
impl TryFrom<&HaXmlValue> for Stop {
    type Error = anyhow::Error;
    fn try_from(value: &HaXmlValue) -> Result<Self, Self::Error> {
        let dir = value.as_dir()?;
        Ok(Self {
            rb: dir.get_key_mapped("rb")?,
            delay: dir.get_key_mapped("delay")?,
            rect: dir.get_opt_key_mapped("rect")?,
            lt: dir.get_key_mapped("lt")?,
            origin: dir.get_key_mapped("origin")?,
            head: dir.get_key_mapped("head")?,
        })
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Ban {
    pub ban_type: Option<i64>,
    pub ban_msg: String,
    pub ban_map: BTreeMap<i64, BanMap>,
    pub ban_msg_type: Option<i64>,
}
impl TryFrom<&HaXmlValue> for Ban {
    type Error = anyhow::Error;
    fn try_from(value: &HaXmlValue) -> Result<Self, Self::Error> {
        let dir = value.as_dir()?;
        Ok(Self {
            ban_type: dir.get_opt_key_mapped("banType")?,
            ban_msg: dir.get_key_mapped("banMsg")?,
            ban_map: dir.get_key_mapped("banMap")?,
            ban_msg_type: dir.get_opt_key_mapped("banMsgType")?,
        })
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Eye {
    pub lt: Option<Vec2>,
    pub head: Vec2,
    pub delay: i64,
    pub origin: Vec2,
    pub rb: Option<Vec2>,
}
impl TryFrom<&HaXmlValue> for Eye {
    type Error = anyhow::Error;
    fn try_from(value: &HaXmlValue) -> Result<Self, Self::Error> {
        let dir = value.as_dir()?;
        Ok(Self {
            lt: dir.get_opt_key_mapped("lt")?,
            head: dir.get_key_mapped("head")?,
            delay: dir.get_key_mapped("delay")?,
            origin: dir.get_key_mapped("origin")?,
            rb: dir.get_opt_key_mapped("rb")?,
        })
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Skill5 {
    pub delay: i64,
    pub head: Vec2,
    pub origin: Vec2,
}
impl TryFrom<&HaXmlValue> for Skill5 {
    type Error = anyhow::Error;
    fn try_from(value: &HaXmlValue) -> Result<Self, Self::Error> {
        let dir = value.as_dir()?;
        Ok(Self {
            delay: dir.get_key_mapped("delay")?,
            head: dir.get_key_mapped("head")?,
            origin: dir.get_key_mapped("origin")?,
        })
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Skill1 {
    pub z: Option<i64>,
    pub a_1: Option<i64>,
    pub a_0: Option<i64>,
    pub speak: Option<Speak>,
    pub delay: Option<i64>,
    pub info: Option<Info>,
    pub origin: Option<Vec2>,
    pub head: Option<Vec2>,
    pub lt: Option<Vec2>,
    pub rb: Option<Vec2>,
    pub rect: Option<BTreeMap<i64, Rect>>,
}
impl TryFrom<&HaXmlValue> for Skill1 {
    type Error = anyhow::Error;
    fn try_from(value: &HaXmlValue) -> Result<Self, Self::Error> {
        let dir = value.as_dir()?;
        Ok(Self {
            z: dir.get_opt_key_mapped("z")?,
            a_1: dir.get_opt_key_mapped("a1")?,
            a_0: dir.get_opt_key_mapped("a0")?,
            speak: dir.get_opt_key_mapped("speak")?,
            delay: dir.get_opt_key_mapped("delay")?,
            info: dir.get_opt_key_mapped("info")?,
            origin: dir.get_opt_key_mapped("origin")?,
            head: dir.get_opt_key_mapped("head")?,
            lt: dir.get_opt_key_mapped("lt")?,
            rb: dir.get_opt_key_mapped("rb")?,
            rect: dir.get_opt_key_mapped("rect")?,
        })
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Move {
    pub rb: Option<Vec2>,
    pub head: Option<Vec2>,
    pub z: Option<i64>,
    pub lt: Option<Vec2>,
    pub origin: Vec2,
    pub rect: Option<BTreeMap<i64, Rect>>,
    pub category: Option<i64>,
    pub zigzag: Option<i64>,
    pub delay: Option<i64>,
    pub a_0: Option<i64>,
}
impl TryFrom<&HaXmlValue> for Move {
    type Error = anyhow::Error;
    fn try_from(value: &HaXmlValue) -> Result<Self, Self::Error> {
        let dir = value.as_dir()?;
        Ok(Self {
            rb: dir.get_opt_key_mapped("rb")?,
            head: dir.get_opt_key_mapped("head")?,
            z: dir.get_opt_key_mapped("z")?,
            lt: dir.get_opt_key_mapped("lt")?,
            origin: dir.get_key_mapped("origin")?,
            rect: dir.get_opt_key_mapped("rect")?,
            category: dir.get_opt_key_mapped("category")?,
            zigzag: dir.get_opt_key_mapped("zigzag")?,
            delay: dir.get_opt_key_mapped("delay")?,
            a_0: dir.get_opt_key_mapped("a0")?,
        })
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Skill3 {
    pub origin: Vec2,
    pub head: Vec2,
    pub delay: Option<i64>,
    pub rb: Option<Vec2>,
    pub rect: Option<BTreeMap<i64, Rect>>,
    pub lt: Option<Vec2>,
    pub z: Option<i64>,
}
impl TryFrom<&HaXmlValue> for Skill3 {
    type Error = anyhow::Error;
    fn try_from(value: &HaXmlValue) -> Result<Self, Self::Error> {
        let dir = value.as_dir()?;
        Ok(Self {
            origin: dir.get_key_mapped("origin")?,
            head: dir.get_key_mapped("head")?,
            delay: dir.get_opt_key_mapped("delay")?,
            rb: dir.get_opt_key_mapped("rb")?,
            rect: dir.get_opt_key_mapped("rect")?,
            lt: dir.get_opt_key_mapped("lt")?,
            z: dir.get_opt_key_mapped("z")?,
        })
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Die2 {
    pub origin: Vec2,
    pub delay: i64,
    pub a_1: Option<i64>,
    pub head: Vec2,
    pub a_0: Option<i64>,
}
impl TryFrom<&HaXmlValue> for Die2 {
    type Error = anyhow::Error;
    fn try_from(value: &HaXmlValue) -> Result<Self, Self::Error> {
        let dir = value.as_dir()?;
        Ok(Self {
            origin: dir.get_key_mapped("origin")?,
            delay: dir.get_key_mapped("delay")?,
            a_1: dir.get_opt_key_mapped("a1")?,
            head: dir.get_key_mapped("head")?,
            a_0: dir.get_opt_key_mapped("a0")?,
        })
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct _0 {
    pub _1: Option<String>,
    pub prob: Option<i64>,
    pub rb: Option<Vec2>,
    pub hp: Option<i64>,
    pub lt: Option<Vec2>,
    pub _0: Option<String>,
}
impl TryFrom<&HaXmlValue> for _0 {
    type Error = anyhow::Error;
    fn try_from(value: &HaXmlValue) -> Result<Self, Self::Error> {
        let dir = value.as_dir()?;
        Ok(Self {
            _1: dir.get_opt_key_mapped("1")?,
            prob: dir.get_opt_key_mapped("prob")?,
            rb: dir.get_opt_key_mapped("rb")?,
            hp: dir.get_opt_key_mapped("hp")?,
            lt: dir.get_opt_key_mapped("lt")?,
            _0: dir.get_opt_key_mapped("0")?,
        })
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Quest {
    pub quest_id: i64,
    pub state: i64,
}
impl TryFrom<&HaXmlValue> for Quest {
    type Error = anyhow::Error;
    fn try_from(value: &HaXmlValue) -> Result<Self, Self::Error> {
        let dir = value.as_dir()?;
        Ok(Self {
            quest_id: dir.get_key_mapped("questID")?,
            state: dir.get_key_mapped("state")?,
        })
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Attack1 {
    pub rect: Option<BTreeMap<i64, Rect>>,
    pub speak: Option<Speak>,
    pub lt: Option<Vec2>,
    pub sp: Option<Vec2>,
    pub origin: Option<Vec2>,
    pub hit: Option<Hit>,
    pub z: Option<i64>,
    pub head: Option<Vec2>,
    pub delay: Option<i64>,
    pub a_1: Option<i64>,
    pub info: Info,
    pub a_0: Option<i64>,
    pub rb: Option<Vec2>,
    pub dealy: Option<i64>,
}
impl TryFrom<&HaXmlValue> for Attack1 {
    type Error = anyhow::Error;
    fn try_from(value: &HaXmlValue) -> Result<Self, Self::Error> {
        let dir = value.as_dir()?;
        Ok(Self {
            rect: dir.get_opt_key_mapped("rect")?,
            speak: dir.get_opt_key_mapped("speak")?,
            lt: dir.get_opt_key_mapped("lt")?,
            sp: dir.get_opt_key_mapped("sp")?,
            origin: dir.get_opt_key_mapped("origin")?,
            hit: dir.get_opt_key_mapped("hit")?,
            z: dir.get_opt_key_mapped("z")?,
            head: dir.get_opt_key_mapped("head")?,
            delay: dir.get_opt_key_mapped("delay")?,
            a_1: dir.get_opt_key_mapped("a1")?,
            info: dir.get_key_mapped("info")?,
            a_0: dir.get_opt_key_mapped("a0")?,
            rb: dir.get_opt_key_mapped("rb")?,
            dealy: dir.get_opt_key_mapped("dealy")?,
        })
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Attack5 {
    pub delay: i64,
    pub info: Info,
    pub head: Vec2,
    pub origin: Vec2,
    pub z: Option<i64>,
    pub lt: Vec2,
    pub rb: Vec2,
}
impl TryFrom<&HaXmlValue> for Attack5 {
    type Error = anyhow::Error;
    fn try_from(value: &HaXmlValue) -> Result<Self, Self::Error> {
        let dir = value.as_dir()?;
        Ok(Self {
            delay: dir.get_key_mapped("delay")?,
            info: dir.get_key_mapped("info")?,
            head: dir.get_key_mapped("head")?,
            origin: dir.get_key_mapped("origin")?,
            z: dir.get_opt_key_mapped("z")?,
            lt: dir.get_key_mapped("lt")?,
            rb: dir.get_key_mapped("rb")?,
        })
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct AngerGaugeAnimation {
    pub origin: Vec2,
}
impl TryFrom<&HaXmlValue> for AngerGaugeAnimation {
    type Error = anyhow::Error;
    fn try_from(value: &HaXmlValue) -> Result<Self, Self::Error> {
        let dir = value.as_dir()?;
        Ok(Self {
            origin: dir.get_key_mapped("origin")?,
        })
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct BanMap {
    pub field: i64,
    pub portal: Option<String>,
    pub potal: Option<String>,
}
impl TryFrom<&HaXmlValue> for BanMap {
    type Error = anyhow::Error;
    fn try_from(value: &HaXmlValue) -> Result<Self, Self::Error> {
        let dir = value.as_dir()?;
        Ok(Self {
            field: dir.get_key_mapped("field")?,
            portal: dir.get_opt_key_mapped("portal")?,
            potal: dir.get_opt_key_mapped("potal")?,
        })
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Attack2 {
    pub info: Info,
    pub delay: Option<i64>,
    pub lt: Option<Vec2>,
    pub head: Option<Vec2>,
    pub rect: Option<BTreeMap<i64, Rect>>,
    pub origin: Option<Vec2>,
    pub rb_1: Option<Vec2>,
    pub speak: Option<Speak>,
    pub a_1: Option<i64>,
    pub rb: Option<Vec2>,
    pub z: Option<i64>,
    pub a_0: Option<i64>,
}
impl TryFrom<&HaXmlValue> for Attack2 {
    type Error = anyhow::Error;
    fn try_from(value: &HaXmlValue) -> Result<Self, Self::Error> {
        let dir = value.as_dir()?;
        Ok(Self {
            info: dir.get_key_mapped("info")?,
            delay: dir.get_opt_key_mapped("delay")?,
            lt: dir.get_opt_key_mapped("lt")?,
            head: dir.get_opt_key_mapped("head")?,
            rect: dir.get_opt_key_mapped("rect")?,
            origin: dir.get_opt_key_mapped("origin")?,
            rb_1: dir.get_opt_key_mapped("rb1")?,
            speak: dir.get_opt_key_mapped("speak")?,
            a_1: dir.get_opt_key_mapped("a1")?,
            rb: dir.get_opt_key_mapped("rb")?,
            z: dir.get_opt_key_mapped("z")?,
            a_0: dir.get_opt_key_mapped("a0")?,
        })
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct DieF {
    pub origin: Vec2,
    pub head: Vec2,
    pub a_0: Option<i64>,
    pub a_1: Option<i64>,
    pub delay: i64,
}
impl TryFrom<&HaXmlValue> for DieF {
    type Error = anyhow::Error;
    fn try_from(value: &HaXmlValue) -> Result<Self, Self::Error> {
        let dir = value.as_dir()?;
        Ok(Self {
            origin: dir.get_key_mapped("origin")?,
            head: dir.get_key_mapped("head")?,
            a_0: dir.get_opt_key_mapped("a0")?,
            a_1: dir.get_opt_key_mapped("a1")?,
            delay: dir.get_key_mapped("delay")?,
        })
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Say {
    pub lt: Option<Vec2>,
    pub head: Vec2,
    pub delay: i64,
    pub origin: Vec2,
    pub speak: Speak,
    pub rb: Option<Vec2>,
}
impl TryFrom<&HaXmlValue> for Say {
    type Error = anyhow::Error;
    fn try_from(value: &HaXmlValue) -> Result<Self, Self::Error> {
        let dir = value.as_dir()?;
        Ok(Self {
            lt: dir.get_opt_key_mapped("lt")?,
            head: dir.get_key_mapped("head")?,
            delay: dir.get_key_mapped("delay")?,
            origin: dir.get_key_mapped("origin")?,
            speak: dir.get_key_mapped("speak")?,
            rb: dir.get_opt_key_mapped("rb")?,
        })
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Attack4 {
    pub z: Option<i64>,
    pub speak: Option<Speak>,
    pub origin: Option<Vec2>,
    pub head: Option<Vec2>,
    pub rb: Option<Vec2>,
    pub info: Info,
    pub delay: Option<i64>,
    pub lt: Option<Vec2>,
    pub rect: Option<BTreeMap<i64, Rect>>,
}
impl TryFrom<&HaXmlValue> for Attack4 {
    type Error = anyhow::Error;
    fn try_from(value: &HaXmlValue) -> Result<Self, Self::Error> {
        let dir = value.as_dir()?;
        Ok(Self {
            z: dir.get_opt_key_mapped("z")?,
            speak: dir.get_opt_key_mapped("speak")?,
            origin: dir.get_opt_key_mapped("origin")?,
            head: dir.get_opt_key_mapped("head")?,
            rb: dir.get_opt_key_mapped("rb")?,
            info: dir.get_key_mapped("info")?,
            delay: dir.get_opt_key_mapped("delay")?,
            lt: dir.get_opt_key_mapped("lt")?,
            rect: dir.get_opt_key_mapped("rect")?,
        })
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SelfDestruction {
    pub hp: Option<i64>,
    pub action: i64,
    pub remove_after: Option<i64>,
}
impl TryFrom<&HaXmlValue> for SelfDestruction {
    type Error = anyhow::Error;
    fn try_from(value: &HaXmlValue) -> Result<Self, Self::Error> {
        let dir = value.as_dir()?;
        Ok(Self {
            hp: dir.get_opt_key_mapped("hp")?,
            action: dir.get_key_mapped("action")?,
            remove_after: dir.get_opt_key_mapped("removeAfter")?,
        })
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct _2 {
    pub _0: String,
    pub hp: i64,
    pub prob: i64,
}
impl TryFrom<&HaXmlValue> for _2 {
    type Error = anyhow::Error;
    fn try_from(value: &HaXmlValue) -> Result<Self, Self::Error> {
        let dir = value.as_dir()?;
        Ok(Self {
            _0: dir.get_key_mapped("0")?,
            hp: dir.get_key_mapped("hp")?,
            prob: dir.get_key_mapped("prob")?,
        })
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Attack3 {
    pub delay: Option<i64>,
    pub rb: Option<Vec2>,
    pub head: Option<Vec2>,
    pub z: Option<i64>,
    pub info: Info,
    pub rect: Option<BTreeMap<i64, Rect>>,
    pub speak: Option<Speak>,
    pub lt: Option<Vec2>,
    pub origin: Option<Vec2>,
}
impl TryFrom<&HaXmlValue> for Attack3 {
    type Error = anyhow::Error;
    fn try_from(value: &HaXmlValue) -> Result<Self, Self::Error> {
        let dir = value.as_dir()?;
        Ok(Self {
            delay: dir.get_opt_key_mapped("delay")?,
            rb: dir.get_opt_key_mapped("rb")?,
            head: dir.get_opt_key_mapped("head")?,
            z: dir.get_opt_key_mapped("z")?,
            info: dir.get_key_mapped("info")?,
            rect: dir.get_opt_key_mapped("rect")?,
            speak: dir.get_opt_key_mapped("speak")?,
            lt: dir.get_opt_key_mapped("lt")?,
            origin: dir.get_opt_key_mapped("origin")?,
        })
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Ladder {
    pub rb: Vec2,
    pub lt: Vec2,
    pub head: Vec2,
    pub delay: i64,
    pub origin: Vec2,
}
impl TryFrom<&HaXmlValue> for Ladder {
    type Error = anyhow::Error;
    fn try_from(value: &HaXmlValue) -> Result<Self, Self::Error> {
        let dir = value.as_dir()?;
        Ok(Self {
            rb: dir.get_key_mapped("rb")?,
            lt: dir.get_key_mapped("lt")?,
            head: dir.get_key_mapped("head")?,
            delay: dir.get_key_mapped("delay")?,
            origin: dir.get_key_mapped("origin")?,
        })
    }
}
