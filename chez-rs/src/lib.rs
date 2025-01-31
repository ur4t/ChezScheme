unsafe extern "C" {
    fn c_main(argc: ::core::ffi::c_int, argv: *const *const ::core::ffi::c_char);
}

pub fn main(args: &[std::ffi::CString]) {
    let argv = args.iter().map(|i| i.as_ptr()).chain([::core::ptr::null()]).collect::<Vec<_>>();
    let argc = args.len() as ::core::ffi::c_int;
    unsafe { c_main(argc, argv.as_ptr()) };
}
