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
/// # Safety
///
/// `allocator` must be [`alloc::alloc`] or [`alloc::alloc_zeroed`].
///
/// Allocated bytes must be freed using [`rust_free`].
unsafe fn allocate(size_bytes: usize, allocator: AllocatorFn) -> *mut u8 {
    assert!(MIN_ALIGN >= mem::align_of::<usize>());
    assert!(MIN_ALIGN >= mem::align_of::<&usize>());
    assert!(MIN_ALIGN >= mem::size_of::<usize>());

    // Allocate MIN_ALIGN + size_bytes many bytes
    // Panic if too many bytes are tried to be allocated
    let size_prefixed_bytes = MIN_ALIGN + size_bytes;
    let layout = Layout::from_size_align(size_prefixed_bytes, MIN_ALIGN).unwrap();
    let ptr_prefix = allocator(layout);
    if ptr_prefix.is_null() {
        alloc::handle_alloc_error(layout);
    }
    // Write the number of allocated bytes to memory
    (ptr_prefix as *mut usize).write(size_prefixed_bytes);
    // Return a pointer to the size_bytes many allocated bytes behind the counter
    // We need to offset the pointer by MIN_ALIGN to keep alignment
    // This means there is a gap of MIN_ALIGN - sizeof(size_t) many unused bytes
    // We asserted that MIN_ALIGN >= sizeof(size_t), so this gap is nonnegative
    ptr_prefix.add(MIN_ALIGN)
}

/// Allocate `size_bytes` many bytes and return a pointer.
///
/// # Safety
///
/// Allocated bytes must be freed using [`rust_free`].
#[no_mangle]
pub unsafe extern "C" fn rust_malloc(size_bytes: usize) -> *mut u8 {
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
    allocate(size_bytes, alloc::alloc_zeroed)
}

/// Free allocated bytes at `ptr_bytes`.
///
/// # Safety
///
/// Bytes must have been allocated using [`rust_malloc`] or [`rust_calloc`].
#[no_mangle]
pub unsafe extern "C" fn rust_free(ptr_bytes: *mut u8) {
    if ptr_bytes.is_null() {
        return;
    }

    // Move MIN_ALIGN many bytes back in memory
    // and read the number of allocated bytes
    let ptr_prefix = ptr_bytes.sub(MIN_ALIGN);
    let size_prefixed_bytes = (ptr_prefix as *mut usize).read();
    // Free the allocated bytes including the counter
    // Panic if the number of bytes overflows
    let layout = Layout::from_size_align(size_prefixed_bytes, MIN_ALIGN).unwrap();
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
