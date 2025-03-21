#ifndef SIMPLICITY_SIMPLICITY_ALLOC_H
#define SIMPLICITY_SIMPLICITY_ALLOC_H

#include <stdlib.h>

/* Declare Rust functions so the compiler can handle them.
 * The linker will include the functions from Rust.
 */
extern void* rust_0_3_malloc(size_t size);
extern void* rust_0_3_calloc(size_t num, size_t size);
extern void rust_0_3_free(void* ptr);

/* Allocate with rust_0_3_malloc. */
#define simplicity_malloc rust_0_3_malloc

/* Allocate+zero initialize with rust_0_3_calloc. */
#define simplicity_calloc rust_0_3_calloc

/* Deallocate with rust_0_3_free. */
#define simplicity_free rust_0_3_free

#endif /* SIMPLICITY_SIMPLICITY_ALLOC_H */
