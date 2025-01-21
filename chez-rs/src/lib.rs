// pub mod externs;
pub mod types;
pub mod cffi;
pub mod ta6le;
pub mod wrapper;

use cffi::*;

#[macro_export]
macro_rules! cstr {
    ($s: expr) => {
        std::ffi::CString::new($s).unwrap().as_ptr()
    };
}

#[unsafe(no_mangle)]
extern "C" fn RS_free_str(s: CStrPtr) {
    drop(unsafe { std::ffi::CString::from_raw(s as *mut std::ffi::c_char) });
}

// machine-dependent .c files, e.g., x88k.c
#[unsafe(no_mangle)]
extern "C" fn S_machine_init() {}

// self-exe.c
#[unsafe(no_mangle)]
extern "C" fn S_get_process_executable_path(_exec_path: CStrPtr) -> CStrPtr {
    std::ffi::CString::new(std::env::current_exe().unwrap().to_str().unwrap())
        .unwrap()
        .into_raw()
}

// statics.c
// #[unsafe(no_mangle)]
// extern "C" fn scheme_statics() {}

pub use ta6le::*;

pub fn boolean(x: bool) -> Ptr {
    if x { r#true() } else { r#false() }
}

pub use wrapper::*;
