#ifndef WRAPPERS_H
#define WRAPPERS_H
#include "simplicity/frame.h"

#include <stdalign.h>

const size_t c_sizeof_UWORD = sizeof(UWORD);
const size_t c_sizeof_frameItem = sizeof(frameItem);

const size_t c_alignof_UWORD = alignof(UWORD);
const size_t c_alignof_frameItem = alignof(frameItem);

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