#ifndef WRAPPERS_H
#define WRAPPERS_H
#include "simplicity/bitstream.h"
#include "simplicity/bitstring.h"
#include "simplicity/dag.h"
#include "simplicity/sha256.h"
#include "simplicity/type.h"
#include "simplicity/bounded.h"
#include "simplicity/frame.h"

#include <stdalign.h>

const uint64_t c_overhead = overhead;

const size_t c_sizeof_uchar = sizeof(unsigned char);
const size_t c_sizeof_int = sizeof(int);
const size_t c_sizeof_uint = sizeof(unsigned int);
const size_t c_sizeof_size_t = sizeof(size_t);
const size_t c_sizeof_uint_fast8_t = sizeof(uint_fast8_t);
const size_t c_sizeof_uint_fast16_t = sizeof(uint_fast16_t);
const size_t c_sizeof_uint_fast32_t = sizeof(uint_fast32_t);
const size_t c_sizeof_uint_fast64_t = sizeof(uint_fast64_t);
const size_t c_sizeof_uint_least32_t = sizeof(uint_least32_t);
const size_t c_sizeof_long_double = sizeof(long double);
const size_t c_sizeof_ubounded = sizeof(ubounded);
const size_t c_sizeof_UWORD = sizeof(UWORD);
const size_t c_sizeof_simplicity_err = sizeof(simplicity_err);
const size_t c_sizeof_frameItem = sizeof(frameItem);
const size_t c_sizeof_bitstream = sizeof(bitstream);
const size_t c_sizeof_bitstring = sizeof(bitstring);
const size_t c_sizeof_tag = sizeof(tag_t);
const size_t c_sizeof_combinator_counters = sizeof(combinator_counters);
const size_t c_sizeof_dag_node = sizeof(dag_node);
const size_t c_sizeof_analyses = sizeof(analyses);
const size_t c_sizeof_sha256_midstate = sizeof(sha256_midstate);
const size_t c_sizeof_typename = sizeof(typeName);
const size_t c_sizeof_type = sizeof(type);

const size_t c_alignof_uchar = alignof(unsigned char);
const size_t c_alignof_int = alignof(int);
const size_t c_alignof_uint = alignof(unsigned int);
const size_t c_alignof_size_t = alignof(size_t);
const size_t c_alignof_uint_fast8_t = alignof(uint_fast8_t);
const size_t c_alignof_uint_fast16_t = alignof(uint_fast16_t);
const size_t c_alignof_uint_fast32_t = alignof(uint_fast32_t);
const size_t c_alignof_uint_fast64_t = alignof(uint_fast64_t);
const size_t c_alignof_uint_least32_t = alignof(uint_least32_t);
const size_t c_alignof_long_double = alignof(long double);
const size_t c_alignof_ubounded = alignof(ubounded);
const size_t c_alignof_UWORD = alignof(UWORD);
const size_t c_alignof_simplicity_err = alignof(simplicity_err);
const size_t c_alignof_frameItem = alignof(frameItem);
const size_t c_alignof_bitstream = alignof(bitstream);
const size_t c_alignof_bitstring = alignof(bitstring);
const size_t c_alignof_tag = alignof(tag_t);
const size_t c_alignof_combinator_counters = alignof(combinator_counters);
const size_t c_alignof_dag_node = alignof(dag_node);
const size_t c_alignof_analyses = alignof(analyses);
const size_t c_alignof_sha256_midstate = alignof(sha256_midstate);
const size_t c_alignof_typename = alignof(typeName);
const size_t c_alignof_type = alignof(type);

void c_initReadFrame(frameItem *frame, size_t n, UWORD *from)
{
    *frame = initReadFrame(n, from);
}

void c_initWriteFrame(frameItem *frame, size_t n, UWORD *from)
{
    *frame = initWriteFrame(n, from);
}

/* Expose readBit. Internal readBit is static inline. */
bool c_readBit(frameItem *frame)
{
    return readBit(frame);
}

/* Expose writeBit. Internal writeBit is static inline. */
void c_writeBit(frameItem *frame, bool bit)
{
    writeBit(frame, bit);
}

void c_forwardBits(frameItem *frame, size_t n)
{
    forwardBits(frame, n);
}

void c_skipBits(frameItem *frame, size_t n)
{
    skipBits(frame, n);
}

#endif
