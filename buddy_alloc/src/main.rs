/// Example
use buddy_alloc::BuddyAlloc;

fn main() {
    unsafe {
        // Prepare metadata memory and arena
        let arena_size = 65536;
        let buddy_metadata_size = BuddyAlloc::sizeof(arena_size).expect("Wrong arena size");
        let mut buddy_metadata = vec![0u8; buddy_metadata_size];
        let mut buddy_arena = vec![0u8; arena_size];

        // Create allocator
        let buddy_allocator = BuddyAlloc::init(
            buddy_metadata.as_mut_ptr(),
            buddy_arena.as_mut_ptr(),
            arena_size,
        )
        .expect("Failed to create allocator");

        // Allocate using the buddy allocator
        let data: *mut u8 = buddy_allocator.malloc(2048);
        // Free using the buddy allocator
        buddy_allocator.free(data);

        // Or using buddy_alloc_sys FFI binding
        let data: *mut core::ffi::c_void =
            buddy_alloc_sys::buddy_malloc(buddy_allocator.buddy_ptr, 2048);
        buddy_alloc_sys::buddy_free(buddy_allocator.buddy_ptr, data);
    }
}
