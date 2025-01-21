use crate::{Ptr, IPtr, cstr};

use crate::cffi::*;

pub fn integer(i: IPtr) -> Ptr {
    unsafe { Sinteger(i) }
}

pub fn string_to_symbol(who: &str) -> Ptr {
    unsafe { Sstring_to_symbol(cstr!(who)) }
}

pub fn string(s: &str) -> Ptr {
    unsafe { Sstring(cstr!(s)) }
}

pub fn string_utf8(s: &str, n: IPtr) -> Ptr {
    unsafe { Sstring_utf8(cstr!(s), n) }
}

pub fn call0(who: &str) -> Ptr {
    unsafe { Scall0(Stop_level_value(string_to_symbol(who))) }
}

#[allow(clippy::not_unsafe_ptr_arg_deref)]
pub fn call1(who: &str, x1: Ptr) -> Ptr {
    unsafe { Scall1(Stop_level_value(string_to_symbol(who)), x1) }
}

pub fn scheme_init() {
    unsafe { Sscheme_init(std::ptr::null()) }
}

pub fn scheme_deinit() {
    unsafe { Sscheme_deinit() }
}

pub fn scheme_script(script_file: &str, args: &Vec<&str>) -> CInt {
    let args = args
        .iter()
        .map(|&i| std::ffi::CString::new(i).unwrap())
        .collect::<Vec<_>>();
    let argv = args.iter().map(|i| i.as_ptr()).collect::<Vec<_>>();
    unsafe { Sscheme_script(cstr!(script_file), argv.len() as CInt, argv.as_ptr()) }
}

pub fn scheme_program(program_file: &str, args: &Vec<&str>) -> CInt {
    let args = args
        .iter()
        .map(|&i| std::ffi::CString::new(i).unwrap())
        .collect::<Vec<_>>();
    let argv = args.iter().map(|i| i.as_ptr()).collect::<Vec<_>>();
    unsafe { Sscheme_program(cstr!(program_file), argv.len() as CInt, argv.as_ptr()) }
}

pub fn scheme_start(args: &Vec<&str>) -> CInt {
    let args = args
        .iter()
        .map(|&i| std::ffi::CString::new(i).unwrap())
        .collect::<Vec<_>>();
    let argv = args.iter().map(|i| i.as_ptr()).collect::<Vec<_>>();
    unsafe { Sscheme_start(argv.len() as CInt, argv.as_ptr()) }
}

pub fn set_verbose() {
    unsafe { Sset_verbose(1) }
}

pub fn retain_static_relocation() {
    unsafe { Sretain_static_relocation() };
}

pub fn build_heap(exec_path: &str) {
    unsafe { Sbuild_heap(cstr!(exec_path), std::ptr::null()) }
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
