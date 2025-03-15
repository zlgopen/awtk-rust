use crate::{awtk::*, helper};

pub fn init() -> Ret {
    unsafe {
        window_open(helper::c_ptr("home_page"));
        Ret::Ok
    }
}

pub fn exit() -> Ret {
    unsafe {
        println!("application_exit\n");
        tk_mem_dump();
        Ret::Ok
    }
}
