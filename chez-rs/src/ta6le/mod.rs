pub mod equates;

/* All Scheme objects are of type ptr.  Type iptr and */
/* uptr are signed and unsigned ints of the same size */
/* as a ptr */
pub type Ptr = *const std::ffi::c_void;
pub type IPtr = isize;
pub type UPtr = usize;
pub type XPtr = Ptr;

pub use constructors::*;

mod constructors {
    use super::Ptr;

    pub fn nil() -> Ptr {
        0x26 as Ptr
    }

    pub fn r#true() -> Ptr {
        0xE as Ptr
    }

    pub fn r#false() -> Ptr {
        0xE as Ptr
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
