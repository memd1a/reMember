use std::{
    collections::{BTreeMap, HashMap},
    fs::File,
    io::{BufRead, BufReader, Cursor},
};

use anyhow::Context;
use quick_xml::{events::Event, Reader};
use rayon::prelude::{ParallelBridge, ParallelIterator};
use serde::{
    ser::{SerializeMap, SerializeSeq},
    Serialize,
    Deserialize
};

#[derive(Debug, Default, Clone)]
pub struct HaXmlDir(pub BTreeMap<String, HaXmlValue>);

impl HaXmlDir {
    pub fn try_get(&self, key: &str) -> anyhow::Result<&HaXmlValue> {
        self.0
            .get(key)
            .ok_or_else(|| anyhow::format_err!("Unknown key: {}", key))
    }

    pub fn get_key_mapped<'a, T: TryFrom<&'a HaXmlValue, Error = anyhow::Error>>(
        &'a self,
        key: &str,
    ) -> anyhow::Result<T> {
        self.try_get(key)?.try_into().context(key.to_string())
    }

    pub fn get_opt_key_mapped<'a, T: TryFrom<&'a HaXmlValue, Error = anyhow::Error>>(
        &'a self,
        key: &str,
    ) -> anyhow::Result<Option<T>> {
        Ok(match self.0.get(key) {
            Some(v) => Some(v.try_into().context(key.to_string())?),
            None => None,
        })
    }
}

#[derive(Debug, Default, Clone)]
pub struct HaXmlNumericDir(pub BTreeMap<i64, HaXmlValue>);

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Vec2 {
    pub x: i64,
    pub y: i64,
}

#[derive(Debug, Clone)]
pub enum HaXmlValue {
    Dir(HaXmlDir),
    NumericDir(HaXmlNumericDir),
    Vector(Vec2),
    Int(i64),
    Float(f64),
    String(String),
}

impl TryFrom<&HaXmlValue> for i64 {
    type Error = anyhow::Error;

    fn try_from(value: &HaXmlValue) -> Result<Self, Self::Error> {
        value.as_int()
    }
}

impl TryFrom<&HaXmlValue> for f32 {
    type Error = anyhow::Error;

    fn try_from(value: &HaXmlValue) -> Result<Self, Self::Error> {
        Ok(value.as_float()? as f32)
    }
}

impl TryFrom<&HaXmlValue> for f64 {
    type Error = anyhow::Error;

    fn try_from(value: &HaXmlValue) -> Result<Self, Self::Error> {
        value.as_float()
    }
}

impl<'a> TryFrom<&'a HaXmlValue> for &'a str {
    type Error = anyhow::Error;

    fn try_from(value: &'a HaXmlValue) -> Result<Self, Self::Error> {
        value.as_str()
    }
}

impl TryFrom<&HaXmlValue> for String {
    type Error = anyhow::Error;

    fn try_from(value: &HaXmlValue) -> Result<Self, Self::Error> {
        value.as_str().map(|s| s.to_string())
    }
}

impl<'a, T: TryFrom<&'a HaXmlValue, Error = anyhow::Error>> TryFrom<&'a HaXmlValue> for BTreeMap<i64, T> {
    type Error = T::Error;

    fn try_from(value: &'a HaXmlValue) -> Result<Self, Self::Error> {
        let num_dir = value.as_num_dir()?;

        num_dir.0.iter()
            .map(|(k, v)| {
                Ok((*k, v.try_into()?))
            })
            .collect()
    }
}

impl TryFrom<&HaXmlValue> for Vec2 {
    type Error = anyhow::Error;

    fn try_from(value: &HaXmlValue) -> Result<Self, Self::Error> {
        value.as_vec2()
    }
}


#[derive(Debug, Serialize)]
pub struct KeyValue<'a> {
    pub key: i64,
    pub value: &'a HaXmlValue,
}

impl Serialize for HaXmlValue {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            HaXmlValue::Float(v) => serializer.serialize_f64(*v),
            HaXmlValue::Int(v) => serializer.serialize_i64(*v),
            HaXmlValue::String(v) => serializer.serialize_str(v),
            HaXmlValue::Vector(v) => v.serialize(serializer),
            HaXmlValue::NumericDir(dir) => {
                let mut seq = serializer.serialize_seq(Some(dir.0.len()))?;
                for (key, value) in dir.0.iter() {
                    seq.serialize_element(&KeyValue { key: *key, value })?;
                }
                seq.end()
            }
            HaXmlValue::Dir(dir) => {
                let mut seq = serializer.serialize_map(Some(dir.0.len()))?;
                for (key, value) in dir.0.iter() {
                    seq.serialize_entry(key, value)?;
                }
                seq.end()
            }
        }
    }
}

impl HaXmlValue {
    pub fn as_dir(&self) -> anyhow::Result<&HaXmlDir> {
        Ok(match self {
            Self::Dir(dir) => dir,
            _ => anyhow::bail!("Invalid value: {:#?}, expected dir", self),
        })
    }


    pub fn as_num_dir(&self) -> anyhow::Result<&HaXmlNumericDir> {
        Ok(match self {
            Self::NumericDir(dir) => dir,
            _ => anyhow::bail!("Invalid value: {:#?}, expected num dir", self),
        })
    }

    pub fn as_int(&self) -> anyhow::Result<i64> {
        Ok(match self {
            Self::Int(v) => *v,
            _ => anyhow::bail!("Invalid value: {:#?}, expected num int", self),
        })
    }

    pub fn as_float(&self) -> anyhow::Result<f64> {
        Ok(match self {
            Self::Float(v) => *v,
            _ => anyhow::bail!("Invalid value: {:#?}, expected num float", self),
        })
    }

    pub fn as_str(&self) -> anyhow::Result<&str> {
        Ok(match self {
            Self::String(v) => v,
            _ => anyhow::bail!("Invalid value: {:#?}, expected num str", self),
        })
    }

    pub fn as_vec2(&self) -> anyhow::Result<Vec2> {
        Ok(match self {
            Self::Vector(v) => v.clone(),
            _ => anyhow::bail!("Invalid value: {:#?}, expected num vec2", self),
        })
    }

    pub fn from_data(data: &[u8], filter_keys: &[&'static str]) -> anyhow::Result<Self> {
        Self::from_reader(Cursor::new(data), filter_keys)
    }

    pub fn from_reader<R: BufRead>(r: R, filter_keys: &[&'static str]) -> anyhow::Result<Self> {
        let mut reader = Reader::from_reader(r);
        reader.trim_text(true);

        const IMG_DIR: &[u8] = b"imgdir";
        let root = BTreeMap::<String, HaXmlValue>::new();
        let mut stack = vec![(String::new(), root)];

        let mut buf = Vec::new();

        loop {
            match reader.read_event_into(&mut buf) {
                Err(e) => anyhow::bail!("Error at position {}: {:?}", reader.buffer_position(), e),
                // exits the loop when reaching end of file
                Ok(Event::Eof) => break,
                Ok(Event::Empty(e)) => {
                    let empty_name = e.name();
                    let empty_name = std::str::from_utf8(empty_name.as_ref())?;

                    let name = e
                        .try_get_attribute("name")?
                        .ok_or_else(|| anyhow::format_err!("No name attr"))?
                        .unescape_value()?
                        .to_string();

                    if filter_keys.iter().any(|&k| k == name.as_str()) {
                        continue;
                    }

                    let val = if empty_name == "vector" {
                        let x = e
                            .try_get_attribute("x")?
                            .ok_or_else(|| anyhow::format_err!("No x attr: {name}"))?
                            .unescape_value()?
                            .to_string()
                            .parse()?;

                        let y = e
                            .try_get_attribute("y")?
                            .ok_or_else(|| anyhow::format_err!("No y attr: {name}"))?
                            .unescape_value()?
                            .to_string()
                            .parse()?;

                        HaXmlValue::Vector(Vec2 { x, y })
                    } else {
                        let value = e
                            .try_get_attribute("value")?
                            .ok_or_else(|| {
                                anyhow::format_err!("No value attr: {name} - {empty_name}")
                            })?
                            .unescape_value()?
                            .to_string();

                        let val_ty = e.name();
                        let val_ty = std::str::from_utf8(val_ty.as_ref())?;
                        match val_ty {
                            "int" | "short" => HaXmlValue::Int(value.parse()?),
                            "string" => HaXmlValue::String(value),
                            "float" => HaXmlValue::Float(value.parse()?),
                            "canvas" | "uol" => continue,
                            _ => anyhow::bail!("Unsupported value: {name} - {val_ty}"),
                        }
                    };

                    stack
                        .last_mut()
                        .ok_or_else(|| anyhow::anyhow!("No Root element left"))?
                        .1
                        .insert(name, val);
                }

                Ok(Event::Start(e)) => match e.name().as_ref() {
                    IMG_DIR => {
                        let name = e
                            .try_get_attribute("name")?
                            .ok_or_else(|| anyhow::format_err!("No name attr"))?
                            .unescape_value()?
                            .to_string();
                        stack.push((name, BTreeMap::new()));
                    }
                    _ => (),
                },
                Ok(Event::End(e)) => match e.name().as_ref() {
                    IMG_DIR => {
                        let (name, dir) = stack
                            .pop()
                            .ok_or_else(|| anyhow::anyhow!("No closing for imgdir"))?;

                        let dir = if dir.keys().all(|v| v.parse::<i64>().is_ok()) {
                            HaXmlValue::NumericDir(HaXmlNumericDir(
                                dir.into_iter()
                                    .map(|(k, v)| (k.parse().unwrap(), v))
                                    .collect(),
                            ))
                        } else {
                            HaXmlValue::Dir(HaXmlDir(dir))
                        };

                        if filter_keys.iter().any(|&k| k == name.as_str()) {
                            continue;
                        }

                        stack
                            .last_mut()
                            .ok_or_else(|| anyhow::anyhow!("No Root element left"))?
                            .1
                            .insert(name, dir);
                    }
                    _ => (),
                },
                Ok(Event::Text(_)) => anyhow::bail!("Text nodes not supported"),
                _ => (),
            }
        }

        Ok(HaXmlValue::Dir(HaXmlDir(
            stack
                .pop()
                .ok_or_else(|| anyhow::anyhow!("No Root element left"))?
                .1,
        )))
    }

    pub fn load_from_glob(
        file_glob: glob::Paths,
        filter_keys: &[&'static str],
    ) -> anyhow::Result<HashMap<String, Self>> {
        file_glob
            .par_bridge()
            .map(|path| {
                let path = path?;
                let name = path
                    .file_stem()
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .trim_end_matches(".img")
                    .to_string();
                let f = File::open(path)?;
                let v = Self::from_reader(BufReader::new(f), filter_keys)?;
                Ok((name, v))
            })
            .collect()
    }
}
