extern crate rust_graphics_log as log;
use log::log_i;

#[allow(dead_code)]
#[no_mangle]
pub unsafe extern "C" fn rust_graphics_main(_: *mut u32, _: *mut u32, _: isize) {
    log_i!("Hello from rust");
}
