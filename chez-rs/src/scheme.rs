#![allow(dead_code)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]

use crate::types::INT;

macro_rules! cstr {
    ($s: expr) => {
        std::ffi::CString::new($s).unwrap().as_ptr()
    };
}

pub struct Object {
    pointer: ptr,
}

pub mod ta6le {
    use super::*;

    pub fn nil() -> Object {
        Object {
            pointer: 0x26 as ptr,
        }
    }

    pub fn boolean(x: bool) -> Object {
        Object {
            pointer: (if x { 0xE } else { 0x6 }) as ptr,
        }
    }

    pub fn bwp_object() -> Object {
        Object {
            pointer: 0x4E as ptr,
        }
    }

    pub fn eof_object() -> Object {
        Object {
            pointer: 0x36 as ptr,
        }
    }

    pub fn void() -> Object {
        Object {
            pointer: 0x2E as ptr,
        }
    }
}

pub fn integer(i: iptr) -> Object {
    Object {
        pointer: unsafe { Sinteger(i) },
    }
}

pub fn string_to_symbol(who: &str) -> Object {
    Object {
        pointer: unsafe { Sstring_to_symbol(cstr!(who)) },
    }
}

pub fn string(s: &str) -> Object {
    Object {
        pointer: unsafe { Sstring(cstr!(s)) },
    }
}

pub fn string_utf8(s: &str, n: iptr) -> Object {
    Object {
        pointer: unsafe { Sstring_utf8(cstr!(s), n) },
    }
}

pub fn call0(who: &str) -> Object {
    Object {
        pointer: unsafe { Scall0(Stop_level_value(string_to_symbol(who).pointer)) },
    }
}

pub fn call1(who: &str, x1: Object) -> Object {
    Object {
        pointer: unsafe { Scall1(Stop_level_value(string_to_symbol(who).pointer), x1.pointer) },
    }
}

pub fn scheme_init() {
    unsafe { Sscheme_init(None) }
}

pub fn scheme_deinit() {
    unsafe { Sscheme_deinit() }
}

pub fn scheme_script(script_file: &str, args: &[std::ffi::CString]) -> INT {
    let mut argv = args
        .iter()
        .map(|i| i.as_ptr())
        .chain([::core::ptr::null()])
        .collect::<Vec<_>>();
    let argc = args.len() as INT;
    unsafe { Sscheme_script(cstr!(script_file), argc, argv.as_mut_ptr()) }
}

pub fn scheme_program(program_file: &str, args: &[std::ffi::CString]) -> INT {
    let mut argv = args
        .iter()
        .map(|i| i.as_ptr())
        .chain([::core::ptr::null()])
        .collect::<Vec<_>>();
    let argc = args.len() as INT;
    unsafe { Sscheme_program(cstr!(program_file), argc, argv.as_mut_ptr()) }
}

pub fn scheme_start(args: &[std::ffi::CString]) -> INT {
    let mut argv = args
        .iter()
        .map(|i| i.as_ptr())
        .chain([::core::ptr::null()])
        .collect::<Vec<_>>();
    let argc = args.len() as INT;
    unsafe { Sscheme_start(argc, argv.as_mut_ptr()) }
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

pub fn register_boot_executable_relative_file(exec_path: &str, name: &str) {
    unsafe { Sregister_boot_executable_relative_file(cstr!(exec_path), cstr!(name)) }
}

// Chez Scheme Version and machine type
const VERSION: &str = "10.2.0-pre-release.2";
const MACHINE_TYPE: &str = "ta6le";

// Integer typedefs
pub type Sint32_t = i32;
pub type Suint32_t = u32;
pub type Sint64_t = i64;
pub type Suint64_t = u64;

// More integer typedefs
pub type I8 = i8;
pub type U8 = u8;
pub type I16 = i16;
pub type U16 = u16;
pub type I32 = i32;
pub type U32 = u32;
pub type I64 = i64;
pub type U64 = u64;

// All Scheme objects are of type ptr.  Type iptr and
// uptr are signed and unsigned ints of the same size
// as a ptr
pub type ptr = *mut ::core::ffi::c_void;
pub type iptr = isize;
pub type uptr = usize;
pub type xptr = ptr;

// The `uptr` and `ptr` types are the same width, but `ptr`
// can be either an integer type or a pointer type; it may
// be larger than a pointer type.
// Use `TO_VOIDP` to get from the `uptr`/`ptr` world to the
// C pointer worlds, and use `TO_PTR` to go the other way.
// #ifdef PORTABLE_BYTECODE
// # define TO_VOIDP(p) ((void *)(intptr_t)(p))
// # define TO_PTR(p) ((ptr)(intptr_t)(p))
// #else
// # define TO_VOIDP(p) ((void *)(p))
// # define TO_PTR(p) ((ptr)(p))
// #endif

// String elements are 32-bit tagged char objects
pub type string_char = ::core::ffi::c_uint;

// Bytevector elements are 8-bit unsigned "octets"
pub type octet = u8;

// Type predicates
unsafe extern "C" {
    // #define Sfixnump(x) (((uptr)(x)&0x7)==0x0)
    // #define Scharp(x) (((uptr)(x)&0xFF)==0x16)
    // #define Snullp(x) ((uptr)(x)==0x26)
    // #define Seof_objectp(x) ((uptr)(x)==0x36)
    // #define Sbwp_objectp(x) ((uptr)(x)==0x4E)
    // #define Sbooleanp(x) (((uptr)(x)&0xF7)==0x6)
    // #define Spairp(x) (((uptr)(x)&0x7)==0x1)
    // #define Ssymbolp(x) (((uptr)(x)&0x7)==0x3)
    // #define Sprocedurep(x) (((uptr)(x)&0x7)==0x5)
    // #define Sflonump(x) (((uptr)(x)&0x7)==0x2)
    // #define Svectorp(x) ((((uptr)(x)&0x7)==0x7) &&\
    //     (((uptr)((*((ptr *)TO_VOIDP((uptr)(x)+1))))&0x7)==0x0))
    // #define Sfxvectorp(x) ((((uptr)(x)&0x7)==0x7) &&\
    //     (((uptr)((*((ptr *)TO_VOIDP((uptr)(x)+1))))&0xF)==0x3))
    // #define Sflvectorp(x) ((((uptr)(x)&0x7)==0x7) &&\
    //     (((uptr)((*((ptr *)TO_VOIDP((uptr)(x)+1))))&0xF)==0xB))
    // #define Sbytevectorp(x) ((((uptr)(x)&0x7)==0x7) &&\
    //     (((uptr)((*((ptr *)TO_VOIDP((uptr)(x)+1))))&0x3)==0x1))
    // #define Sstringp(x) ((((uptr)(x)&0x7)==0x7) &&\
    //     (((uptr)((*((ptr *)TO_VOIDP((uptr)(x)+1))))&0x7)==0x2))
    // #define Sstencil_vectorp(x) ((((uptr)(x)&0x7)==0x7) &&\
    //     (((uptr)((*((ptr *)TO_VOIDP((uptr)(x)+1))))&0x3F)==0xE))
    // #define Ssystem_stencil_vectorp(x) ((((uptr)(x)&0x7)==0x7) &&\
    //     (((uptr)((*((ptr *)TO_VOIDP((uptr)(x)+1))))&0x3F)==0x2E))
    // #define Sany_stencil_vectorp(x) ((((uptr)(x)&0x7)==0x7) &&\
    //     (((uptr)((*((ptr *)TO_VOIDP((uptr)(x)+1))))&0x1F)==0xE))
    // #define Sbignump(x) ((((uptr)(x)&0x7)==0x7) &&\
    //     (((uptr)((*((ptr *)TO_VOIDP((uptr)(x)+1))))&0x1F)==0x6))
    // #define Sboxp(x) ((((uptr)(x)&0x7)==0x7) &&\
    //     (((uptr)((*((ptr *)TO_VOIDP((uptr)(x)+1))))&0xFF)==0x1E))
    // #define Sinexactnump(x) ((((uptr)(x)&0x7)==0x7) &&\
    //     ((uptr)((*((ptr *)TO_VOIDP((uptr)(x)+1))))==0x36))
    // #define Sexactnump(x) ((((uptr)(x)&0x7)==0x7) &&\
    //     ((uptr)((*((ptr *)TO_VOIDP((uptr)(x)+1))))==0x56))
    // #define Sratnump(x) ((((uptr)(x)&0x7)==0x7) &&\
    //     ((uptr)((*((ptr *)TO_VOIDP((uptr)(x)+1))))==0x16))
    // #define Sinputportp(x) ((((uptr)(x)&0x7)==0x7) &&\
    //     (((uptr)((*((ptr *)TO_VOIDP((uptr)(x)+1))))&0x1FF)==0x1DE))
    // #define Soutputportp(x) ((((uptr)(x)&0x7)==0x7) &&\
    //     (((uptr)((*((ptr *)TO_VOIDP((uptr)(x)+1))))&0x2FF)==0x2DE))
    // #define Srecordp(x) ((((uptr)(x)&0x7)==0x7) &&\
    //     (((uptr)((*((ptr *)TO_VOIDP((uptr)(x)+1))))&0x7)==0x7))
}

// Accessors
unsafe extern "C" {
    // #define Sfixnum_value(x) ((iptr)(x)/8)
    // #define Schar_value(x) ((string_char)((uptr)(x)>>8))
    // #define Sboolean_value(x) ((x) != Sfalse)
    // #define Scar(x) (*((ptr *)TO_VOIDP((uptr)(x)+7)))
    // #define Scdr(x) (*((ptr *)TO_VOIDP((uptr)(x)+15)))
    // #define Sflonum_value(x) (*((double *)TO_VOIDP((uptr)(x)+6)))
    // #define Svector_length(x) ((iptr)((uptr)(*((iptr *)TO_VOIDP((uptr)(x)+1)))>>4))
    // #define Svector_ref(x,i) (((ptr *)TO_VOIDP((uptr)(x)+9))[i])
    // #define Sfxvector_length(x) ((iptr)((uptr)(*((iptr *)TO_VOIDP((uptr)(x)+1)))>>4))
    // #define Sfxvector_ref(x,i) (((ptr *)TO_VOIDP((uptr)(x)+9))[i])
    // #define Sflvector_length(x) ((iptr)((uptr)(*((iptr *)TO_VOIDP((uptr)(x)+1)))>>4))
    // #define Sflvector_ref(x,i) (((double *)TO_VOIDP((uptr)(x)+9))[i])
    // #define Sbytevector_length(x) ((iptr)((uptr)(*((iptr *)TO_VOIDP((uptr)(x)+1)))>>3))
    // #define Sbytevector_u8_ref(x,i) (((octet *)TO_VOIDP((uptr)(x)+9))[i])
    // /* Warning: Sbytevector_data(x) returns a pointer into x. */
    // #define Sbytevector_data(x) &Sbytevector_u8_ref(x,0)
    // #define Sstring_length(x) ((iptr)((uptr)(*((iptr *)TO_VOIDP((uptr)(x)+1)))>>4))
    // #define Sstring_ref(x,i) Schar_value(((string_char *)TO_VOIDP((uptr)(x)+9))[i])
    // #define Sunbox(x) (*((ptr *)TO_VOIDP((uptr)(x)+9)))
    // #define Sstencil_vector_length(x) Spopcount(((uptr)(*((iptr *)TO_VOIDP((uptr)(x)+1))))>>6)
    // #define Sstencil_vector_ref(x,i) (((ptr *)TO_VOIDP((uptr)(x)+9))[i])
    // #define Sunsigned_value(x) (uptr)Sinteger_value(x)
    // #define Sunsigned32_value(x) (Suint32_t)Sinteger32_value(x)
    // #define Sunsigned64_value(x) (Suint64_t)Sinteger64_value(x)
    pub fn Sinteger_value(arg1: ptr) -> iptr;
    pub fn Sinteger32_value(arg1: ptr) -> Sint32_t;
    pub fn Sinteger64_value(arg1: ptr) -> Sint64_t;
    pub fn Stry_integer_value(
        arg1: ptr,
        arg2: *mut iptr,
        arg3: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    pub fn Stry_integer32_value(
        arg1: ptr,
        arg2: *mut Sint32_t,
        arg3: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    pub fn Stry_integer64_value(
        arg1: ptr,
        arg2: *mut Sint64_t,
        arg3: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    pub fn Stry_unsigned_value(
        arg1: ptr,
        arg2: *mut uptr,
        arg3: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    pub fn Stry_unsigned32_value(
        arg1: ptr,
        arg2: *mut Suint32_t,
        arg3: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    pub fn Stry_unsigned64_value(
        arg1: ptr,
        arg2: *mut Suint64_t,
        arg3: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
}

// Mutators
unsafe extern "C" {
    pub fn Sset_box(arg1: ptr, arg2: ptr);
    pub fn Sset_car(arg1: ptr, arg2: ptr);
    pub fn Sset_cdr(arg1: ptr, arg2: ptr);
    pub fn Svector_set(arg1: ptr, arg2: iptr, arg3: ptr);
}

// Constructors
unsafe extern "C" {
    // #define Sfixnum(x) ((ptr)(uptr)((x)*8))
    // #define Schar(x) ((ptr)(uptr)((x)<<8|0x16))
    // #define Snil ((ptr)0x26)
    // #define Strue ((ptr)0xE)
    // #define Sfalse ((ptr)0x6)
    // #define Sboolean(x) ((x)?Strue:Sfalse)
    // #define Sbwp_object ((ptr)0x4E)
    // #define Seof_object ((ptr)0x36)
    // #define Svoid ((ptr)0x2E)
    pub fn Scons(arg1: ptr, arg2: ptr) -> ptr;
    pub fn Sstring_to_symbol(arg1: *const ::core::ffi::c_char) -> ptr;
    pub fn Ssymbol_to_string(arg1: ptr) -> ptr;
    pub fn Sflonum(arg1: f64) -> ptr;
    pub fn Smake_vector(arg1: iptr, arg2: ptr) -> ptr;
    pub fn Smake_fxvector(arg1: iptr, arg2: ptr) -> ptr;
    pub fn Smake_flvector(arg1: iptr, arg2: f64) -> ptr;
    pub fn Smake_bytevector(arg1: iptr, arg2: ::core::ffi::c_int) -> ptr;
    pub fn Smake_string(arg1: iptr, arg2: ::core::ffi::c_int) -> ptr;
    pub fn Smake_uninitialized_string(arg1: iptr) -> ptr;
    pub fn Sstring(arg1: *const ::core::ffi::c_char) -> ptr;
    pub fn Sstring_of_length(arg1: *const ::core::ffi::c_char, arg2: iptr) -> ptr;
    pub fn Sstring_utf8(arg1: *const ::core::ffi::c_char, arg2: iptr) -> ptr;
    pub fn Sbox(arg1: ptr) -> ptr;
    pub fn Sinteger(arg1: iptr) -> ptr;
    pub fn Sunsigned(arg1: uptr) -> ptr;
    pub fn Sinteger32(arg1: Sint32_t) -> ptr;
    pub fn Sunsigned32(arg1: Suint32_t) -> ptr;
    pub fn Sinteger64(arg1: Sint64_t) -> ptr;
    pub fn Sunsigned64(arg1: Suint64_t) -> ptr;
}

// Records
unsafe extern "C" {
    // #define Srecord_uniform_ref(x,i) (((ptr *)TO_VOIDP((uptr)(x)+9))[i])
    pub fn Srecord_type(arg1: ptr) -> ptr;
    pub fn Srecord_type_parent(arg1: ptr) -> ptr;
    pub fn Srecord_type_uniformp(arg1: ptr) -> ::core::ffi::c_int;
}

// Miscellaneous
unsafe extern "C" {
    pub fn Srecord_type_size(arg1: ptr) -> uptr;
    pub fn Stop_level_value(arg1: ptr) -> ptr;
    pub fn Sset_top_level_value(arg1: ptr, arg2: ptr);
    pub fn Slock_object(arg1: ptr);
    pub fn Sunlock_object(arg1: ptr);
    pub fn Slocked_objectp(arg1: ptr) -> ::core::ffi::c_int;
    pub fn Sforeign_symbol(arg1: *const ::core::ffi::c_char, arg2: *mut ::core::ffi::c_void);
    pub fn Sregister_symbol(arg1: *const ::core::ffi::c_char, arg2: *mut ::core::ffi::c_void);
}

// Calls into Scheme
unsafe extern "C" {
    pub fn Scall0(arg1: ptr) -> ptr;
    pub fn Scall1(arg1: ptr, arg2: ptr) -> ptr;
    pub fn Scall2(arg1: ptr, arg2: ptr, arg3: ptr) -> ptr;
    pub fn Scall3(arg1: ptr, arg2: ptr, arg3: ptr, arg4: ptr) -> ptr;
    pub fn Sinitframe(arg1: iptr);
    pub fn Sput_arg(arg1: iptr, arg2: ptr);
    pub fn Scall(arg1: ptr, arg2: iptr) -> ptr;
    // /* Warning: Sforeign_callable_entry_point(x) returns a pointer into x. */
    // #define Sforeign_callable_entry_point(x) ((void (*)(void))TO_VOIDP((uptr)(x)+65))
    // #define Sforeign_callable_code_object(x) ((ptr)TO_VOIDP((uptr)(x)-65))
}

// Customization support
unsafe extern "C" {
    pub fn Skernel_version() -> *const ::core::ffi::c_char;
    pub fn Sretain_static_relocation();
    pub fn Sset_verbose(arg1: ::core::ffi::c_int);
    pub fn Sscheme_init(arg1: ::core::option::Option<unsafe extern "C" fn()>);
    pub fn Sregister_boot_file(arg1: *const ::core::ffi::c_char);
    pub fn Sregister_boot_executable_relative_file(
        arg1: *const ::core::ffi::c_char,
        arg2: *const ::core::ffi::c_char,
    );
    pub fn Sregister_boot_relative_file(arg1: *const ::core::ffi::c_char);
    pub fn Sregister_boot_file_fd(arg1: *const ::core::ffi::c_char, arg2: ::core::ffi::c_int);
    pub fn Sregister_boot_file_fd_region(
        arg1: *const ::core::ffi::c_char,
        arg2: ::core::ffi::c_int,
        arg3: iptr,
        arg4: iptr,
        arg5: ::core::ffi::c_int,
    );
    pub fn Sregister_boot_file_bytes(
        arg1: *const ::core::ffi::c_char,
        arg2: *mut ::core::ffi::c_void,
        arg3: iptr,
    );
    pub fn Sregister_heap_file(arg1: *const ::core::ffi::c_char);
    pub fn Scompact_heap();
    pub fn Ssave_heap(arg1: *const ::core::ffi::c_char, arg2: ::core::ffi::c_int);
    pub fn Sbuild_heap(
        arg1: *const ::core::ffi::c_char,
        arg2: ::core::option::Option<unsafe extern "C" fn()>,
    );
    pub fn Senable_expeditor(arg1: *const ::core::ffi::c_char);
    pub fn Sscheme_start(
        arg1: ::core::ffi::c_int,
        arg2: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    pub fn Sscheme_script(
        arg1: *const ::core::ffi::c_char,
        arg2: ::core::ffi::c_int,
        arg3: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    pub fn Sscheme_program(
        arg1: *const ::core::ffi::c_char,
        arg2: ::core::ffi::c_int,
        arg3: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    pub fn Sscheme_deinit();
    pub fn Sscheme_register_signal_registerer(
        f: ::core::option::Option<unsafe extern "C" fn(arg1: ::core::ffi::c_int)>,
    );
}

// Thread support
unsafe extern "C" {
    pub fn Sactivate_thread() -> ::core::ffi::c_int;
    pub fn Sdeactivate_thread();
    pub fn Sdestroy_thread() -> ::core::ffi::c_int;
}
