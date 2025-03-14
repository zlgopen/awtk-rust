use std::{env, error::Error};

#[derive(Default)]
pub struct Args {
    pub header_path: String,
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

    pub fn parser() -> Result<Args, Box<dyn Error>> {
        let mut args = env::args();
        let mut result: Args = Args::new();

        while let Some(arg) = args.next() {
            match arg.as_str() {
                "-h" | "--header" => {
                    result.header_path = args.next().unwrap();
                }
                "-i" | "--idl" => {
                    result.idl_path = args.next().unwrap();
                }
                "-p" | "--py" => {
                    result.py_config_path = args.next().unwrap();
                }
                "-o" | "--out" => {
                    result.out_path = args.next().unwrap();
                }
                _ => continue,
            }
        }

        if result.header_path.is_empty() {
            return Err("Didn't get a header file path!".into());
        } else if result.idl_path.is_empty() {
            return Err("Didn't get a idl file path!".into());
        } else if result.py_config_path.is_empty() {
            return Err("Didn't get a py config file path!".into());
        } else if result.out_path.is_empty() {
            return Err("Didn't get a out file path!".into());
        }

        Ok(result)
    }

    pub fn help() -> String {
        let mut ret = String::new();
        ret.push_str("Usage: awtk_rust_gen [OPTIONS]\n");
        ret.push_str("Options:\n");
        ret.push_str("  -h, --header <HEADER_PATH>     Specify the header file path.\n");
        ret.push_str("  -i, --idl    <IDL_PATH>        Specify the idl file path.\n");
        ret.push_str("  -p, --py     <PY_CONFIG_PATH>  Specify the py config file path.\n");
        ret.push_str("  -o, --out    <OUT_PATH>        Specify the output file path.\n");
        ret
    }
}
