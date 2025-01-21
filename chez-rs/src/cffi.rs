pub type CFuncPtr = *const std::ffi::c_void;
pub type CStrPtr = *const std::ffi::c_char;
pub type CInt = std::ffi::c_int;

use crate::{IPtr, Ptr};

unsafe extern "C" {
    pub fn Sscheme_init(abnormal_exit: CFuncPtr);
    pub fn Sscheme_deinit();
    pub fn Sscheme_script(script_file: CStrPtr, argc: CInt, argv: *const CStrPtr) -> CInt;
    pub fn Sscheme_program(program_file: CStrPtr, argc: CInt, argv: *const CStrPtr) -> CInt;
    pub fn Sscheme_start(argc: CInt, argv: *const CStrPtr) -> CInt;
    pub fn Sregister_boot_relative_file(name: CStrPtr);
    pub fn Sregister_boot_executable_relative_file(exec_path: CStrPtr, name: CStrPtr);
    pub fn Sretain_static_relocation();
    pub fn Sbuild_heap(exec_path: CStrPtr, custom_init: CFuncPtr);
    pub fn Senable_expeditor(history_file: CStrPtr);

    pub fn Scall0(cp: Ptr) -> Ptr;
    pub fn Scall1(cp: Ptr, x1: Ptr) -> Ptr;
    pub fn Stop_level_value(x: Ptr) -> Ptr;

    pub fn Sset_verbose(v: CInt);

    pub fn Sinteger(i: IPtr) -> Ptr;
    pub fn Sstring_to_symbol(s: CStrPtr) -> Ptr;
    pub fn Sstring(s: CStrPtr) -> Ptr;
    pub fn Sstring_utf8(s: CStrPtr, n: IPtr) -> Ptr;
}
