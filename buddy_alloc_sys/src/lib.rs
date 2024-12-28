#![no_std]
#![allow(warnings)]
/* automatically generated by rust-bindgen 0.70.1 */

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct buddy {
    _unused: [u8; 0],
}
extern "C" {
    pub fn buddy_sizeof(memory_size: usize) -> usize;
}
extern "C" {
    pub fn buddy_sizeof_alignment(memory_size: usize, alignment: usize) -> usize;
}
extern "C" {
    pub fn buddy_init(
        at: *mut ::core::ffi::c_uchar,
        main: *mut ::core::ffi::c_uchar,
        memory_size: usize,
    ) -> *mut buddy;
}
extern "C" {
    pub fn buddy_init_alignment(
        at: *mut ::core::ffi::c_uchar,
        main: *mut ::core::ffi::c_uchar,
        memory_size: usize,
        alignment: usize,
    ) -> *mut buddy;
}
extern "C" {
    pub fn buddy_embed(main: *mut ::core::ffi::c_uchar, memory_size: usize) -> *mut buddy;
}
extern "C" {
    pub fn buddy_get_embed_at(main: *mut ::core::ffi::c_uchar, memory_size: usize) -> *mut buddy;
}
extern "C" {
    pub fn buddy_embed_alignment(
        main: *mut ::core::ffi::c_uchar,
        memory_size: usize,
        alignment: usize,
    ) -> *mut buddy;
}
extern "C" {
    pub fn buddy_get_embed_at_alignment(
        main: *mut ::core::ffi::c_uchar,
        memory_size: usize,
        alignment: usize,
    ) -> *mut buddy;
}
extern "C" {
    pub fn buddy_resize(buddy: *mut buddy, new_memory_size: usize) -> *mut buddy;
}
extern "C" {
    pub fn buddy_can_shrink(buddy: *mut buddy) -> bool;
}
extern "C" {
    pub fn buddy_is_empty(buddy: *mut buddy) -> bool;
}
extern "C" {
    pub fn buddy_is_full(buddy: *mut buddy) -> bool;
}
extern "C" {
    pub fn buddy_arena_size(buddy: *mut buddy) -> usize;
}
extern "C" {
    pub fn buddy_arena_free_size(buddy: *mut buddy) -> usize;
}
extern "C" {
    pub fn buddy_malloc(buddy: *mut buddy, requested_size: usize) -> *mut ::core::ffi::c_void;
}
extern "C" {
    pub fn buddy_calloc(
        buddy: *mut buddy,
        members_count: usize,
        member_size: usize,
    ) -> *mut ::core::ffi::c_void;
}
extern "C" {
    pub fn buddy_realloc(
        buddy: *mut buddy,
        ptr: *mut ::core::ffi::c_void,
        requested_size: usize,
        ignore_data: bool,
    ) -> *mut ::core::ffi::c_void;
}
extern "C" {
    pub fn buddy_reallocarray(
        buddy: *mut buddy,
        ptr: *mut ::core::ffi::c_void,
        members_count: usize,
        member_size: usize,
        ignore_data: bool,
    ) -> *mut ::core::ffi::c_void;
}
extern "C" {
    pub fn buddy_free(buddy: *mut buddy, ptr: *mut ::core::ffi::c_void);
}
pub const buddy_safe_free_status_BUDDY_SAFE_FREE_SUCCESS: buddy_safe_free_status = 0;
pub const buddy_safe_free_status_BUDDY_SAFE_FREE_BUDDY_IS_NULL: buddy_safe_free_status = 1;
pub const buddy_safe_free_status_BUDDY_SAFE_FREE_INVALID_ADDRESS: buddy_safe_free_status = 2;
pub const buddy_safe_free_status_BUDDY_SAFE_FREE_SIZE_MISMATCH: buddy_safe_free_status = 3;
pub const buddy_safe_free_status_BUDDY_SAFE_FREE_ALREADY_FREE: buddy_safe_free_status = 4;
pub type buddy_safe_free_status = ::core::ffi::c_uint;
extern "C" {
    pub fn buddy_safe_free(
        buddy: *mut buddy,
        ptr: *mut ::core::ffi::c_void,
        requested_size: usize,
    ) -> buddy_safe_free_status;
}
extern "C" {
    pub fn buddy_reserve_range(
        buddy: *mut buddy,
        ptr: *mut ::core::ffi::c_void,
        requested_size: usize,
    );
}
extern "C" {
    pub fn buddy_unsafe_release_range(
        buddy: *mut buddy,
        ptr: *mut ::core::ffi::c_void,
        requested_size: usize,
    );
}
extern "C" {
    pub fn buddy_walk(
        buddy: *mut buddy,
        fp: ::core::option::Option<
            unsafe extern "C" fn(
                ctx: *mut ::core::ffi::c_void,
                addr: *mut ::core::ffi::c_void,
                slot_size: usize,
                allocated: usize,
            ) -> *mut ::core::ffi::c_void,
        >,
        ctx: *mut ::core::ffi::c_void,
    ) -> *mut ::core::ffi::c_void;
}
extern "C" {
    pub fn buddy_fragmentation(buddy: *mut buddy) -> ::core::ffi::c_uchar;
}
