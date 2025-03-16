use clap::{CommandFactory, FromArgMatches, Parser};
use std::{error::Error, path};

#[derive(Parser, Debug)]
#[command(
    author = env!("CARGO_PKG_AUTHORS"),
    version = env!("CARGO_PKG_VERSION"),
    name = env!("CARGO_PKG_NAME"),
    about = env!("CARGO_PKG_DESCRIPTION"),
    help_template = "{about-section}Version: {version}\n\nUsage: {name} [OPTIONS]\n\n{all-args}"
)]
pub struct Args {
    #[arg(short='H', long="header", required=true, num_args=1..,help = "Specify the header file path.")]
    pub header_paths: Vec<String>,

    #[arg(
        short = 'i',
        long = "idl",
        required = true,
        help = "Specify the idl file path."
    )]
    pub idl_path: String,

    #[arg(
        short = 'p',
        long = "py",
        required = true,
        help = "Specify the py config file path."
    )]
    pub py_config_path: String,

    #[arg(
        short = 'o',
        long = "output",
        required = true,
        help = "Specify the output file path."
    )]
    pub out_path: String,
}

impl Args {
    pub fn parse() -> Result<Self, Box<dyn Error>> {
        let mut matches = Self::command().get_matches();
        let mut args = Self::from_arg_matches_mut(&mut matches)?;

        args.header_paths = args
            .header_paths
            .iter()
            .map(|p| Self::_convert_to_absolute_path(p))
            .collect::<Result<_, _>>()?;

        args.idl_path = Self::_convert_to_absolute_path(&args.idl_path)?;
        args.py_config_path = Self::_convert_to_absolute_path(&args.py_config_path)?;
        args.out_path = Self::_convert_to_absolute_path(&args.out_path)?;

        println!("{:#?}", args);

        Ok(args)
    }

    #[inline]
    fn _convert_to_absolute_path(path: &str) -> Result<String, Box<dyn Error>> {
        let ret = path::absolute(path)?
            .to_str()
            .ok_or("Failed to convert to absolute path")?
            .into();
        Ok(ret)
    }
}
