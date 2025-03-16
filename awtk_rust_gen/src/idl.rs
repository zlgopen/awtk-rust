use std::{collections::HashMap, error::Error};

#[derive(Default, Debug, Clone)]
pub struct IdlMethodAnnotation {
    pub static_: bool,
    pub constructor: bool,
    pub deconstructor: bool,
    pub gc: bool,
}

#[derive(Default, Debug, Clone)]
pub struct IdlMethod {
    pub name: String,
    pub annotation: IdlMethodAnnotation,
}

#[derive(Default, Debug, Clone)]
pub struct IdlClass {
    pub name: String,
    pub parent: String,
    pub methods: Vec<IdlMethod>,
}

#[derive(Default, Debug, Clone)]
pub struct IdlEnum {
    pub name: String,
    pub prefix: String,
}

#[derive(Default, Debug, Clone)]
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
        let mut result: Idl = Idl::new();
        let parsed = json::parse(idl)?;

        if let json::JsonValue::Array(arr) = parsed {
            arr.iter().for_each(|item| {
                if let json::JsonValue::Object(object) = item {
                    match (
                        object.get("type").and_then(|t| t.as_str()),
                        object.get("name").and_then(|n| n.as_str()),
                    ) {
                        (Some(type_), Some(name)) => {
                            if type_ == "class" {
                                let mut class = IdlClass {
                                    name: name.into(),
                                    ..Default::default()
                                };

                                if let Some(parent) = object.get("parent") {
                                    class.parent = parent.to_string();
                                }

                                if let Some(json::JsonValue::Array(methods)) = object.get("methods")
                                {
                                    class.methods = methods
                                        .iter()
                                        .filter_map(|mth| {
                                            if let json::JsonValue::Object(m) = mth {
                                                let mut method = IdlMethod {
                                                    ..Default::default()
                                                };

                                                if let Some(name) = m["name"].as_str() {
                                                    method.name = name.into()
                                                }

                                                if let Some(json::JsonValue::Object(annotation)) =
                                                    m.get("annotation")
                                                {
                                                    if let Some(static_) =
                                                        annotation["static"].as_bool()
                                                    {
                                                        method.annotation.static_ = static_;
                                                    }
                                                    if let Some(constructor) =
                                                        annotation["constructor"].as_bool()
                                                    {
                                                        method.annotation.constructor = constructor;
                                                    }
                                                    if let Some(deconstructor) =
                                                        annotation["deconstructor"].as_bool()
                                                    {
                                                        method.annotation.deconstructor =
                                                            deconstructor;
                                                    }
                                                    if let Some(gc) = annotation["gc"].as_bool() {
                                                        method.annotation.gc = gc;
                                                    }
                                                }

                                                Some(method)
                                            } else {
                                                None
                                            }
                                        })
                                        .collect();
                                }

                                result.classes.insert(name.into(), class);
                            } else if type_ == "enum" {
                                let mut enum_ = IdlEnum {
                                    name: name.into(),
                                    ..Default::default()
                                };

                                if let Some(prefix) = object.get("prefix") {
                                    enum_.prefix = prefix.to_string();
                                }

                                result.enums.insert(name.into(), enum_);
                            }
                        }
                        _ => {}
                    }
                }
            });
        }

        Ok(result)
    }
}
