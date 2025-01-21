use crate::{IPtr, Ptr};

use crate::ta6le::equates::{max_real_space, static_generation};

// documentary names for ints and unsigned ints
pub type INT = core::ffi::c_int; // honest-to-goodness C int
pub type UINT = core::ffi::c_uint; // honest-to-goodness C unsigned int
pub type ITYPE = core::ffi::c_int; // ptr types
pub type ISPC = core::ffi::c_int; // storage manager spaces
pub type IGEN = core::ffi::c_int; // storage manager generations
pub type IDIRTYBYTE = core::ffi::c_int; // storage manager dirty bytes
pub type IBOOL = core::ffi::c_int; // int used exclusively as a boolean
pub type ICHAR = core::ffi::c_int; // int used exclusively as a character
pub type IFASLCODE = core::ffi::c_int; // fasl type codes

pub type Int = INT;

const fn get_dirty_segment_index(from_g: usize, to_g: usize) -> usize {
    ((from_g * (from_g - 1)) >> 1) + to_g
}

const DIRTY_SEGMENT_LISTS: usize = get_dirty_segment_index(static_generation, static_generation);

struct SegInfo {}

#[repr(C)]
pub struct ThreadGc {
    tc: Ptr,

    during_alloc: Int,
    queued_fire: IBOOL,
    preserve_ownership: IBOOL,

    next: *mut ThreadGc,

    base_loc: [[Ptr; static_generation + 1]; max_real_space + 1],
    next_loc: [[Ptr; static_generation + 1]; max_real_space + 1],
    bytes_left: [[IPtr; static_generation + 1]; max_real_space + 1],
    orig_next_loc: [Ptr; max_real_space + 1],
    sweep_loc: [[Ptr; static_generation + 1]; max_real_space + 1],
    sweep_next: [[*mut SegInfo; static_generation + 1]; max_real_space + 1],

    pending_ephemerons: Ptr,

    sweep_stack: Ptr,
    sweep_stack_start: Ptr,
    sweep_stack_limit: Ptr,

    sweep_change: Int,

    sweeper: Int, // parallel GC: sweeper thread identity

    // modified only by owning sweeper; contains Ptr and thread_gc*
    send_remote_sweep_stack: Ptr,
    send_remote_sweep_stack_start: Ptr,
    send_remote_sweep_stack_limit: Ptr,

    // modified with sweeper mutex held; contains just Ptr
    receive_remote_sweep_stack: Ptr,
    receive_remote_sweep_stack_start: Ptr,
    receive_remote_sweep_stack_limit: Ptr,

    dirty_segments: [*mut SegInfo; DIRTY_SEGMENT_LISTS],

    bitmask_overhead: [IPtr; static_generation + 1],
}
