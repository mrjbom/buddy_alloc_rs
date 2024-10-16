#![no_std]

// Rust-style binding to buddy_alloc

use core::ffi::c_void;

/// Buddy allocator
pub struct BuddyAlloc {
    /// Used by buddy_alloc-sys functions
    pub buddy_ptr: *mut buddy_alloc_sys::buddy,
}

/// For [`BuddyAlloc::safe_free`]
#[derive(Debug, Copy, Clone)]
pub enum BuddySafeFreeStatus {
    Success,
    BuddyIsNull,
    InvalidAddress,
    SizeMismatch,
    AlreadyFree,
}

impl BuddyAlloc {
    /// Returns the size of a buddy required to manage a block of the specified size
    pub unsafe fn sizeof(memory_size: usize) -> Result<usize, ()> {
        let size = unsafe { buddy_alloc_sys::buddy_sizeof(memory_size) };

        if size != 0 {
            Ok(size)
        } else {
            Err(())
        }
    }

    /// Returns the size of a buddy required to manage a block of the specified size
    /// using a non-default alignment.
    pub unsafe fn sizeof_alignment(memory_size: usize, alignment: usize) -> Result<usize, ()> {
        let size = unsafe { buddy_alloc_sys::buddy_sizeof_alignment(memory_size, alignment) };

        if size != 0 {
            Ok(size)
        } else {
            Err(())
        }
    }

    /// Initializes a binary buddy memory allocator at the specified location
    pub unsafe fn init(at: *mut u8, memory: *mut u8, memory_size: usize) -> Result<Self, ()> {
        let buddy_ptr = buddy_alloc_sys::buddy_init(at, memory, memory_size);

        if !buddy_ptr.is_null() {
            Ok(Self { buddy_ptr })
        } else {
            Err(())
        }
    }

    /// Initializes a binary buddy memory allocator at the specified location using a non-default alignment
    pub unsafe fn init_alignment(
        at: *mut u8,
        memory: *mut u8,
        memory_size: usize,
        alignment: usize,
    ) -> Result<Self, ()> {
        let buddy_ptr = buddy_alloc_sys::buddy_init_alignment(at, memory, memory_size, alignment);

        if !buddy_ptr.is_null() {
            Ok(Self { buddy_ptr })
        } else {
            Err(())
        }
    }

    /// Initializes a binary buddy memory allocator embedded in the specified arena.
    ///
    /// The arena's capacity is reduced to account for the allocator metadata.
    pub unsafe fn embed(main: *mut u8, memory_size: usize) -> Result<Self, ()> {
        let buddy_ptr = buddy_alloc_sys::buddy_embed(main, memory_size);

        if !buddy_ptr.is_null() {
            Ok(Self { buddy_ptr })
        } else {
            Err(())
        }
    }

    /// Returns the address of a previously-created buddy allocator at the arena.
    ///
    /// Use to get a new handle to the allocator when the arena is moved or copied.
    pub unsafe fn get_embed_at(main: *mut u8, memory_size: usize) -> Result<Self, ()> {
        let buddy_ptr = buddy_alloc_sys::buddy_get_embed_at(main, memory_size);

        if !buddy_ptr.is_null() {
            Ok(Self { buddy_ptr })
        } else {
            Err(())
        }
    }

    /// Initializes a binary buddy memory allocator embedded in the specified arena
    /// using a non-default alignment.
    ///
    /// The arena's capacity is reduced to account for the allocator metadata.
    pub unsafe fn embed_alignment(
        main: *mut u8,
        memory_size: usize,
        alignment: usize,
    ) -> Result<Self, ()> {
        let buddy_ptr = buddy_alloc_sys::buddy_embed_alignment(main, memory_size, alignment);

        if !buddy_ptr.is_null() {
            Ok(Self { buddy_ptr })
        } else {
            Err(())
        }
    }

    /// Returns the address of a previously-created buddy allocator at the arena.
    ///
    /// Use to get a new handle to the allocator when the arena is moved or copied.
    pub unsafe fn get_embed_at_alignment(
        main: *mut u8,
        memory_size: usize,
        alignment: usize,
    ) -> Result<Self, ()> {
        let buddy_ptr = buddy_alloc_sys::buddy_get_embed_at_alignment(main, memory_size, alignment);

        if !buddy_ptr.is_null() {
            Ok(Self { buddy_ptr })
        } else {
            Err(())
        }
    }

    /// Resizes the arena and metadata to a new size.
    pub unsafe fn resize(&self, new_memory_size: usize) -> Result<Self, ()> {
        let buddy_ptr = buddy_alloc_sys::buddy_resize(self.buddy_ptr, new_memory_size);

        if !buddy_ptr.is_null() {
            Ok(Self { buddy_ptr })
        } else {
            Err(())
        }
    }

    /// Tests if the allocator can be shrunk in half
    pub unsafe fn can_shrink(&self) -> bool {
        buddy_alloc_sys::buddy_can_shrink(self.buddy_ptr)
    }

    /// Tests if the allocator is completely empty
    pub unsafe fn is_empty(&self) -> bool {
        buddy_alloc_sys::buddy_is_empty(self.buddy_ptr)
    }

    /// Tests if the allocator is completely full
    pub unsafe fn is_full(&self) -> bool {
        buddy_alloc_sys::buddy_is_full(self.buddy_ptr)
    }

    /// Reports the arena size
    pub unsafe fn arena_size(&self) -> usize {
        buddy_alloc_sys::buddy_arena_size(self.buddy_ptr)
    }

    /// Reports the arena's free size.
    ///
    /// Note that this is (often) not a continuous size but the sum of all free slots in the buddy.
    pub unsafe fn arena_free_size(&self) -> usize {
        buddy_alloc_sys::buddy_arena_free_size(self.buddy_ptr)
    }

    // Allocation functions

    /// Use the specified buddy to allocate memory. See malloc.
    pub unsafe fn malloc(&self, requested_size: usize) -> *mut u8 {
        buddy_alloc_sys::buddy_malloc(self.buddy_ptr, requested_size).cast::<u8>()
    }

    /// Use the specified buddy to allocate zeroed memory. See calloc.
    pub unsafe fn calloc(&self, members_count: usize, member_size: usize) -> *mut u8 {
        buddy_alloc_sys::buddy_calloc(self.buddy_ptr, members_count, member_size).cast::<u8>()
    }

    /// Realloc semantics are a joke. See realloc.
    pub unsafe fn realloc(
        &self,
        ptr: *mut u8,
        requested_size: usize,
        ignore_data: bool,
    ) -> *mut u8 {
        buddy_alloc_sys::buddy_realloc(
            self.buddy_ptr,
            ptr.cast::<c_void>(),
            requested_size,
            ignore_data,
        )
        .cast::<u8>()
    }

    /// Realloc-like behavior that checks for overflow. See reallocarray
    pub unsafe fn reallocarray(
        &self,
        ptr: *mut u8,
        members_count: usize,
        member_size: usize,
        ignore_data: bool,
    ) -> *mut u8 {
        buddy_alloc_sys::buddy_reallocarray(
            self.buddy_ptr,
            ptr.cast::<c_void>(),
            members_count,
            member_size,
            ignore_data,
        )
        .cast::<u8>()
    }

    /// Use the specified buddy to free memory. See free.
    pub unsafe fn free(&self, ptr: *mut u8) {
        buddy_alloc_sys::buddy_free(self.buddy_ptr, ptr.cast::<c_void>());
    }

    /// A (safer) free with a size. Will not free unless the size fits the target span.
    pub unsafe fn safe_free(&self, ptr: *mut u8, requested_size: usize) -> BuddySafeFreeStatus {
        let buddy_safe_free_status_ffi =
            buddy_alloc_sys::buddy_safe_free(self.buddy_ptr, ptr.cast::<c_void>(), requested_size);
        match buddy_safe_free_status_ffi {
            buddy_alloc_sys::buddy_safe_free_status_BUDDY_SAFE_FREE_SUCCESS => {
                BuddySafeFreeStatus::Success
            }
            buddy_alloc_sys::buddy_safe_free_status_BUDDY_SAFE_FREE_BUDDY_IS_NULL => {
                BuddySafeFreeStatus::BuddyIsNull
            }
            buddy_alloc_sys::buddy_safe_free_status_BUDDY_SAFE_FREE_INVALID_ADDRESS => {
                BuddySafeFreeStatus::InvalidAddress
            }
            buddy_alloc_sys::buddy_safe_free_status_BUDDY_SAFE_FREE_SIZE_MISMATCH => {
                BuddySafeFreeStatus::SizeMismatch
            }
            buddy_alloc_sys::buddy_safe_free_status_BUDDY_SAFE_FREE_ALREADY_FREE => {
                BuddySafeFreeStatus::AlreadyFree
            }
            _ => unreachable!("Unexpected safe free status"),
        }
    }

    // Reservation functions

    /// Reserve a range by marking it as allocated. Useful for dealing with physical memory.
    pub unsafe fn reserve_range(&self, ptr: *mut u8, requested_size: usize) {
        buddy_alloc_sys::buddy_reserve_range(self.buddy_ptr, ptr.cast::<c_void>(), requested_size);
    }

    /// Release a reserved memory range. Unsafe, this can mess up other allocations if called with wrong parameters!
    pub unsafe fn unsafe_release_range(&self, ptr: *mut u8, requested_size: usize) {
        buddy_alloc_sys::buddy_unsafe_release_range(
            self.buddy_ptr,
            ptr.cast::<c_void>(),
            requested_size,
        );
    }

    // Iteration functions

    /// Iterate through the free and allocated slots and call the provided function for each of them.
    ///
    /// If the provided function returns a non-NULL result the iteration stops and the result
    /// is returned to called. NULL is returned upon completing iteration without stopping.
    ///
    /// The iteration order is implementation-defined and may change between versions.
    ///
    /// Parameters in closure: `ctx, addr, slot_size, allocated`
    /// # Example
    /// ```ignore
    /// buddy_allocator.walk(
    ///             &mut |_ctx, addr, slot_size, allocated| {
    ///         println!("{addr:p}, {slot_size}, {allocated}");
    ///         0 as *mut u8
    ///     },
    ///         0 as *mut u8,
    /// );
    /// ```
    pub unsafe fn walk(
        &self,
        closure: &mut dyn FnMut(*mut u8, *mut u8, usize, usize) -> *mut u8,
        ctx: *mut u8,
    ) -> *mut u8 {
        // Trampoline С function
        unsafe extern "C" fn trampoline(
            closure_and_ctx_ptr: *mut c_void,
            addr: *mut c_void,
            slot_size: usize,
            allocated: usize,
        ) -> *mut c_void {
            // Cast the closure_and_ctx_ptr back to its original type
            let (closure, ctx) = &mut *(closure_and_ctx_ptr
                as *mut (
                    &mut dyn FnMut(*mut u8, *mut u8, usize, usize) -> *mut u8,
                    *mut u8,
                ));
            // Call the closure
            let result = closure(*ctx, addr as *mut u8, slot_size, allocated);
            result as *mut c_void
        }

        // Package the closure and ctx together
        let closure_and_ctx = (closure, ctx);

        // Cast to a raw pointer
        let closure_and_ctx_ptr = &closure_and_ctx as *const _ as *mut c_void;

        buddy_alloc_sys::buddy_walk(self.buddy_ptr, Some(trampoline), closure_and_ctx_ptr)
            .cast::<u8>()
    }

    // Miscellaneous functions

    /// Calculates the fragmentation in the allocator in a 0 - 255 range.
    ///
    /// NOTE: if you are using a non-power-of-two sized arena the maximum upper bound can be lower.
    pub unsafe fn fragmentation(&self) -> u8 {
        buddy_alloc_sys::buddy_fragmentation(self.buddy_ptr)
    }
}
