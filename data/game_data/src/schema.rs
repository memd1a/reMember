use std::{borrow::Cow, collections::HashMap, fmt};

use convert_case::{Case, Casing};
use quote::{format_ident, IdentFragment, ToTokens, __private::TokenStream};

use crate::ha_xml::{HaXmlDir, HaXmlNumericDir, HaXmlValue};

pub type SchemaRef = String;

#[derive(Debug, Clone)]
pub enum SchemaValue {
    Int,
    Float,
    Vec2,
    String,
    Struct(SchemaRef),
    NumericDir(SchemaRef),
    Optional(Box<SchemaValue>),
}

impl SchemaValue {
    pub fn into_optional(self) -> SchemaValue {
        SchemaValue::Optional(Box::new(self))
    }

    pub fn is_optional(&self) -> bool {
        matches!(self, SchemaValue::Optional(_))
    }

    pub fn make_optional(&mut self) {
        if !self.is_optional() {
            let opt = self.clone().into_optional();
            *self = opt;
        }
    }
}

impl SchemaValue {
    pub fn from_haxml_value(val: &HaXmlValue, key: Option<&str>) -> Self {
        match val {
            HaXmlValue::Float(_) => SchemaValue::Float,
            HaXmlValue::Int(_) => SchemaValue::Int,
            HaXmlValue::String(_) => SchemaValue::String,
            HaXmlValue::Vector(_) => SchemaValue::Vec2,
            HaXmlValue::Dir(_) => SchemaValue::Struct(fmt_type_name(key.unwrap())),
            HaXmlValue::NumericDir(_) => SchemaValue::NumericDir(fmt_type_name(key.unwrap())),
        }
    }

    pub fn to_rust_type(&self) -> Cow<'_, str> {
        match self {
            SchemaValue::Float => "f32".into(),
            SchemaValue::Int => "i64".into(),
            SchemaValue::String => "String".into(),
            SchemaValue::Vec2 => "Vec2".into(),
            SchemaValue::Struct(name) => name.as_str().into(),
            SchemaValue::NumericDir(name) => format!("BTreeMap<i64, {name}>").into(),
            SchemaValue::Optional(opt) => {
                let ty = opt.to_rust_type();
                format!("Option<{ty}>").into()
            }
        }
    }

    pub fn to_rust_type_token(&self) -> TokenStream {
        match self {
            SchemaValue::Float => quote::quote!(f32),
            SchemaValue::Int => quote::quote!(i64),
            SchemaValue::String => quote::quote!(String),
            SchemaValue::Vec2 => quote::quote!(Vec2),
            SchemaValue::Struct(name) => {
                let id = if name.parse::<usize>().is_ok() {
                    format_ident!("_{name}")
                } else {
                    format_ident!("{name}")
                };
                quote::quote!(#id)
            }
            SchemaValue::NumericDir(name) => {
                let id = format_ident!("{name}");
                quote::quote!(BTreeMap<i64, #id>)
            }

            SchemaValue::Optional(opt) => {
                let ty = opt.to_rust_type_token();
                quote::quote!(Option<#ty>)
            }
        }
    }
}

fn fmt_type_name(s: &str) -> String {
    let s = s.to_case(Case::Pascal);
    if s.parse::<usize>().is_ok() {
        format!("_{s}")   
    } else {
        s
    }
}

fn fmt_field_name(s: &str) -> String {
    let s = s.to_case(Case::Snake);
    if s.parse::<usize>().is_ok() {
        format!("_{s}")   
    } else {
        s
    }
}

#[derive(Debug)]
pub struct SchemaStruct(HashMap<String, SchemaValue>);

impl SchemaStruct {
    pub fn has_optional(&self) -> bool {
        self.0.values().any(|f| f.is_optional())
    }

    pub fn merge_fields(&mut self, other: SchemaStruct) -> anyhow::Result<()> {
        // Mark keys in current struct as optional
        for (k, v) in self.0.iter_mut() {
            if !other.0.contains_key(k) {
                v.make_optional();
            }
        }

        // Add new keys
        for (k, mut v) in other.0.into_iter() {
            self.0.entry(k).or_insert_with(|| {
                v.make_optional();
                v
            });
        }

        Ok(())
    }

    pub fn from_haxml_dir(dir: &HaXmlDir) -> Self {
        let mut fields = HashMap::new();
        for (key, val) in dir.0.iter() {
            let val = SchemaValue::from_haxml_value(val, Some(key.as_str()));
            fields.insert(key.clone(), val);
        }

        Self(fields)
    }

    pub fn fmt_rust_struct(&self, name: &str, mut w: impl fmt::Write) -> fmt::Result {
        let name = fmt_type_name(name);
        writeln!(w, "#[derive(Debug)]")?;
        writeln!(w, "pub struct {name} {{")?;
        for (name, val) in self.0.iter() {
            let name = fmt_field_name(name);
            let ty = val.to_rust_type();
            writeln!(w, "\tpub {name}: {ty},")?;
        }
        writeln!(w, "}}")?;

        Ok(())
    }

    pub fn impl_try_from_ha_xml(&self, name: &str) -> String {
        fn fmt_name(name: &str) -> impl IdentFragment + ToTokens {
            let name_ident = fmt_field_name(name);
            if ["type", "struct", "move"].iter().any(|&s| s == name_ident) {
                format_ident!("_{}", name_ident)
            } else {
                format_ident!("{}", name_ident)
            }
        }

        let ty_name = fmt_type_name(name);
        let name_ident = format_ident!("{}", ty_name);

        let fields = self.0.iter().map(|(name, v)| {
            let name_ident = fmt_name(name);
            let ty = v.to_rust_type_token();
            quote::quote!(pub #name_ident: #ty)
        });

        let map_fields = self.0.iter().map(|(name, v)| {
            let name_ident = fmt_name(name);

            let val = if v.is_optional() {
                quote::quote!(dir.get_opt_key_mapped(#name)?)
            } else {
                quote::quote!(dir.get_key_mapped(#name)?)
            };
            quote::quote!(#name_ident: #val)
        });

        quote::quote! {
            #[derive(Debug)]
            pub struct #name_ident {
                #(#fields),*
            }

            impl TryFrom<&HaXmlValue> for #name_ident {
                type Error = anyhow::Error;

                fn try_from(value: &HaXmlValue) -> Result<Self, Self::Error> {
                    let dir = value.as_dir()?;
                    Ok(Self {
                        #(#map_fields),*
                    })
                }
            }
        }
        .to_string()
    }
}

#[derive(Debug)]
pub struct Schema {
    schema_structs: HashMap<String, SchemaStruct>,
}

impl Schema {
    pub fn from_multiple_roots_dir<'a>(
        root_name: &str,
        dirs: impl Iterator<Item = &'a HaXmlDir>,
    ) -> Self {
        let mut schema = Self {
            schema_structs: HashMap::new(),
        };

        for dir in dirs {
            schema.process_dir(root_name, dir).unwrap();
        }

        schema
    }

    pub fn from_root_dir(root_name: &str, dir: &HaXmlDir) -> Self {
        let mut schema = Self {
            schema_structs: HashMap::new(),
        };

        schema.process_dir(root_name, dir).unwrap();

        schema
    }

    fn process_num_dir(&mut self, name: &str, dir: &HaXmlNumericDir) -> anyhow::Result<()> {
        //TODO support double-nesting for Footholds for example fh/1/1
        for (_k, v) in dir.0.iter() {
            match v {
                HaXmlValue::Dir(dir) => self.process_dir(name, dir)?,
                HaXmlValue::NumericDir(dir) => self.process_num_dir(name, dir)?,

                _ => (),
            }
        }

        Ok(())
    }
    pub fn process_dir(&mut self, name: &str, dir: &HaXmlDir) -> anyhow::Result<()> {
        let strct = SchemaStruct::from_haxml_dir(dir);
        let name = fmt_type_name(name);

        //Either insert or merge
        if let Some(merge_strct) = self.schema_structs.get_mut(&name) {
            merge_strct.merge_fields(strct)?;
        } else {
            self.schema_structs.insert(name.to_string(), strct);
        }

        // Process all sub structs
        for (k, v) in dir.0.iter() {
            match v {
                HaXmlValue::Dir(dir) => self.process_dir(k, dir)?,
                HaXmlValue::NumericDir(dir) => self.process_num_dir(k, dir)?,
                _ => (),
            }
        }
        Ok(())
    }

    pub fn fmt_rust(&self, mut w: impl fmt::Write) -> fmt::Result {
        writeln!(w, "use std::collections::BTreeMap;")?;
        writeln!(w, "use crate::ha_xml::HaXmlValue;")?;
        writeln!(w, "pub type Seat = i64;")?;

        for (key, val) in self.schema_structs.iter() {
            writeln!(w, "{}", val.impl_try_from_ha_xml(key))?;
        }
        Ok(())
    }
}
