use std::{collections::HashMap, error::Error};

#[derive(Default)]
pub struct Idl {
    /* key是类名，value是方法名数组 */
    pub classes: HashMap<String, Vec<String>>,
}

impl Idl {
    fn new() -> Idl {
        Idl {
            ..Default::default()
        }
    }

    pub fn parser(idl: &str) -> Result<Idl, Box<dyn Error>> {
        let mut result: Idl = Idl::new();
        let parsed = json::parse(idl)?;

        if let json::JsonValue::Array(arr) = parsed {
            arr.iter().for_each(|item| {
                if let json::JsonValue::Object(object) = item {
                    match (
                        object.get("type").and_then(|t| t.as_str()),
                        object.get("name").and_then(|n| n.as_str()),
                        object.get("methods"),
                    ) {
                        (Some("class"), Some(name), Some(json::JsonValue::Array(methods))) => {
                            let arr: Vec<String> = methods
                                .iter()
                                .filter_map(|method| {
                                    if let json::JsonValue::Object(m) = method {
                                        m["name"].as_str().map(String::from)
                                    } else {
                                        None
                                    }
                                })
                                .collect();

                            result.classes.insert(name.to_string(), arr);
                        }
                        _ => {}
                    }
                }
            });
        }

        Ok(result)
    }
}
