use std::{collections::BTreeMap, fs::File};

use game_data::{
    gen,
    ha_xml::{HaXmlNumericDir, HaXmlValue},
    schema::Schema,
};

fn preprocess_map(v: &mut HaXmlValue) {
    // Remove any numeric key
    let HaXmlValue::Dir(dir) = v else {
        return;
    };

    // Take the first node which is an img
    let HaXmlValue::Dir(map_dir) = dir.0.first_entry().unwrap().remove() else {
        return;
    };

    let mut extra_dir = HaXmlNumericDir::default();

    for (k, v) in map_dir.0.into_iter() {
        if let Ok(k) = k.parse::<i64>() {
            extra_dir.0.insert(k, v);
        } else {
            dir.0.insert(k, v);
        }
    }

    dir.0
        .insert("extra".to_string(), HaXmlValue::NumericDir(extra_dir));
}

fn main() -> anyhow::Result<()> {
    let mut v = HaXmlValue::load_from_glob(
        glob::glob("game_data/Map.wz/Map/Map0/*.xml").unwrap(),
        &["0char", "2char", "ToolTip", "nodeInfo"],
    )
    .unwrap();
    for (_, v) in v.iter_mut() {
        preprocess_map(v);
    }

    let dirs = v.values().map(|v| match v {
        HaXmlValue::Dir(v) => v,
        _ => unreachable!(),
    });
    let schema = Schema::from_multiple_roots_dir("map", dirs);

    let mut str = String::new();
    schema.fmt_rust(&mut str)?;
    //println!("{str}");

    let mapped_maps  = v
        .iter()
        .map(|(k, v)| Ok((k.parse::<u64>()?, <gen::map::Map>::try_from(v)?)))
        .collect::<anyhow::Result<BTreeMap<u64, gen::map::Map>>>()?;


    let mut f = File::create("map0.rbin")?;
    bincode::serialize_into(&mut f, &mapped_maps)?;
    println!("Wrote maps, cwd: {:?}", std::env::current_dir()?);
    Ok(())
}
