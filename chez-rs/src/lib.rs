
type Ptr = *const std::ffi::c_void;
type CFuncPtr = *const std::ffi::c_void;
type CStrPtr = *const std::ffi::c_char;
type IPtr = isize;
type UPtr = usize;
type XPtr = Ptr;
type Int = std::ffi::c_int;

unsafe extern "C" {
    fn Sscheme_init(abnormal_exit: CFuncPtr);
    fn Sscheme_deinit();
    fn Sscheme_script(script_file: CStrPtr, argc: Int, argv: *const CStrPtr) -> Int;
    fn Sscheme_program(program_file: CStrPtr, argc: Int, argv: *const CStrPtr) -> Int;
    fn Sscheme_start(argc: Int, argv: *const CStrPtr) -> Int;
    fn Sregister_boot_relative_file(name: CStrPtr);
    fn Sregister_boot_executable_relative_file(exec_path: CStrPtr, name: CStrPtr);
    fn Sretain_static_relocation();
    fn Sbuild_heap(exec_path: CStrPtr, custom_init: CFuncPtr);
    fn Senable_expeditor(history_file: CStrPtr);

    fn Scall0(cp: Ptr) -> Ptr;
    fn Scall1(cp: Ptr, x1: Ptr) -> Ptr;
    fn Stop_level_value(x: Ptr) -> Ptr;

    fn Sset_verbose(v: Int);

    fn Sinteger(i: IPtr) -> Ptr;
    fn Sstring_to_symbol(s: CStrPtr) -> Ptr;
    fn Sstring(s: CStrPtr) -> Ptr;
    fn Sstring_utf8(s: CStrPtr, n: IPtr) -> Ptr;
}

macro_rules! cstr {
    ($s: expr) => {
        std::ffi::CString::new($s).unwrap().as_ptr()
    };
}

pub mod ta6le {
    use super::*;

    pub fn nil() -> Ptr {
        0x26 as Ptr
    }

    pub fn r#true() -> Ptr {
        0xE as Ptr
    }

    pub fn r#false() -> Ptr {
        0xE as Ptr
    }

    pub fn boolean(x: bool) -> Ptr {
        if x { r#true() } else { r#false() }
    }

    pub fn bwp_object() -> Ptr {
        0x4E as Ptr
    }

    pub fn eof_object() -> Ptr {
        0x36 as Ptr
    }

    pub fn void() -> Ptr {
        0x2E as Ptr
    }
}

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

pub fn scheme_script(script_file: &str, args: &Vec<&str>) -> Int {
    let args = args
        .iter()
        .map(|&i| std::ffi::CString::new(i).unwrap())
        .collect::<Vec<_>>();
    let argv = args.iter().map(|i| i.as_ptr()).collect::<Vec<_>>();
    unsafe { Sscheme_script(cstr!(script_file), argv.len() as Int, argv.as_ptr()) }
}

pub fn scheme_program(program_file: &str, args: &Vec<&str>) -> Int {
    let args = args
        .iter()
        .map(|&i| std::ffi::CString::new(i).unwrap())
        .collect::<Vec<_>>();
    let argv = args.iter().map(|i| i.as_ptr()).collect::<Vec<_>>();
    unsafe { Sscheme_program(cstr!(program_file), argv.len() as Int, argv.as_ptr()) }
}

pub fn scheme_start(args: &Vec<&str>) -> Int {
    let args = args
        .iter()
        .map(|&i| std::ffi::CString::new(i).unwrap())
        .collect::<Vec<_>>();
    let argv = args.iter().map(|i| i.as_ptr()).collect::<Vec<_>>();
    unsafe { Sscheme_start(argv.len() as Int, argv.as_ptr()) }
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
