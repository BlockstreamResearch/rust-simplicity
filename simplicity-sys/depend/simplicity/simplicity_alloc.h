#ifndef SIMPLICITY_SIMPLICITY_ALLOC_H
#define SIMPLICITY_SIMPLICITY_ALLOC_H

#include <stdlib.h>

/* Declare Rust functions so the compiler can handle them.
 * The linker will include the functions from Rust.
 */
extern void* rust_malloc(size_t size);
extern void* rust_calloc(size_t num, size_t size);
extern void rust_free(void* ptr);

/* Allocate with rust_malloc. */
#define simplicity_malloc rust_malloc

/* Allocate+zero initialize with rust_calloc. */
#define simplicity_calloc rust_calloc

/* Deallocate with rust_free. */
#define simplicity_free rust_free

#endif /* SIMPLICITY_SIMPLICITY_ALLOC_H */
