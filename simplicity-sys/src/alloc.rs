use std::alloc::{self, Layout};
use std::mem;

/// A type whose alignment is greater equal that of any C type.
/// Allocations with this alignment are valid for all C types.
/// Remake of `max_align_t` from `stddef.h`.
/// [16 bytes is the greatest alignment of any architecture Rust currently supports](https://github.com/rust-lang/rust/blob/61223975d46f794466efa832bc7562b9707ecc46/library/std/src/sys/pal/common/alloc.rs).
#[repr(align(16))]
#[derive(Default, Copy, Clone)]
#[allow(dead_code)]
pub struct AlignedType([u8; 16]);

/// Minimal alignment which is valid for all C types.
pub const MIN_ALIGN: usize = mem::align_of::<AlignedType>();

/// Signature of an allocator function.
type AllocatorFn = unsafe fn(Layout) -> *mut u8;

/// Allocate `size_bytes` many bytes using the `allocator` function
/// and return a pointer.
///
/// # Panics
///
/// This function panics if `size_bytes` + [`MIN_ALIGN`],
/// rounded up to next multiple of [`MIN_ALIGN`],
/// is greater than [`isize::MAX`] (allocated too many bytes).
///
/// # Safety
///
/// - `allocator` must be [`alloc::alloc`] or [`alloc::alloc_zeroed`].
/// - Allocated bytes must be freed using [`rust_free`].
unsafe fn allocate(size_bytes: usize, allocator: AllocatorFn) -> *mut u8 {
    assert!(mem::align_of::<usize>() <= MIN_ALIGN);
    assert!(mem::align_of::<&usize>() <= MIN_ALIGN);
    assert!(mem::size_of::<usize>() <= MIN_ALIGN);

    // We allocate a sequence of N bytes (size_bytes) that will hold the actual data,
    // prefixed by a sequence of MIN_ALIGN bytes that hold the number `MIN_ALIGN + N`
    // (number of allocated bytes).
    // The prefix needs to be offset by MIN_ALIGN to ensure correct alignment of the next bytes.
    // Finally, we return a pointer after this prefix for the caller to use.
    //
    //                 MIN_ALIGN       MIN_ALIGN + 2
    //                 |               |
    // +---------------+-------+-------+-----+-------+
    // | MIN_ALIGN + N | byte0 | byte1 | ... | byteN |
    // +---------------+-------+-------+-----+-------+
    // |               ^       |                     |
    // 0               |       MIN_ALIGN + 1         MIN_ALIGN + N
    //                 WE RETURN THIS POINTER
    //
    let size_prefixed_bytes = MIN_ALIGN.saturating_add(size_bytes);
    // PANIC: Allocated too many bytes (documented above).
    let layout =
        Layout::from_size_align(size_prefixed_bytes, MIN_ALIGN).expect("allocated too many bytes");
    // SAFETY: `layout` is nonzero.
    let ptr_prefix = allocator(layout);
    if ptr_prefix.is_null() {
        // Abort execution if allocation failed.
        alloc::handle_alloc_error(layout);
    }
    // Write number of allocated bytes into prefix.
    //
    // SAFETY: prefix is valid for writes and well-aligned.
    (ptr_prefix as *mut usize).write(size_prefixed_bytes);
    // Return pointer behind prefix.
    //
    // SAFETY: `ptr_prefix` and `ptr_prefix + MIN_ALIGN` are part of same allocated object.
    ptr_prefix.add(MIN_ALIGN)
}

/// Allocate `size_bytes` many bytes and return a pointer.
///
/// # Safety
///
/// Allocated bytes must be freed using [`rust_free`].
#[no_mangle]
pub unsafe extern "C" fn rust_malloc(size_bytes: usize) -> *mut u8 {
    // SAFETY: Allocator is `alloc::alloc`.
    allocate(size_bytes, alloc::alloc)
}

/// Allocate `size_bytes` many zeroed bytes and return a pointer.
///
/// # Safety
///
/// Allocated bytes must be freed using [`rust_free`].
#[no_mangle]
pub unsafe extern "C" fn rust_calloc(num: usize, size: usize) -> *mut u8 {
    let size_bytes = num * size;
    // SAFETY: Allocator is `alloc_alloc_zeroed`.
    allocate(size_bytes, alloc::alloc_zeroed)
}

/// Free allocated bytes at `ptr_bytes`.
///
/// # Safety
///
/// - `ptr_bytes` must have been allocated using [`rust_malloc`] or [`rust_calloc`].
/// - If `ptr_bytes` is a `NULL` pointer, then this function is a NO-OP.
#[no_mangle]
pub unsafe extern "C" fn rust_free(ptr_bytes: *mut u8) {
    if ptr_bytes.is_null() {
        return;
    }

    // We got a pointer to an allocation from `rust_malloc` or `rust_calloc`,
    // so the memory looks as follows.
    // There is a prefix of `MIN_ALIGN` bytes in front of the pointer we got.
    // This prefix holds the total number of allocated bytes.
    // We free this number of bytes to free the entire sequence.
    //
    //                 MIN_ALIGN       MIN_ALIGN + 2
    //                 |               |
    // +---------------+-------+-------+-----+-------+
    // | MIN_ALIGN + N | byte0 | byte1 | ... | byteN |
    // +---------------+-------+-------+-----+-------+
    // |               ^       |                     |
    // 0               |       MIN_ALIGN + 1         MIN_ALIGN + N
    //                 WE GOT THIS POINTER
    //
    // SAFETY: `ptr_bytes` and `ptr_bytes - MIN_ALIGN` are part of same allocated object.
    let ptr_prefix = ptr_bytes.sub(MIN_ALIGN);
    // SAFETY: prefix is valid for reads and well-aligned.
    let size_prefixed_bytes = (ptr_prefix as *mut usize).read();
    // INFALLIBLE: This layout was already allocated, so there is no overflow.
    let layout = Layout::from_size_align(size_prefixed_bytes, MIN_ALIGN).unwrap();
    // SAFETY: `ptr_prefix` was allocated via same allocator with same layout.
    alloc::dealloc(ptr_prefix, layout)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn aligned_type_size() {
        unsafe {
            assert!(MIN_ALIGN >= mem::size_of::<usize>());
            assert!(MIN_ALIGN >= mem::size_of::<u128>());
            assert!(MIN_ALIGN >= crate::ffi::c_sizeof_long_double);
        }
    }

    #[test]
    fn aligned_type_align() {
        unsafe {
            assert!(MIN_ALIGN.is_power_of_two());
            assert!(MIN_ALIGN >= mem::align_of::<usize>());
            assert!(MIN_ALIGN >= mem::align_of::<&usize>());
            assert!(MIN_ALIGN >= mem::align_of::<u128>());
            assert!(MIN_ALIGN >= crate::ffi::c_alignof_long_double);
        }
    }
}
