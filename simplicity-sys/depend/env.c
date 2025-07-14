#include <stdlib.h>
#include <stdalign.h>
#include "simplicity/elements/env.h"
#include "simplicity/primitive/elements/primitive.h"

const size_t rustsimplicity_0_4_c_sizeof_rawBuffer = sizeof(rawBuffer);
const size_t rustsimplicity_0_4_c_sizeof_rawOutput = sizeof(rawOutput);
const size_t rustsimplicity_0_4_c_sizeof_rawInput = sizeof(rawInput);
const size_t rustsimplicity_0_4_c_sizeof_rawTransaction = sizeof(rawTransaction);
const size_t rustsimplicity_0_4_c_sizeof_rawTapEnv = sizeof(rawTapEnv);
const size_t rustsimplicity_0_4_c_sizeof_txEnv = sizeof(txEnv);

const size_t rustsimplicity_0_4_c_alignof_rawBuffer = alignof(rawBuffer);
const size_t rustsimplicity_0_4_c_alignof_rawOutput = alignof(rawOutput);
const size_t rustsimplicity_0_4_c_alignof_rawInput = alignof(rawInput);
const size_t rustsimplicity_0_4_c_alignof_rawTransaction = alignof(rawTransaction);
const size_t rustsimplicity_0_4_c_alignof_rawTapEnv = alignof(rawTapEnv);
const size_t rustsimplicity_0_4_c_alignof_txEnv = alignof(txEnv);

void rustsimplicity_0_4_c_set_rawBuffer(rawBuffer *result, const unsigned char *buf, unsigned int len)
{
    *result = (rawBuffer){.buf = buf, .len = len};
}

void rustsimplicity_0_4_c_set_rawOutput(rawOutput *result, const unsigned char *asset, const unsigned char *value, const unsigned char *nonce, const rawBuffer *scriptPubKey,
                     const rawBuffer *surjectionProof, const rawBuffer *rangeProof)
{
    *result = (rawOutput){.asset = asset, .value = value, .nonce = nonce, .scriptPubKey = *scriptPubKey, .surjectionProof = *surjectionProof, .rangeProof = *rangeProof};
}

void rustsimplicity_0_4_c_set_rawInput(rawInput *result, const rawBuffer *annex, const unsigned char *pegin, const rawBuffer *scriptSig,
                    const unsigned char *prevTxid, unsigned int prevIx,
                    const unsigned char *asset, const unsigned char *value, const rawBuffer *scriptPubKey,
                    unsigned int sequence,
                    const unsigned char *blindingNonce, const unsigned char *assetEntropy, const unsigned char *amount, const unsigned char *inflationKeys,
                    const rawBuffer *amountRangePrf, const rawBuffer *inflationKeysRangePrf)
{
    *result = (rawInput){.annex = annex, .scriptSig = *scriptSig, .prevTxid = prevTxid, .pegin = pegin, .issuance = {.blindingNonce = blindingNonce, .assetEntropy = assetEntropy, .amount = amount, .inflationKeys = inflationKeys, .amountRangePrf = *amountRangePrf, .inflationKeysRangePrf = *inflationKeysRangePrf}, .txo = {.asset = asset, .value = value, .scriptPubKey = *scriptPubKey}, .prevIx = prevIx, .sequence = sequence};
}

void rustsimplicity_0_4_c_set_rawTransaction(rawTransaction *result, unsigned int version,
                          const unsigned char *txid,
                          const rawInput *input, unsigned int numInputs,
                          const rawOutput *output, unsigned int numOutputs,
                          unsigned int lockTime)
{
    *result = (rawTransaction){
        .version = version,
        .txid = txid,
        .input = input,
        .numInputs = numInputs,
        .output = output,
        .numOutputs = numOutputs,
        .lockTime = lockTime,
    };
}

void rustsimplicity_0_4_c_set_rawTapEnv(rawTapEnv *result, const unsigned char *controlBlock, unsigned char pathLen, const unsigned char *scriptCMR)
{
    *result = (rawTapEnv){.controlBlock = controlBlock, .pathLen = pathLen, .scriptCMR = scriptCMR};
}

void rustsimplicity_0_4_c_set_txEnv(txEnv *result, const transaction *tx, const tapEnv *taproot, const unsigned char *genesisHash, unsigned int ix)
{
    sha256_midstate genesis;
    sha256_toMidstate(genesis.s, genesisHash);
    *result = rustsimplicity_0_4_build_txEnv(tx, taproot, &genesis, ix);
}
