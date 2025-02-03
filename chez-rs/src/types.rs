#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(clippy::upper_case_acronyms)]
#![allow(clippy::useless_transmute)]

use crate::scheme::{iptr, octet, ptr, uptr};
use crate::thread::{s_thread_mutex_t, s_thread_t};

pub type INT = ::core::ffi::c_int; // honest-to-goodness C int
pub type UINT = ::core::ffi::c_uint; // honest-to-goodness C unsigned int
pub type ITYPE = ::core::ffi::c_int; // ptr types
pub type ISPC = ::core::ffi::c_int; // storage manager spaces
pub type IGEN = ::core::ffi::c_int; // storage manager generations
pub type IDIRTYBYTE = ::core::ffi::c_int; // storage manager dirty bytes
pub type IBOOL = ::core::ffi::c_int; // int used exclusively as a boolean
pub type ICHAR = ::core::ffi::c_int; // int used exclusively as a character
pub type IFASLCODE = ::core::ffi::c_int; // fasl type codes

#[repr(C)]
#[derive(Copy, Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct __BindgenBitfieldUnit<Storage> {
    storage: Storage,
}

impl<Storage> __BindgenBitfieldUnit<Storage> {
    #[inline]
    pub const fn new(storage: Storage) -> Self {
        Self { storage }
    }
}

impl<Storage> __BindgenBitfieldUnit<Storage>
where
    Storage: AsRef<[u8]> + AsMut<[u8]>,
{
    #[inline]
    fn extract_bit(byte: u8, index: usize) -> bool {
        let bit_index = if cfg!(target_endian = "big") {
            7 - (index % 8)
        } else {
            index % 8
        };
        let mask = 1 << bit_index;
        byte & mask == mask
    }
    #[inline]
    pub fn get_bit(&self, index: usize) -> bool {
        debug_assert!(index / 8 < self.storage.as_ref().len());
        let byte_index = index / 8;
        let byte = self.storage.as_ref()[byte_index];
        Self::extract_bit(byte, index)
    }
    #[inline]
    pub unsafe fn raw_get_bit(this: *const Self, index: usize) -> bool {
        debug_assert!(index / 8 < core::mem::size_of::<Storage>());
        let byte_index = index / 8;
        let byte = unsafe { *(core::ptr::addr_of!((*this).storage) as *const u8).add(byte_index) };
        Self::extract_bit(byte, index)
    }
    #[inline]
    fn change_bit(byte: u8, index: usize, val: bool) -> u8 {
        let bit_index = if cfg!(target_endian = "big") {
            7 - (index % 8)
        } else {
            index % 8
        };
        let mask = 1 << bit_index;
        if val { byte | mask } else { byte & !mask }
    }
    #[inline]
    pub fn set_bit(&mut self, index: usize, val: bool) {
        debug_assert!(index / 8 < self.storage.as_ref().len());
        let byte_index = index / 8;
        let byte = &mut self.storage.as_mut()[byte_index];
        *byte = Self::change_bit(*byte, index, val);
    }
    #[inline]
    pub unsafe fn raw_set_bit(this: *mut Self, index: usize, val: bool) {
        debug_assert!(index / 8 < core::mem::size_of::<Storage>());
        let byte_index = index / 8;
        unsafe {
            let byte = (core::ptr::addr_of_mut!((*this).storage) as *mut u8).add(byte_index);
            *byte = Self::change_bit(*byte, index, val)
        };
    }
    #[inline]
    pub fn get(&self, bit_offset: usize, bit_width: u8) -> u64 {
        debug_assert!(bit_width <= 64);
        debug_assert!(bit_offset / 8 < self.storage.as_ref().len());
        debug_assert!((bit_offset + (bit_width as usize)) / 8 <= self.storage.as_ref().len());
        let mut val = 0;
        for i in 0..(bit_width as usize) {
            if self.get_bit(i + bit_offset) {
                let index = if cfg!(target_endian = "big") {
                    bit_width as usize - 1 - i
                } else {
                    i
                };
                val |= 1 << index;
            }
        }
        val
    }
    #[inline]
    pub unsafe fn raw_get(this: *const Self, bit_offset: usize, bit_width: u8) -> u64 {
        debug_assert!(bit_width <= 64);
        debug_assert!(bit_offset / 8 < core::mem::size_of::<Storage>());
        debug_assert!((bit_offset + (bit_width as usize)) / 8 <= core::mem::size_of::<Storage>());
        let mut val = 0;
        for i in 0..(bit_width as usize) {
            if unsafe { Self::raw_get_bit(this, i + bit_offset) } {
                let index = if cfg!(target_endian = "big") {
                    bit_width as usize - 1 - i
                } else {
                    i
                };
                val |= 1 << index;
            }
        }
        val
    }
    #[inline]
    pub fn set(&mut self, bit_offset: usize, bit_width: u8, val: u64) {
        debug_assert!(bit_width <= 64);
        debug_assert!(bit_offset / 8 < self.storage.as_ref().len());
        debug_assert!((bit_offset + (bit_width as usize)) / 8 <= self.storage.as_ref().len());
        for i in 0..(bit_width as usize) {
            let mask = 1 << i;
            let val_bit_is_set = val & mask == mask;
            let index = if cfg!(target_endian = "big") {
                bit_width as usize - 1 - i
            } else {
                i
            };
            self.set_bit(index + bit_offset, val_bit_is_set);
        }
    }
    #[inline]
    pub unsafe fn raw_set(this: *mut Self, bit_offset: usize, bit_width: u8, val: u64) {
        debug_assert!(bit_width <= 64);
        debug_assert!(bit_offset / 8 < core::mem::size_of::<Storage>());
        debug_assert!((bit_offset + (bit_width as usize)) / 8 <= core::mem::size_of::<Storage>());
        for i in 0..(bit_width as usize) {
            let mask = 1 << i;
            let val_bit_is_set = val & mask == mask;
            let index = if cfg!(target_endian = "big") {
                bit_width as usize - 1 - i
            } else {
                i
            };
            unsafe { Self::raw_set_bit(this, index + bit_offset, val_bit_is_set) };
        }
    }
}

#[repr(C)]
#[derive(Default)]
pub struct __IncompleteArrayField<T>(::core::marker::PhantomData<T>, [T; 0]);
impl<T> __IncompleteArrayField<T> {
    #[inline]
    pub const fn new() -> Self {
        __IncompleteArrayField(::core::marker::PhantomData, [])
    }
    #[inline]
    pub fn as_ptr(&self) -> *const T {
        self as *const _ as *const T
    }
    #[inline]
    pub fn as_mut_ptr(&mut self) -> *mut T {
        self as *mut _ as *mut T
    }
    #[inline]
    pub unsafe fn as_slice(&self, len: usize) -> &[T] {
        unsafe { ::core::slice::from_raw_parts(self.as_ptr(), len) }
    }
    #[inline]
    pub unsafe fn as_mut_slice(&mut self, len: usize) -> &mut [T] {
        unsafe { ::core::slice::from_raw_parts_mut(self.as_mut_ptr(), len) }
    }
}

impl<T> ::core::fmt::Debug for __IncompleteArrayField<T> {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.write_str("__IncompleteArrayField")
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct seginfo {
    pub space: ::core::ffi::c_uchar,
    pub generation: ::core::ffi::c_uchar,
    pub _bitfield_align_1: [u8; 0],
    pub _bitfield_1: __BindgenBitfieldUnit<[u8; 1usize]>,
    pub min_dirty_byte: octet,
    pub list_bits: *mut octet,
    pub number: uptr,
    pub creator: *mut thread_gc,
    pub chunk: *mut chunkinfo,
    pub next: *mut seginfo,
    pub sweep_next: *mut seginfo,
    pub sweep_start: ptr,
    pub dirty_prev: *mut *mut seginfo,
    pub dirty_next: *mut seginfo,
    pub trigger_ephemerons: ptr,
    pub trigger_guardians: ptr,
    pub marked_mask: *mut octet,
    pub marked_count: uptr,
    pub forwarded_flonums: *mut octet,
    pub counting_mask: *mut octet,
    pub measured_mask: *mut octet,
    pub dirty_bytes: [octet; 32usize],
}

impl seginfo {
    #[inline]
    pub fn old_space(&self) -> ::core::ffi::c_uchar {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(0usize, 1u8) as u8) }
    }
    #[inline]
    pub fn set_old_space(&mut self, val: ::core::ffi::c_uchar) {
        unsafe {
            let val: u8 = ::core::mem::transmute(val);
            self._bitfield_1.set(0usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn old_space_raw(this: *const Self) -> ::core::ffi::c_uchar {
        unsafe {
            ::core::mem::transmute(<__BindgenBitfieldUnit<[u8; 1usize]>>::raw_get(
                ::core::ptr::addr_of!((*this)._bitfield_1),
                0usize,
                1u8,
            ) as u8)
        }
    }
    #[inline]
    pub unsafe fn set_old_space_raw(this: *mut Self, val: ::core::ffi::c_uchar) {
        unsafe {
            let val: u8 = ::core::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 1usize]>>::raw_set(
                ::core::ptr::addr_of_mut!((*this)._bitfield_1),
                0usize,
                1u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn use_marks(&self) -> ::core::ffi::c_uchar {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(1usize, 1u8) as u8) }
    }
    #[inline]
    pub fn set_use_marks(&mut self, val: ::core::ffi::c_uchar) {
        unsafe {
            let val: u8 = ::core::mem::transmute(val);
            self._bitfield_1.set(1usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn use_marks_raw(this: *const Self) -> ::core::ffi::c_uchar {
        unsafe {
            ::core::mem::transmute(<__BindgenBitfieldUnit<[u8; 1usize]>>::raw_get(
                ::core::ptr::addr_of!((*this)._bitfield_1),
                1usize,
                1u8,
            ) as u8)
        }
    }
    #[inline]
    pub unsafe fn set_use_marks_raw(this: *mut Self, val: ::core::ffi::c_uchar) {
        unsafe {
            let val: u8 = ::core::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 1usize]>>::raw_set(
                ::core::ptr::addr_of_mut!((*this)._bitfield_1),
                1usize,
                1u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn sorted(&self) -> ::core::ffi::c_uchar {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(2usize, 1u8) as u8) }
    }
    #[inline]
    pub fn set_sorted(&mut self, val: ::core::ffi::c_uchar) {
        unsafe {
            let val: u8 = ::core::mem::transmute(val);
            self._bitfield_1.set(2usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn sorted_raw(this: *const Self) -> ::core::ffi::c_uchar {
        unsafe {
            ::core::mem::transmute(<__BindgenBitfieldUnit<[u8; 1usize]>>::raw_get(
                ::core::ptr::addr_of!((*this)._bitfield_1),
                2usize,
                1u8,
            ) as u8)
        }
    }
    #[inline]
    pub unsafe fn set_sorted_raw(this: *mut Self, val: ::core::ffi::c_uchar) {
        unsafe {
            let val: u8 = ::core::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 1usize]>>::raw_set(
                ::core::ptr::addr_of_mut!((*this)._bitfield_1),
                2usize,
                1u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn has_triggers(&self) -> ::core::ffi::c_uchar {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(3usize, 1u8) as u8) }
    }
    #[inline]
    pub fn set_has_triggers(&mut self, val: ::core::ffi::c_uchar) {
        unsafe {
            let val: u8 = ::core::mem::transmute(val);
            self._bitfield_1.set(3usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn has_triggers_raw(this: *const Self) -> ::core::ffi::c_uchar {
        unsafe {
            ::core::mem::transmute(<__BindgenBitfieldUnit<[u8; 1usize]>>::raw_get(
                ::core::ptr::addr_of!((*this)._bitfield_1),
                3usize,
                1u8,
            ) as u8)
        }
    }
    #[inline]
    pub unsafe fn set_has_triggers_raw(this: *mut Self, val: ::core::ffi::c_uchar) {
        unsafe {
            let val: u8 = ::core::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 1usize]>>::raw_set(
                ::core::ptr::addr_of_mut!((*this)._bitfield_1),
                3usize,
                1u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn must_mark(&self) -> ::core::ffi::c_uchar {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(4usize, 2u8) as u8) }
    }
    #[inline]
    pub fn set_must_mark(&mut self, val: ::core::ffi::c_uchar) {
        unsafe {
            let val: u8 = ::core::mem::transmute(val);
            self._bitfield_1.set(4usize, 2u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn must_mark_raw(this: *const Self) -> ::core::ffi::c_uchar {
        unsafe {
            ::core::mem::transmute(<__BindgenBitfieldUnit<[u8; 1usize]>>::raw_get(
                ::core::ptr::addr_of!((*this)._bitfield_1),
                4usize,
                2u8,
            ) as u8)
        }
    }
    #[inline]
    pub unsafe fn set_must_mark_raw(this: *mut Self, val: ::core::ffi::c_uchar) {
        unsafe {
            let val: u8 = ::core::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 1usize]>>::raw_set(
                ::core::ptr::addr_of_mut!((*this)._bitfield_1),
                4usize,
                2u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn new_bitfield_1(
        old_space: ::core::ffi::c_uchar,
        use_marks: ::core::ffi::c_uchar,
        sorted: ::core::ffi::c_uchar,
        has_triggers: ::core::ffi::c_uchar,
        must_mark: ::core::ffi::c_uchar,
    ) -> __BindgenBitfieldUnit<[u8; 1usize]> {
        let mut __bindgen_bitfield_unit: __BindgenBitfieldUnit<[u8; 1usize]> = Default::default();
        __bindgen_bitfield_unit.set(0usize, 1u8, {
            let old_space: u8 = unsafe { ::core::mem::transmute(old_space) };
            old_space as u64
        });
        __bindgen_bitfield_unit.set(1usize, 1u8, {
            let use_marks: u8 = unsafe { ::core::mem::transmute(use_marks) };
            use_marks as u64
        });
        __bindgen_bitfield_unit.set(2usize, 1u8, {
            let sorted: u8 = unsafe { ::core::mem::transmute(sorted) };
            sorted as u64
        });
        __bindgen_bitfield_unit.set(3usize, 1u8, {
            let has_triggers: u8 = unsafe { ::core::mem::transmute(has_triggers) };
            has_triggers as u64
        });
        __bindgen_bitfield_unit.set(4usize, 2u8, {
            let must_mark: u8 = unsafe { ::core::mem::transmute(must_mark) };
            must_mark as u64
        });
        __bindgen_bitfield_unit
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct chunkinfo {
    pub addr: *mut ::core::ffi::c_void,
    pub base: iptr,
    pub bytes: iptr,
    pub segs: iptr,
    pub nused_segs: iptr,
    pub prev: *mut *mut chunkinfo,
    pub next: *mut chunkinfo,
    pub unused_segs: *mut seginfo,
    pub sis: __IncompleteArrayField<seginfo>,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct t1table {
    pub t1: [*mut seginfo; 65536usize],
    pub refcount: iptr,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct t2table {
    pub t2: [*mut t1table; 131072usize],
    pub refcount: iptr,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct bucket {
    pub sym: ptr,
    pub next: *mut bucket,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct bucket_list {
    pub car: *mut bucket,
    pub cdr: *mut bucket_list,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct bucket_pointer_list {
    pub car: *mut *mut bucket,
    pub cdr: *mut bucket_pointer_list,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct dirtycardinfo {
    pub card: uptr,
    pub youngest: IGEN,
    pub next: *mut dirtycardinfo,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct scheme_mutex_t {
    pub owner: s_thread_t,
    pub count: uptr,
    pub pmutex: s_thread_mutex_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct thread_gc {
    pub tc: ptr,
    pub during_alloc: ::core::ffi::c_int,
    pub queued_fire: IBOOL,
    pub preserve_ownership: IBOOL,
    pub next: *mut thread_gc,
    pub base_loc: [[ptr; 19usize]; 8usize],
    pub next_loc: [[ptr; 19usize]; 8usize],
    pub bytes_left: [[iptr; 19usize]; 8usize],
    pub orig_next_loc: [ptr; 19usize],
    pub sweep_loc: [[ptr; 19usize]; 8usize],
    pub sweep_next: [[*mut seginfo; 19usize]; 8usize],
    pub pending_ephemerons: ptr,
    pub sweep_stack: ptr,
    pub sweep_stack_start: ptr,
    pub sweep_stack_limit: ptr,
    pub sweep_change: ::core::ffi::c_int,
    pub sweeper: ::core::ffi::c_int,
    pub send_remote_sweep_stack: ptr,
    pub send_remote_sweep_stack_start: ptr,
    pub send_remote_sweep_stack_limit: ptr,
    pub receive_remote_sweep_stack: ptr,
    pub receive_remote_sweep_stack_start: ptr,
    pub receive_remote_sweep_stack_limit: ptr,
    pub dirty_segments: [*mut seginfo; 28usize],
    pub bitmask_overhead: [iptr; 8usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct unbufFaslFileObj {
    pub path: ptr,
    pub type_: INT,
    pub fd: INT,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct faslFileObj {
    pub uf: unbufFaslFileObj,
    pub buffer_mode: ::core::ffi::c_int,
    pub remaining: iptr,
    pub next: *mut octet,
    pub end: *mut octet,
    pub buf: *mut octet,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct fileFaslFileObj {
    pub f: faslFileObj,
    pub buf_space: [octet; 4096usize],
}

pub type unbufFaslFile = *mut unbufFaslFileObj;
pub type faslFile = *mut faslFileObj;
pub type fileFaslFile = *mut fileFaslFileObj;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct vfasl_hash_table {
    pub _address: u8,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct gzFile_s {
    pub _address: u8,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct lz4File_in_r {
    pub _address: u8,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct lz4File_out_r {
    pub _address: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct glzFile_r {
    pub fd: INT,
    pub inputp: IBOOL,
    pub format: INT,
    pub __bindgen_anon_1: glzFile_r__bindgen_ty_1,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union glzFile_r__bindgen_ty_1 {
    pub gz: *mut gzFile_s,
    pub lz4_in: *mut lz4File_in_r,
    pub lz4_out: *mut lz4File_out_r,
}

pub type glzFile = *mut glzFile_r;
