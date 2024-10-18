#![no_std]
#![allow(warnings)]
/* automatically generated by rust-bindgen 0.70.1 */

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
extern "C" {
    pub fn buddy_debug(buddy: *mut buddy);
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct buddy_tree_pos {
    pub index: usize,
    pub depth: usize,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of buddy_tree_pos"][::core::mem::size_of::<buddy_tree_pos>() - 16usize];
    ["Alignment of buddy_tree_pos"][::core::mem::align_of::<buddy_tree_pos>() - 8usize];
    ["Offset of field: buddy_tree_pos::index"]
        [::core::mem::offset_of!(buddy_tree_pos, index) - 0usize];
    ["Offset of field: buddy_tree_pos::depth"]
        [::core::mem::offset_of!(buddy_tree_pos, depth) - 8usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct buddy_tree_interval {
    pub from: buddy_tree_pos,
    pub to: buddy_tree_pos,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of buddy_tree_interval"][::core::mem::size_of::<buddy_tree_interval>() - 32usize];
    ["Alignment of buddy_tree_interval"][::core::mem::align_of::<buddy_tree_interval>() - 8usize];
    ["Offset of field: buddy_tree_interval::from"]
        [::core::mem::offset_of!(buddy_tree_interval, from) - 0usize];
    ["Offset of field: buddy_tree_interval::to"]
        [::core::mem::offset_of!(buddy_tree_interval, to) - 16usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct buddy_tree_walk_state {
    pub starting_pos: buddy_tree_pos,
    pub current_pos: buddy_tree_pos,
    pub going_up: ::core::ffi::c_uint,
    pub walk_done: ::core::ffi::c_uint,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of buddy_tree_walk_state"][::core::mem::size_of::<buddy_tree_walk_state>() - 40usize];
    ["Alignment of buddy_tree_walk_state"]
        [::core::mem::align_of::<buddy_tree_walk_state>() - 8usize];
    ["Offset of field: buddy_tree_walk_state::starting_pos"]
        [::core::mem::offset_of!(buddy_tree_walk_state, starting_pos) - 0usize];
    ["Offset of field: buddy_tree_walk_state::current_pos"]
        [::core::mem::offset_of!(buddy_tree_walk_state, current_pos) - 16usize];
    ["Offset of field: buddy_tree_walk_state::going_up"]
        [::core::mem::offset_of!(buddy_tree_walk_state, going_up) - 32usize];
    ["Offset of field: buddy_tree_walk_state::walk_done"]
        [::core::mem::offset_of!(buddy_tree_walk_state, walk_done) - 36usize];
};
pub const buddy_tree_release_status_BUDDY_TREE_RELEASE_SUCCESS: buddy_tree_release_status = 0;
pub const buddy_tree_release_status_BUDDY_TREE_RELEASE_FAIL_PARTIALLY_USED:
    buddy_tree_release_status = 1;
pub type buddy_tree_release_status = ::core::ffi::c_uint;
extern "C" {
    pub fn buddy_tree_check_invariant(
        t: *mut buddy_tree,
        pos: buddy_tree_pos,
    ) -> ::core::ffi::c_uint;
}
pub const BUDDY_RELATIVE_MODE: ::core::ffi::c_uint = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct buddy {
    pub memory_size: usize,
    pub alignment: usize,
    pub arena: buddy__bindgen_ty_1,
    pub buddy_flags: usize,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union buddy__bindgen_ty_1 {
    pub main: *mut ::core::ffi::c_uchar,
    pub main_offset: isize,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of buddy__bindgen_ty_1"][::core::mem::size_of::<buddy__bindgen_ty_1>() - 8usize];
    ["Alignment of buddy__bindgen_ty_1"][::core::mem::align_of::<buddy__bindgen_ty_1>() - 8usize];
    ["Offset of field: buddy__bindgen_ty_1::main"]
        [::core::mem::offset_of!(buddy__bindgen_ty_1, main) - 0usize];
    ["Offset of field: buddy__bindgen_ty_1::main_offset"]
        [::core::mem::offset_of!(buddy__bindgen_ty_1, main_offset) - 0usize];
};
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of buddy"][::core::mem::size_of::<buddy>() - 32usize];
    ["Alignment of buddy"][::core::mem::align_of::<buddy>() - 8usize];
    ["Offset of field: buddy::memory_size"][::core::mem::offset_of!(buddy, memory_size) - 0usize];
    ["Offset of field: buddy::alignment"][::core::mem::offset_of!(buddy, alignment) - 8usize];
    ["Offset of field: buddy::arena"][::core::mem::offset_of!(buddy, arena) - 16usize];
    ["Offset of field: buddy::buddy_flags"][::core::mem::offset_of!(buddy, buddy_flags) - 24usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct buddy_embed_check {
    pub can_fit: ::core::ffi::c_uint,
    pub offset: usize,
    pub buddy_size: usize,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of buddy_embed_check"][::core::mem::size_of::<buddy_embed_check>() - 24usize];
    ["Alignment of buddy_embed_check"][::core::mem::align_of::<buddy_embed_check>() - 8usize];
    ["Offset of field: buddy_embed_check::can_fit"]
        [::core::mem::offset_of!(buddy_embed_check, can_fit) - 0usize];
    ["Offset of field: buddy_embed_check::offset"]
        [::core::mem::offset_of!(buddy_embed_check, offset) - 8usize];
    ["Offset of field: buddy_embed_check::buddy_size"]
        [::core::mem::offset_of!(buddy_embed_check, buddy_size) - 16usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct buddy_tree {
    pub upper_pos_bound: usize,
    pub size_for_order_offset: usize,
    pub order: u8,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of buddy_tree"][::core::mem::size_of::<buddy_tree>() - 24usize];
    ["Alignment of buddy_tree"][::core::mem::align_of::<buddy_tree>() - 8usize];
    ["Offset of field: buddy_tree::upper_pos_bound"]
        [::core::mem::offset_of!(buddy_tree, upper_pos_bound) - 0usize];
    ["Offset of field: buddy_tree::size_for_order_offset"]
        [::core::mem::offset_of!(buddy_tree, size_for_order_offset) - 8usize];
    ["Offset of field: buddy_tree::order"][::core::mem::offset_of!(buddy_tree, order) - 16usize];
};
