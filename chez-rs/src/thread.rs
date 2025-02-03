#![allow(non_camel_case_types)]

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pthread_mutex_t {
    pub __u: pthread_mutex_t__bindgen_ty_1,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union pthread_mutex_t__bindgen_ty_1 {
    pub __i: [::core::ffi::c_int; 10usize],
    pub __vi: [::core::ffi::c_int; 10usize],
    pub __p: [*mut ::core::ffi::c_void; 5usize],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pthread_cond_t {
    pub __u: pthread_cond_t__bindgen_ty_1,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union pthread_cond_t__bindgen_ty_1 {
    pub __i: [::core::ffi::c_int; 12usize],
    pub __vi: [::core::ffi::c_int; 12usize],
    pub __p: [*mut ::core::ffi::c_void; 6usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __pthread {
    _unused: [u8; 0],
}

pub type pthread_t = *mut __pthread;
pub type pthread_key_t = ::core::ffi::c_uint;

pub type s_thread_t = pthread_t;
pub type s_thread_key_t = pthread_key_t;
pub type s_thread_mutex_t = pthread_mutex_t;
pub type s_thread_cond_t = pthread_cond_t;
