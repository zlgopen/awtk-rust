use hello_rust::{application, awtk::*, helper};
use std::{env, ptr::null};

fn main() {
    let args: Vec<String> = env::args().collect();
    let w = if args.len() >= 2 {
        args[1].parse::<i32>().unwrap()
    } else {
        320
    };
    let h = if args.len() >= 3 {
        args[2].parse::<i32>().unwrap()
    } else {
        480
    };
    unsafe {
        tk_init(
            w,
            h,
            AppType::Simulator,
            helper::c_ptr(env!("CARGO_PKG_NAME")),
            null() as _,
        );

        tk_init_assets();
        tk_ext_widgets_init();

        application::init();
        tk_run();
        application::exit();
        tk_exit();
    }
}
