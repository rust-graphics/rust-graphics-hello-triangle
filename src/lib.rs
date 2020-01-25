mod main;

#[allow(dead_code)]
#[no_mangle]
pub unsafe extern "C" fn rust_graphics_main(_: *mut u32, _: *mut u32, _: isize) {
    main::main();
}
