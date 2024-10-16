fn main() {
    unsafe {
        let buddy_metadata_size = buddy_alloc::BuddyAlloc::sizeof_alignment(16384, 4096).unwrap();
        let mut buddy_metadata = vec![0u8; buddy_metadata_size];

        let buddy_allocator = buddy_alloc::BuddyAlloc::init_alignment(
            buddy_metadata.as_mut_ptr(),
            0x1000 as *mut u8,
            16384,
            4096,
        )
        .unwrap();

        println!("{:p}", buddy_allocator.malloc(4096));
        //println!("{:p}", buddy_allocator.malloc(8192));
        buddy_allocator.walk(
            &mut |_ctx, addr, slot_size, allocated| {
                println!("{addr:p}, {slot_size}, {allocated}");
                0 as *mut u8
            },
            0 as *mut u8,
        );
    }
}
