#ifndef SIMPLICITY_SIMPLICITY_ALLOC_H
#define SIMPLICITY_SIMPLICITY_ALLOC_H

#include <stdlib.h>

/* Declare Rust functions so the compiler can handle them.
 * The linker will include the functions from Rust.
 */
extern void* rust_0_5_malloc(size_t size);
extern void* rust_0_5_calloc(size_t num, size_t size);
extern void rust_0_5_free(void* ptr);

/* Allocate with rust_0_5_malloc. */
#define rustsimplicity_0_5_malloc rust_0_5_malloc

/* Allocate+zero initialize with rust_0_5_calloc. */
#define rustsimplicity_0_5_calloc rust_0_5_calloc

/* Deallocate with rust_0_5_free. */
#define rustsimplicity_0_5_free rust_0_5_free

#endif /* SIMPLICITY_SIMPLICITY_ALLOC_H */
