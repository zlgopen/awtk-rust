use crate::{awtk::*, helper};

pub fn init() -> ret_t {
    unsafe {
        window_open(helper::c_ptr("home_page"));
        ret_t::RET_OK
    }
}

pub fn exit() -> ret_t {
    unsafe {
        println!("application_exit\n");
        tk_mem_dump();
        ret_t::RET_OK
    }
}
