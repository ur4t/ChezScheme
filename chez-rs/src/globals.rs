#![allow(dead_code)]

use crate::scheme::{iptr, ptr, uptr};
use crate::thread::{s_thread_cond_t, s_thread_key_t, s_thread_t};
use crate::types::{IBOOL, IGEN, INT};
use crate::types::{
    bucket, bucket_list, chunkinfo, dirtycardinfo, scheme_mutex_t, seginfo, t2table, thread_gc,
    vfasl_hash_table,
};

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct S_G_struct {
    pub thread_context: [f64; 96usize],
    pub main_thread_gc: thread_gc,
    pub active_threads_id: ptr,
    pub error_invoke_code_object: ptr,
    pub invoke_code_object: ptr,
    pub dummy_code_object: ptr,
    pub heap_reserve_ratio_id: ptr,
    pub retain_static_relocation: IBOOL,
    pub enable_object_counts: IBOOL,
    pub enable_object_backreferences: IBOOL,
    pub scheme_version_id: ptr,
    pub make_load_binary_id: ptr,
    pub load_binary: ptr,
    pub profile_counters: ptr,
    pub foreign_static: ptr,
    pub foreign_names: ptr,
    pub threadno: ptr,
    pub occupied_segments: [[*mut seginfo; 19usize]; 8usize],
    pub number_of_nonstatic_segments: uptr,
    pub number_of_empty_segments: uptr,
    pub protected: [*mut ptr; 100usize],
    pub protect_next: uptr,
    pub bytes_of_space: [[uptr; 19usize]; 8usize],
    pub bytes_of_generation: [uptr; 8usize],
    pub bitmask_overhead: [uptr; 8usize],
    pub g0_bytes_after_last_gc: uptr,
    pub collect_trip_bytes: uptr,
    pub nonprocedure_code: ptr,
    pub null_string: ptr,
    pub null_vector: ptr,
    pub null_fxvector: ptr,
    pub null_flvector: ptr,
    pub null_bytevector: ptr,
    pub null_immutable_string: ptr,
    pub null_immutable_vector: ptr,
    pub null_immutable_bytevector: ptr,
    pub zero_length_bignum: ptr,
    pub dirty_segments: [*mut seginfo; 28usize],
    pub error_id: ptr,
    pub nuate_id: ptr,
    pub null_continuation_id: ptr,
    pub collect_request_pending_id: ptr,
    pub event_and_resume_id: ptr,
    pub event_and_resume_star_id: ptr,
    pub guardians: [ptr; 8usize],
    pub locked_objects: [ptr; 8usize],
    pub unlocked_objects: [ptr; 8usize],
    pub min_free_gen: IGEN,
    pub new_min_free_gen: IGEN,
    pub max_nonstatic_generation: IGEN,
    pub new_max_nonstatic_generation: IGEN,
    pub min_mark_gen: IGEN,
    pub countof: [[uptr; 30usize]; 8usize],
    pub bytesof: [[uptr; 30usize]; 8usize],
    pub gctimestamp: [uptr; 8usize],
    pub rtds_with_counts: [ptr; 8usize],
    pub countof_size: [uptr; 30usize],
    pub static_id: ptr,
    pub countof_names: ptr,
    pub gcbackreference: [ptr; 8usize],
    pub prcgeneration: IGEN,
    pub bytes_finalized: uptr,
    pub new_dirty_cards: *mut dirtycardinfo,
    pub must_mark_gen0: IBOOL,
    pub oblist_length: iptr,
    pub oblist_count: iptr,
    pub oblist: *mut *mut bucket,
    pub buckets_of_generation: [*mut bucket_list; 7usize],
    pub library_entry_vector: ptr,
    pub c_entry_vector: ptr,
    pub base_rtd: ptr,
    pub rtd_key: ptr,
    pub eq_symbol: ptr,
    pub eq_ht_rtd: ptr,
    pub symbol_symbol: ptr,
    pub symbol_ht_rtd: ptr,
    pub eqp: ptr,
    pub eqvp: ptr,
    pub equalp: ptr,
    pub symboleqp: ptr,
    pub c_entries: *mut vfasl_hash_table,
    pub library_entries: *mut vfasl_hash_table,
    pub library_entry_codes: *mut vfasl_hash_table,
}

unsafe extern "C" {
    pub static mut S_checkheap: IBOOL;
    pub static mut S_checkheap_errors: uptr;
    pub static mut S_child_processes: [ptr; 8usize];
    pub static mut S_boot_time: IBOOL;
    pub static mut S_vfasl_boot_mode: ::core::ffi::c_int;
    pub static mut S_errors_to_console: IBOOL;
    pub static mut S_threads: ptr;
    pub static mut S_nthreads: uptr;
    pub static mut S_pagesize: uptr;
    pub static mut S_abnormal_exit_proc: ::core::option::Option<unsafe extern "C" fn()>;
    pub static mut Sschemeheapdirs: *mut ::core::ffi::c_char;
    pub static mut Sdefaultheapdirs: *mut ::core::ffi::c_char;
    pub static mut S_tc_key: s_thread_key_t;
    pub static mut S_tc_mutex: scheme_mutex_t;
    pub static mut S_collect_cond: s_thread_cond_t;
    pub static mut S_collect_thread0_cond: s_thread_cond_t;
    pub static mut S_tc_mutex_depth: INT;
    pub static mut S_alloc_mutex: scheme_mutex_t;
    pub static mut S_alloc_mutex_depth: INT;
    pub static mut S_terminated_cond: s_thread_cond_t;
    pub static mut S_collect_waiting_threads: ::core::ffi::c_int;
    pub static mut S_collect_waiting_tcs: [ptr; 16usize];
    pub static mut S_num_preserve_ownership_threads: ::core::ffi::c_int;
    pub static mut S_main_thread_id: s_thread_t;
    pub static mut S_segment_info: [*mut t2table; 131072usize];
    pub static mut S_chunks_full: *mut chunkinfo;
    pub static mut S_chunks: [*mut chunkinfo; 9usize];
    pub static mut S_code_chunks_full: *mut chunkinfo;
    pub static mut S_code_chunks: [*mut chunkinfo; 9usize];
    pub static mut S_foreign_dynamic: ptr;
    pub static mut S_G: S_G_struct;
}
