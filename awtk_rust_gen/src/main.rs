use awtk_rust_gen::{args::Args, builder::Builder, idl::Idl};
use std::{fs, process};

fn main() {
    let args = Args::parse().unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}!");
        process::exit(1);
    });

    let idl_json = fs::read_to_string(&args.idl_path).unwrap_or_else(|err| {
        println!("Problem reading idl file\"{}\":{err}", args.idl_path);
        process::exit(2);
    });

    let idl = Idl::parse(&idl_json).unwrap_or_else(|err| {
        println!("Problem parsing idl: {err}!");
        process::exit(3);
    });

    Builder::build(&args, &idl).unwrap_or_else(|err| {
        println!("Problem building: {err}!");
        process::exit(4);
    });

    println!(
        "{} : Generate \"{}\" success!",
        env!("CARGO_PKG_NAME"),
        args.out_path
    );
}
