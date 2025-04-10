#include "typeSkipTest.h"

/* A length-prefixed encoding of the following Simplicity program:
 *     witness (runIdentity (getValue (return True))) >>> mn >>> unit
 *      where
 *       l1 = take l0 &&& drop l0
 *       l2 = take l1 &&& drop l1
 *       l3 = take l2 &&& drop l2
 *       ltop = l3
 *       m1 = copair l3 l3
 *       m2 = take l1 &&& drop m1
 *       m3 = take m2 &&& drop l2
 *       m4 = take l3 &&& drop m3
 *       m5 = copair (injl m4) (injr ltop)
 *       m6 = take l1 &&& drop m5
 *       m7 = take m6 &&& drop l2
 *       m8 = take l3 &&& drop m7
 *       n1 = copair l3 l3
 *       n2 = take n1 &&& drop l1
 *       n3 = take l2 &&& drop n2
 *       n4 = take n3 &&& drop l3
 *       n5 = copair (injl ltop) (injr n4)
 *       n6 = take n5 &&& drop l0
 *       n7 = take l1 &&& drop n6
 *       n8 = take n7 &&& drop l2
 *       mn = copair (injl m8) (injr n8)
 */
const unsigned char typeSkipTest[] = {
  0xe3, 0x77, 0x42, 0x45, 0x0c, 0xb5, 0xf0, 0xec, 0x6e, 0x61, 0xbb, 0x47, 0xa7, 0x00, 0x47, 0x8f, 0xc4, 0xd3, 0x00, 0x4b,
  0x65, 0x5d, 0xb7, 0x70, 0x69, 0xf6, 0x7a, 0x99, 0x73, 0x8f, 0x03, 0x8f, 0x08, 0x94, 0x6b, 0x10, 0x8a, 0x83, 0x0f, 0x05,
  0x06, 0x1e, 0x0a, 0x0c, 0x36, 0x84, 0x24, 0x50, 0xc3, 0xff, 0xf7, 0x0a, 0xf7, 0xbb, 0x16, 0x15, 0x3b, 0x42, 0x52, 0x00,
  0x81, 0xa2, 0xc0, 0x81, 0xb1, 0x19, 0xa7, 0x90, 0xd6, 0x0c, 0x30, 0x8b, 0xd2, 0x37, 0xb6, 0xd8, 0x47, 0x5e, 0x0b, 0x4a,
  0x66, 0xcd, 0x09, 0x14, 0x37, 0x9e, 0x40, 0x52, 0x60, 0x02, 0xb3, 0xf3, 0x3f, 0xc5, 0x2a, 0xba, 0xbd, 0x6f, 0x3d, 0x92,
  0x1d, 0xc9, 0x30, 0x79, 0xe6, 0xe1, 0x2b, 0x6f, 0xdb, 0x7e, 0x6d, 0x62, 0x7f, 0x23, 0xc7, 0x9a, 0x81, 0xb6, 0x1f, 0x68,
  0xa0, 0xc0, 0xe2, 0x06, 0x63, 0x0f, 0xc1, 0x85, 0x07, 0x0b, 0x78, 0xc3, 0xea, 0x14, 0x14, 0x60, 0x78, 0x30, 0x1c, 0x20,
  0x70, 0xb8, 0x68, 0xc3, 0xf1, 0x20, 0xa0, 0xe1, 0x71, 0x18, 0xa3, 0x03, 0xc5, 0xc0, 0x71, 0x81, 0x20, 0x80, 0x71, 0xb0
};

const size_t sizeof_typeSkipTest = sizeof(typeSkipTest);
const unsigned char typeSkipTest_witness[] = {
  0xe0
};

const size_t sizeof_typeSkipTest_witness = sizeof(typeSkipTest_witness);

/* The commitment Merkle root of the above typeSkipTest Simplicity expression. */
const uint32_t typeSkipTest_cmr[] = {
  0x2a791cd8u, 0xf1e2beeau, 0x883e53f2u, 0xce36db2bu, 0x246b3156u, 0xcc40f91bu, 0xb2f59059u, 0xb601ac4au
};

/* The identity hash of the root of the above typeSkipTest Simplicity expression. */
const uint32_t typeSkipTest_ihr[] = {
  0xbadac773u, 0x19e9cabau, 0x7fe49174u, 0x54d0e25eu, 0x7d4c4a7eu, 0x4867c392u, 0x20bf409au, 0xc6e6bf10u
};

/* The annotated Merkle root of the above typeSkipTest Simplicity expression. */
const uint32_t typeSkipTest_amr[] = {
  0xbf18694du, 0x9b6a4b10u, 0xe4facc4du, 0xf8718387u, 0x2998f5dau, 0x1b0adc45u, 0x1f58fc4du, 0x924de2c9u
};

/* The cost of the above typeSkipTest Simplicity expression in milli weight units. */
const ubounded typeSkipTest_cost = 13720;
