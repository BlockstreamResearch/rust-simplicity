#include <stdlib.h>
#include "simplicity/elements/env.h"
#include "simplicity/primitive/elements/primitive.h"

const size_t c_sizeof_rawBuffer = sizeof(rawBuffer);
const size_t c_sizeof_rawOutput = sizeof(rawOutput);
const size_t c_sizeof_rawInput = sizeof(rawInput);
const size_t c_sizeof_rawTransaction = sizeof(rawTransaction);
const size_t c_sizeof_rawTapEnv = sizeof(rawTapEnv);
const size_t c_sizeof_txEnv = sizeof(txEnv);

void c_set_rawBuffer(rawBuffer *result, const unsigned char *buf, unsigned int len)
{
    *result = (rawBuffer){.buf = buf, .len = len};
}

void c_set_rawOutput(rawOutput *result, const unsigned char *asset, const unsigned char *value, const unsigned char *nonce, const rawBuffer *scriptPubKey,
                     const rawBuffer *surjectionProof, const rawBuffer *rangeProof)
{
    *result = (rawOutput){.asset = asset, .value = value, .nonce = nonce, .scriptPubKey = *scriptPubKey, .surjectionProof = *surjectionProof, .rangeProof = *rangeProof};
}

void c_set_rawInput(rawInput *result, const rawBuffer *annex, const unsigned char *pegin, const rawBuffer *scriptSig,
                    const unsigned char *prevTxid, unsigned int prevIx,
                    const unsigned char *asset, const unsigned char *value, const rawBuffer *scriptPubKey,
                    unsigned int sequence,
                    const unsigned char *blindingNonce, const unsigned char *assetEntropy, const unsigned char *amount, const unsigned char *inflationKeys,
                    const rawBuffer *amountRangePrf, const rawBuffer *inflationKeysRangePrf)
{
    *result = (rawInput){.annex = annex, .scriptSig = *scriptSig, .prevTxid = prevTxid, .pegin = pegin, .issuance = {.blindingNonce = blindingNonce, .assetEntropy = assetEntropy, .amount = amount, .inflationKeys = inflationKeys, .amountRangePrf = *amountRangePrf, .inflationKeysRangePrf = *inflationKeysRangePrf}, .txo = {.asset = asset, .value = value, .scriptPubKey = *scriptPubKey}, .prevIx = prevIx, .sequence = sequence};
}

void c_set_rawTransaction(rawTransaction *result, unsigned int version,
                          const rawInput *input, unsigned int numInputs,
                          const rawOutput *output, unsigned int numOutputs,
                          unsigned int lockTime)
{
    *result = (rawTransaction){
        .version = version,
        .input = input,
        .numInputs = numInputs,
        .output = output,
        .numOutputs = numOutputs,
        .lockTime = lockTime,
    };
}

void c_set_rawTapEnv(rawTapEnv *result, const unsigned char *controlBlock, unsigned char branchLen, const unsigned char *scriptCMR)
{
    *result = (rawTapEnv){.controlBlock = controlBlock, .branchLen = branchLen, .scriptCMR = scriptCMR};
}

void c_set_txEnv(txEnv *result, const transaction *tx, const tapEnv *taproot, const unsigned char *genesisHash, unsigned int ix)
{
    sha256_midstate genesis;
    sha256_toMidstate(genesis.s, genesisHash);
    *result = build_txEnv(tx, taproot, &genesis, ix);
}

void c_free_tapEnv(tapEnv *env)
{
    free(env);
}

void c_free_transaction(transaction *tx)
{
    free(tx);
}
