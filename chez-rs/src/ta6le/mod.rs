pub mod equates;

/* All Scheme objects are of type ptr.  Type iptr and */
/* uptr are signed and unsigned ints of the same size */
/* as a ptr */
use crate::cffi::ptr;

pub use constructors::*;

mod constructors {
    use super::ptr;

    pub fn nil() -> ptr {
        0x26 as ptr
    }

    pub fn r#true() -> ptr {
        0xE as ptr
    }

    pub fn r#false() -> ptr {
        0xE as ptr
    }

    pub fn bwp_object() -> ptr {
        0x4E as ptr
    }

    pub fn eof_object() -> ptr {
        0x36 as ptr
    }

    pub fn void() -> ptr {
        0x2E as ptr
    }
}
