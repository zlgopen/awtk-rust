use crate::{awtk::*, helper};

pub fn init() -> Ret {
    unsafe {
        window_open(helper::c_ptr("home_page"));
    }
    Ret::Ok
}

pub fn exit() -> Ret {
    println!("application_exit\n");
    unsafe {
        tk_mem_dump();
    }
    Ret::Ok
}
