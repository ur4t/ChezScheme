use crate::cstr;

use crate::cffi::*;

use ::std::os::raw::c_int;

pub fn integer(i: iptr) -> ptr {
    unsafe { Sinteger(i) }
}

pub fn string_to_symbol(who: &str) -> ptr {
    unsafe { Sstring_to_symbol(cstr!(who)) }
}

pub fn string(s: &str) -> ptr {
    unsafe { Sstring(cstr!(s)) }
}

pub fn string_utf8(s: &str, n: iptr) -> ptr {
    unsafe { Sstring_utf8(cstr!(s), n) }
}

pub fn call0(who: &str) -> ptr {
    unsafe { Scall0(Stop_level_value(string_to_symbol(who))) }
}

#[allow(clippy::not_unsafe_ptr_arg_deref)]
pub fn call1(who: &str, x1: ptr) -> ptr {
    unsafe { Scall1(Stop_level_value(string_to_symbol(who)), x1) }
}

#[allow(clippy::not_unsafe_ptr_arg_deref)]
pub(crate) fn error1(who: &str, s: &str, x: ptr) {
    unsafe { S_error1(cstr!(who), cstr!(s), x) }
}

pub fn scheme_init() {
    unsafe { Sscheme_init(None) }
}

pub fn scheme_deinit() {
    unsafe { Sscheme_deinit() }
}

pub fn scheme_script(script_file: &str, args: &Vec<&str>) -> c_int {
    let args = args
        .iter()
        .map(|&i| std::ffi::CString::new(i).unwrap())
        .collect::<Vec<_>>();
    let mut argv = args.iter().map(|i| i.as_ptr()).collect::<Vec<_>>();
    unsafe { Sscheme_script(cstr!(script_file), argv.len() as c_int, argv.as_mut_ptr()) }
}

pub fn scheme_program(program_file: &str, args: &Vec<&str>) -> c_int {
    let args = args
        .iter()
        .map(|&i| std::ffi::CString::new(i).unwrap())
        .collect::<Vec<_>>();
    let mut argv = args.iter().map(|i| i.as_ptr()).collect::<Vec<_>>();
    unsafe { Sscheme_program(cstr!(program_file), argv.len() as c_int, argv.as_mut_ptr()) }
}

pub fn scheme_start(args: &Vec<&str>) -> c_int {
    let args = args
        .iter()
        .map(|&i| std::ffi::CString::new(i).unwrap())
        .collect::<Vec<_>>();
    let mut argv = args.iter().map(|i| i.as_ptr()).collect::<Vec<_>>();
    unsafe { Sscheme_start(argv.len() as c_int, argv.as_mut_ptr()) }
}

pub fn set_verbose() {
    unsafe { Sset_verbose(1) }
}

pub fn retain_static_relocation() {
    unsafe { Sretain_static_relocation() };
}

pub fn build_heap(exec_path: &str) {
    unsafe { Sbuild_heap(cstr!(exec_path), None) }
}

pub fn enable_expeditor(history_file: &str) {
    if history_file == "off" {
        unsafe { Senable_expeditor(std::ptr::null()) }
    } else {
        unsafe { Senable_expeditor(cstr!(history_file)) }
    }
}

pub fn register_boot_relative_file(name: &str) {
    unsafe { Sregister_boot_relative_file(cstr!(name)) }
}

pub fn register_boot_executable_relative_file(exec_path: &str, name: &str) {
    unsafe { Sregister_boot_executable_relative_file(cstr!(exec_path), cstr!(name)) }
}
