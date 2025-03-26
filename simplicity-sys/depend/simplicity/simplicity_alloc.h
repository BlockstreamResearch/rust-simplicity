#ifndef SIMPLICITY_SIMPLICITY_ALLOC_H
#define SIMPLICITY_SIMPLICITY_ALLOC_H

#include <stdlib.h>

/* Declare Rust functions so the compiler can handle them.
 * The linker will include the functions from Rust.
 */
extern void* rust_0_4_malloc(size_t size);
extern void* rust_0_4_calloc(size_t num, size_t size);
extern void rust_0_4_free(void* ptr);

/* Allocate with rust_0_4_malloc. */
#define simplicity_malloc rust_0_4_malloc

/* Allocate+zero initialize with rust_0_4_calloc. */
#define simplicity_calloc rust_0_4_calloc

/* Deallocate with rust_0_4_free. */
#define simplicity_free rust_0_4_free

#endif /* SIMPLICITY_SIMPLICITY_ALLOC_H */
