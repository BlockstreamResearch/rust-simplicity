#ifndef SIMPLICITY_SIMPLICITY_ALLOC_H
#define SIMPLICITY_SIMPLICITY_ALLOC_H

#include <stdlib.h>

/* Declare Rust functions so the compiler can handle them.
 * The linker will include the functions from Rust.
 */
extern void* rust_0_8_malloc(size_t size);
extern void* rust_0_8_calloc(size_t num, size_t size);
extern void rust_0_8_free(void* ptr);

/* Allocate with rust_0_8_malloc. */
#define rustsimplicity_0_8_malloc rust_0_8_malloc

/* Allocate+zero initialize with rust_0_8_calloc. */
#define rustsimplicity_0_8_calloc rust_0_8_calloc

/* Deallocate with rust_0_8_free. */
#define rustsimplicity_0_8_free rust_0_8_free

#endif /* SIMPLICITY_SIMPLICITY_ALLOC_H */
