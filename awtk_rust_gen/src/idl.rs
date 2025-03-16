use serde::Deserialize;
use std::{collections::HashMap, error::Error};

#[derive(Default, Debug, Clone, Deserialize)]
pub struct IdlMethodAnnotation {
    #[serde(default, rename = "static")]
    pub static_: bool,

    #[serde(default)]
    pub constructor: bool,

    #[serde(default)]
    pub deconstructor: bool,

    #[serde(default)]
    pub gc: bool,
}

#[derive(Default, Debug, Clone, Deserialize)]
pub struct IdlMethod {
    pub name: String,

    #[serde(default)]
    pub annotation: IdlMethodAnnotation,
}

#[derive(Default, Debug, Clone, Deserialize)]
pub struct IdlClass {
    pub name: String,

    #[serde(default)]
    pub parent: String,

    #[serde(default)]
    pub methods: Vec<IdlMethod>,
}

#[derive(Default, Debug, Clone, Deserialize)]
pub struct IdlEnum {
    pub name: String,

    #[serde(default)]
    pub prefix: String,
}

#[derive(Default, Debug, Clone, Deserialize)]
pub struct Idl {
    /* key是类名，value是 IdlClass 对象 */
    pub classes: HashMap<String, IdlClass>,

    /* key是枚举名，value是 IdlEnum 对象 */
    pub enums: HashMap<String, IdlEnum>,
}

impl Idl {
    fn new() -> Self {
        Idl {
            ..Default::default()
        }
    }

    pub fn parse(idl: &str) -> Result<Self, Box<dyn Error>> {
        let mut result = Idl::new();
        let items: Vec<serde_json::Value> = serde_json::from_str(idl)?;

        for item in items {
            if let Some(type_) = item.get("type").and_then(|v| v.as_str()) {
                match type_ {
                    "class" => {
                        let class: IdlClass = serde_json::from_value(item)?;
                        result.classes.insert(class.name.clone(), class);
                    }
                    "enum" => {
                        let enum_: IdlEnum = serde_json::from_value(item)?;
                        result.enums.insert(enum_.name.clone(), enum_);
                    }
                    _ => {}
                }
            }
        }

        Ok(result)
    }
}
