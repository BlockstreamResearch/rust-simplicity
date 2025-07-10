#include <stdlib.h>
#include <stdalign.h>
#include "simplicity/elements/env.h"
#include "simplicity/elements/txEnv.h"
#include "simplicity/elements/primitive.h"

typedef rawElementsBuffer rawBuffer;
typedef rawElementsBuffer rawBuffer;
typedef rawElementsOutput rawOutput;
typedef rawElementsInput rawInput;
typedef rawElementsTransaction rawTransaction;
typedef rawElementsTapEnv rawTapEnv;
 
const size_t rustsimplicity_0_5_c_sizeof_rawElementsBuffer = sizeof(rawElementsBuffer);
const size_t rustsimplicity_0_5_c_sizeof_rawElementsOutput = sizeof(rawElementsOutput);
const size_t rustsimplicity_0_5_c_sizeof_rawElementsInput = sizeof(rawElementsInput);
const size_t rustsimplicity_0_5_c_sizeof_rawElementsTransaction = sizeof(rawElementsTransaction);
const size_t rustsimplicity_0_5_c_sizeof_rawElementsTapEnv = sizeof(rawElementsTapEnv);
const size_t rustsimplicity_0_5_c_sizeof_txEnv = sizeof(txEnv);

const size_t rustsimplicity_0_5_c_alignof_rawElementsBuffer = alignof(rawElementsBuffer);
const size_t rustsimplicity_0_5_c_alignof_rawElementsOutput = alignof(rawElementsOutput);
const size_t rustsimplicity_0_5_c_alignof_rawElementsInput = alignof(rawElementsInput);
const size_t rustsimplicity_0_5_c_alignof_rawElementsTransaction = alignof(rawElementsTransaction);
const size_t rustsimplicity_0_5_c_alignof_rawElementsTapEnv = alignof(rawElementsTapEnv);
const size_t rustsimplicity_0_5_c_alignof_txEnv = alignof(txEnv);

void rustsimplicity_0_5_c_set_rawElementsBuffer(rawElementsBuffer *result, const unsigned char *buf, unsigned int len)
{
    *result = (rawElementsBuffer){.buf = buf, .len = len};
}

void rustsimplicity_0_5_c_set_rawElementsOutput(rawElementsOutput *result, const unsigned char *asset, const unsigned char *value, const unsigned char *nonce, const rawElementsBuffer *scriptPubKey,
                     const rawElementsBuffer *surjectionProof, const rawElementsBuffer *rangeProof)
{
    *result = (rawElementsOutput){.asset = asset, .value = value, .nonce = nonce, .scriptPubKey = *scriptPubKey, .surjectionProof = *surjectionProof, .rangeProof = *rangeProof};
}

void rustsimplicity_0_5_c_set_rawElementsInput(rawElementsInput *result, const rawElementsBuffer *annex, const unsigned char *pegin, const rawElementsBuffer *scriptSig,
                    const unsigned char *prevTxid, unsigned int prevIx,
                    const unsigned char *asset, const unsigned char *value, const rawElementsBuffer *scriptPubKey,
                    unsigned int sequence,
                    const unsigned char *blindingNonce, const unsigned char *assetEntropy, const unsigned char *amount, const unsigned char *inflationKeys,
                    const rawElementsBuffer *amountRangePrf, const rawElementsBuffer *inflationKeysRangePrf)
{
    *result = (rawElementsInput){.annex = annex, .scriptSig = *scriptSig, .prevTxid = prevTxid, .pegin = pegin, .issuance = {.blindingNonce = blindingNonce, .assetEntropy = assetEntropy, .amount = amount, .inflationKeys = inflationKeys, .amountRangePrf = *amountRangePrf, .inflationKeysRangePrf = *inflationKeysRangePrf}, .txo = {.asset = asset, .value = value, .scriptPubKey = *scriptPubKey}, .prevIx = prevIx, .sequence = sequence};
}

void rustsimplicity_0_5_c_set_rawElementsTransaction(rawElementsTransaction *result, unsigned int version,
                          const unsigned char *txid,
                          const rawElementsInput *input, unsigned int numInputs,
                          const rawElementsOutput *output, unsigned int numOutputs,
                          unsigned int lockTime)
{
    *result = (rawElementsTransaction){
        .version = version,
        .txid = txid,
        .input = input,
        .numInputs = numInputs,
        .output = output,
        .numOutputs = numOutputs,
        .lockTime = lockTime,
    };
}

void rustsimplicity_0_5_c_set_rawElementsTapEnv(rawElementsTapEnv *result, const unsigned char *controlBlock, unsigned char pathLen, const unsigned char *scriptCMR)
{
    *result = (rawElementsTapEnv){.controlBlock = controlBlock, .pathLen = pathLen, .scriptCMR = scriptCMR};
}

void rustsimplicity_0_5_c_set_txEnv(txEnv *result, const elementsTransaction *tx, const elementsTapEnv *taproot, const unsigned char *genesisHash, unsigned int ix)
{
    sha256_midstate genesis;
    sha256_toMidstate(genesis.s, genesisHash);
    *result = rustsimplicity_0_5_elements_build_txEnv(tx, taproot, &genesis, ix);
}
