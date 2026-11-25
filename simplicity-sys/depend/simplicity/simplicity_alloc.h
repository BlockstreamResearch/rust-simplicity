#ifndef SIMPLICITY_SIMPLICITY_ALLOC_H
#define SIMPLICITY_SIMPLICITY_ALLOC_H

#include <stdlib.h>

/* Declare Rust functions so the compiler can handle them.
 * The linker will include the functions from Rust.
 */
extern void* rust_0_9_malloc(size_t size);
extern void* rust_0_9_calloc(size_t num, size_t size);
extern void rust_0_9_free(void* ptr);

/* Allocate with rust_0_9_malloc. */
#define rustsimplicity_0_9_malloc rust_0_9_malloc

/* Allocate+zero initialize with rust_0_9_calloc. */
#define rustsimplicity_0_9_calloc rust_0_9_calloc

/* Deallocate with rust_0_9_free. */
#define rustsimplicity_0_9_free rust_0_9_free

#endif /* SIMPLICITY_SIMPLICITY_ALLOC_H */
