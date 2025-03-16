use std::{env, error::Error, path};

enum ArgsState {
    None,
    Header,
    Idl,
    Py,
    Out,
}

#[derive(Default, Debug)]
pub struct Args {
    pub header_paths: Vec<String>,
    pub idl_path: String,
    pub py_config_path: String,
    pub out_path: String,
}

impl Args {
    fn new() -> Args {
        Args {
            ..Default::default()
        }
    }

    #[inline]
    fn _convert_to_absolute_path(path: &str) -> Result<String, Box<dyn Error>> {
        let ret = path::absolute(path)?
            .to_str()
            .ok_or("Failed to convert to absolute path")?
            .into();
        Ok(ret)
    }

    pub fn parse() -> Result<Args, Box<dyn Error>> {
        let mut args = env::args();
        let mut result = Args::new();
        let mut state = ArgsState::None;

        while let Some(arg) = args.next() {
            match arg.as_str() {
                "-h" | "--header" => {
                    state = ArgsState::Header;
                }
                "-i" | "--idl" => {
                    state = ArgsState::Idl;
                }
                "-p" | "--py" => {
                    state = ArgsState::Py;
                }
                "-o" | "--out" => {
                    state = ArgsState::Out;
                }
                _ => match state {
                    ArgsState::Header => {
                        let path = Args::_convert_to_absolute_path(&arg)?;
                        result.header_paths.push(path);
                    }
                    ArgsState::Idl => {
                        if !result.idl_path.is_empty() {
                            return Err("Got multiple idl file path!".into());
                        }
                        result.idl_path = Args::_convert_to_absolute_path(&arg)?;
                    }
                    ArgsState::Py => {
                        if !result.py_config_path.is_empty() {
                            return Err("Got multiple py config file path!".into());
                        }
                        result.py_config_path = Args::_convert_to_absolute_path(&arg)?;
                    }
                    ArgsState::Out => {
                        if !result.out_path.is_empty() {
                            return Err("Got multiple out file path!".into());
                        }
                        result.out_path = Args::_convert_to_absolute_path(&arg)?;
                    }
                    _ => {
                        continue;
                    }
                },
            }
        }

        if result.header_paths.is_empty() {
            return Err("Didn't get a header file paths!".into());
        } else if result.idl_path.is_empty() {
            return Err("Didn't get a idl file path!".into());
        } else if result.py_config_path.is_empty() {
            return Err("Didn't get a py config file path!".into());
        } else if result.out_path.is_empty() {
            return Err("Didn't get a out file path!".into());
        }

        println!("{:#?}", result);

        Ok(result)
    }

    pub fn help() -> String {
        let mut ret = String::new();
        ret.push_str("Usage: awtk_rust_gen [OPTIONS]\n");
        ret.push_str("Options:\n");
        ret.push_str(
            "  -h, --header <HEADER_PATH 1> ... <HEADER_PATH n> Specify the header file path.\n",
        );
        ret.push_str(
            "  -i, --idl    <IDL_PATH>                          Specify the idl file path.\n",
        );
        ret.push_str(
            "  -p, --py     <PY_CONFIG_PATH>                    Specify the py config file path.\n",
        );
        ret.push_str(
            "  -o, --out    <OUT_PATH>                          Specify the output file path.\n",
        );
        ret
    }
}
