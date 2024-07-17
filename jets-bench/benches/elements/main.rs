use std::sync::Arc;

use criterion::{criterion_group, criterion_main, BatchSize, Criterion};
use elements::confidential;
use rand::rngs::ThreadRng;
use rand::{thread_rng, RngCore};
use simplicity::hashes::{sha256, Hash, HashEngine};
use simplicity::jet::elements::ElementsEnv;
use simplicity::jet::{Elements, Jet};
use simplicity::types;
use simplicity::Value;
use simplicity::{bitcoin, elements};
use simplicity_bench::{
    genesis_pegin, BenchSample, EnvSampling, InputSampling, JetBuffer, JetParams, SimplicityCtx8,
    SimplicityEncode, SimplicityFe, SimplicityGe, SimplicityGej, SimplicityPoint, SimplicityScalar,
};

const NUM_RANDOM_SAMPLES: usize = 100;

/// Number of inputs and outputs in the tx
/// RATIONALE: One input to spend a asset, one input to pay fees, one input
/// to interact with the contract
/// Two outputs for asset(dest, change), two outputs for bitcoin (dest, change)
/// One output for fees, and one output for the contract
///
/// Why these constants don't matter: (FOR NOW)?
///
/// All jets actually use some pre-computed cache which does not depend on
/// the number of inputs and outputs. We have already pre-calculated all the
/// inputs and outputs. There is no iteration over jets, only
pub const NUM_TX_INPUTS: usize = 3;
pub const NUM_TX_OUTPUTS: usize = 6;

/// Worst case env for each jet
enum ElementsBenchEnvType {
    /// None
    None,
    /// Random env, we don't care about issuances, pegins or conf data
    /// These jets use pre-cached data and don't care about how the data
    /// was constructed
    Random,
    /// Jets have worst case on issuance inputs
    Issuance,
    /// Jets have worst case on pegin inputs
    Pegin,
    /// Annex inputs
    Annex,
}

impl ElementsBenchEnvType {
    fn env(&self) -> ElementsEnv<Arc<elements::Transaction>> {
        let env_sampler = match self {
            ElementsBenchEnvType::None => EnvSampling::null(),
            ElementsBenchEnvType::Random | ElementsBenchEnvType::Annex => {
                let selector = rand::random::<usize>() % 4;
                EnvSampling::random(selector)
            }
            ElementsBenchEnvType::Issuance => EnvSampling::issuance(),
            ElementsBenchEnvType::Pegin => EnvSampling::pegin(),
        };
        env_sampler
            .n_inputs(NUM_TX_INPUTS)
            .n_outputs(NUM_TX_OUTPUTS)
            .env()
    }
}

fn jet_arrow(jet: Elements) -> (Arc<types::Final>, Arc<types::Final>) {
    let src_ty = jet.source_ty().to_final();
    let tgt_ty = jet.target_ty().to_final();
    (src_ty, tgt_ty)
}

// Separate out heavy jets to run them more times in our benchmark.
fn is_heavy_jet(jet: Elements) -> bool {
    // Hashes
    match jet {
        Elements::Sha256Iv |
        Elements::Sha256Block |
        Elements::Sha256Ctx8Init |
        Elements::Sha256Ctx8Add1 |
        Elements::Sha256Ctx8Add2 |
        Elements::Sha256Ctx8Add4 |
        Elements::Sha256Ctx8Add8 |
        Elements::Sha256Ctx8Add16 |
        Elements::Sha256Ctx8Add32 |
        Elements::Sha256Ctx8Add64 |
        Elements::Sha256Ctx8Add128 |
        Elements::Sha256Ctx8Add256 |
        Elements::Sha256Ctx8Add512 |
        Elements::Sha256Ctx8AddBuffer511 |
        Elements::Sha256Ctx8Finalize |
        // Jets for secp FE
        Elements::FeNormalize |
        Elements::FeNegate |
        Elements::FeAdd |
        Elements::FeSquare |
        Elements::FeMultiply |
        Elements::FeMultiplyBeta |
        Elements::FeInvert |
        Elements::FeSquareRoot |
        Elements::FeIsZero |
        Elements::FeIsOdd |
        // Jets for secp scalars
        Elements::ScalarNormalize |
        Elements::ScalarNegate |
        Elements::ScalarAdd |
        Elements::ScalarSquare |
        Elements::ScalarMultiply |
        Elements::ScalarMultiplyLambda |
        Elements::ScalarInvert |
        Elements::ScalarIsZero |
        // Jets for secp gej points
        Elements::GejInfinity |
        Elements::GejRescale |
        Elements::GejNormalize |
        Elements::GejNegate |
        Elements::GeNegate |
        Elements::GejDouble |
        Elements::GejAdd |
        Elements::GejGeAddEx |
        Elements::GejGeAdd |
        Elements::GejIsInfinity |
        Elements::GejXEquiv |
        Elements::GejYIsOdd |
        Elements::GejIsOnCurve |
        // Other jets
        Elements::GeIsOnCurve |
        Elements::Scale |
        Elements::Generate |
        Elements::LinearCombination1 |
        Elements::LinearVerify1 |
        Elements::Decompress |
        Elements::PointVerify1 |
        // Signature jets
        Elements::CheckSigVerify |
        Elements::Bip0340Verify  => true,
        _ => false,
    }
}

#[rustfmt::skip]
fn bench(c: &mut Criterion) {
    // Sanity Check: This should never really fail, but still good to do
    if !simplicity::ffi::c_jets::sanity_checks() {
        panic!("Sanity checks failed");
    }

    // Initialize set of all jets
    simplicity_bench::check_all_jets::initialize();

    let mut rng = ThreadRng::default();

    fn eq_256() -> Arc<Value> {
        let v  = rand::random::<[u8; 32]>();
        Value::prod(Value::u256_from_slice(&v), Value::u256_from_slice(&v))
    }

    fn value_64_bytes() -> Arc<Value> {
        let (a, b) = (rand::random::<[u8; 32]>(), rand::random::<[u8; 32]>());
        Value::prod(Value::u256_from_slice(&a), Value::u256_from_slice(&b))
    }

    fn value_128_bytes() -> Arc<Value> {
        Value::prod(value_64_bytes(), value_64_bytes())
    }

    fn value_256_bytes() -> Arc<Value> {
        Value::prod(value_128_bytes(), value_128_bytes())
    }

    fn value_512_bytes() -> Arc<Value> {
        Value::prod(value_256_bytes(), value_256_bytes())
    }

    fn ctx8_add_n(n: usize) -> Arc<Value> {
        let v = match n {
            1 => Value::u8(rand::random::<u8>()),
            2 => Value::u16(rand::random::<u16>()),
            4 => Value::u32(rand::random::<u32>()),
            8 => Value::u64(rand::random::<u64>()),
            16 => {
                let (a, b) = (rand::random::<u64>(), rand::random::<u64>());
                Value::prod(Value::u64(a), Value::u64(b))
            }
            32 => {
                let v = rand::random::<[u8; 32]>();
                Value::u256_from_slice(&v)
            }
            64 => value_64_bytes(),
            128 => value_128_bytes(),
            256 => value_256_bytes(),
            512 => value_512_bytes(),
            511 => {
                // Worst case when all bytes are present for ctx8 < 512
                let mut v = [0u8; 511];
                thread_rng().fill_bytes(&mut v);
                simplicity_bench::var_len_buf_from_slice(&v, 8).unwrap()
            }
            _ => unreachable!(),
        };
        let ctx8 = SimplicityCtx8::with_len(511).value();
        Value::prod(ctx8, v)
    }

    fn sequence() -> Arc<Value> {
        let v = rand::random::<u32>();
        // set the first bit to zero
        Value::u32(v & !(1 << 31))
    }

    fn fe_pair() -> Arc<Value> {
        let (a, b) = (SimplicityFe::sample().value(), SimplicityFe::sample().value());
        Value::prod(a, b)
    }

    fn scalar_pair() -> Arc<Value> {
        let (a, b) = (SimplicityScalar::sample().value(), SimplicityScalar::sample().value());
        Value::prod(a, b)
    }

    fn gej_fe_pair() -> Arc<Value> {
        let (a, b) = (SimplicityGej::sample().value(), SimplicityFe::sample().value());
        Value::prod(a, b)
    }

    fn fe_gej_pair() -> Arc<Value> {
        let (a, b) = (SimplicityFe::sample().value(), SimplicityGej::sample().value());
        Value::prod(a, b)
    }

    fn gej_pair() -> Arc<Value> {
        let (a, b) = (SimplicityGej::sample().value(), SimplicityGej::sample().value());
        Value::prod(a, b)
    }

    fn gej_ge_pair() -> Arc<Value> {
        let (a, b) = (SimplicityGej::sample().value(), SimplicityGe::sample().value());
        Value::prod(a, b)
    }

    fn scalar_gej_pair() -> Arc<Value> {
        let (a, b) = (SimplicityScalar::sample().value(), SimplicityGej::sample().value());
        Value::prod(a, b)
    }

    fn scalar_gej_scalar_pair() -> Arc<Value> {
        let (a, b, c) = (
            SimplicityScalar::sample().value(),
            SimplicityGej::sample().value(),
            SimplicityScalar::sample().value(),
        );
        Value::prod(Value::prod(a, b), c)
    }

    fn linear_verify() -> Arc<Value> {
        let (a, b, c, d) = (
            SimplicityScalar::sample().value(),
            SimplicityGe::sample().value(),
            SimplicityScalar::sample().value(),
            SimplicityGe::sample().value(),
        );
        Value::prod(Value::prod(Value::prod(a, b), c), d)
    }

    fn point_verify() -> Arc<Value> {
        let (a, b, c, d) = (
            SimplicityScalar::sample().value(),
            SimplicityPoint::sample().value(),
            SimplicityScalar::sample().value(),
            SimplicityPoint::sample().value(),
        );
        Value::prod(Value::prod(Value::prod(a, b), c), d)
    }

    fn bip_0340() -> Arc<Value> {
        let secp_ctx = bitcoin::secp256k1::Secp256k1::new();
        let keypair = bitcoin::key::Keypair::new(&secp_ctx, &mut thread_rng());
        let xpk = bitcoin::key::XOnlyPublicKey::from_keypair(&keypair);

        let msg = bitcoin::secp256k1::Message::from_digest_slice(&rand::random::<[u8; 32]>()).unwrap();
        let sig = secp_ctx.sign_schnorr(&msg, &keypair);
        let xpk_value = Value::u256_from_slice(&xpk.0.serialize());
        let sig_value = Value::u512_from_slice(sig.as_ref());
        let msg_value = Value::u256_from_slice(&msg[..]);
        assert!(secp_ctx.verify_schnorr(&sig, &msg, &xpk.0).is_ok());
        Value::prod(Value::prod(xpk_value, msg_value), sig_value)
    }

    fn tagged_hash(tag: &[u8], msg_block: [u8; 64]) -> sha256::Hash {
        let tag_hash = sha256::Hash::hash(tag);
        let block = [tag_hash.to_byte_array(), tag_hash.to_byte_array()].concat();
        let mut engine = sha256::Hash::engine();
        engine.input(&block);
        engine.input(&msg_block);

        sha256::Hash::from_engine(engine)
    }

    fn check_sig_verify() -> Arc<Value> {
        let secp_ctx = bitcoin::secp256k1::Secp256k1::signing_only();
        let keypair = bitcoin::key::Keypair::new(&secp_ctx, &mut thread_rng());
        let xpk = bitcoin::key::XOnlyPublicKey::from_keypair(&keypair);

        let msg = [0xab; 64];
        let hashed_msg = tagged_hash(b"Simplicity-Draft\x1fSignature", msg);
        let hashed_msg = bitcoin::secp256k1::Message::from(hashed_msg);
        let sig = secp_ctx.sign_schnorr(&hashed_msg, &keypair);
        let xpk_value = Value::u256_from_slice(&xpk.0.serialize());
        let sig_value = Value::u512_from_slice(sig.as_ref());
        let msg_value = Value::u512_from_slice(&msg[..]);
        Value::prod(Value::prod(xpk_value, msg_value), sig_value)
    }

    // Worst case for eq should be when all bytes are the same
    let arr = [
        // Bit logics
        (Elements::Verify, InputSampling::Random),
        // low
        (Elements::Low8, InputSampling::Random),
        (Elements::Low16, InputSampling::Random),
        (Elements::Low32, InputSampling::Random),
        (Elements::Low64, InputSampling::Random),
        // high
        (Elements::High8, InputSampling::Random),
        (Elements::High16, InputSampling::Random),
        (Elements::High32, InputSampling::Random),
        (Elements::High64, InputSampling::Random),
        // complement
        (Elements::Complement8, InputSampling::Random),
        (Elements::Complement16, InputSampling::Random),
        (Elements::Complement32, InputSampling::Random),
        (Elements::Complement64, InputSampling::Random),
        // and
        (Elements::And8, InputSampling::Random),
        (Elements::And16, InputSampling::Random),
        (Elements::And32, InputSampling::Random),
        (Elements::And64, InputSampling::Random),
        // or
        (Elements::Or8, InputSampling::Random),
        (Elements::Or16, InputSampling::Random),
        (Elements::Or32, InputSampling::Random),
        (Elements::Or64, InputSampling::Random),
        // xor
        (Elements::Xor8, InputSampling::Random),
        (Elements::Xor16, InputSampling::Random),
        (Elements::Xor32, InputSampling::Random),
        (Elements::Xor64, InputSampling::Random),
        // maj
        (Elements::Maj8, InputSampling::Random),
        (Elements::Maj16, InputSampling::Random),
        (Elements::Maj32, InputSampling::Random),
        (Elements::Maj64, InputSampling::Random),
        // xor xor
        (Elements::XorXor8, InputSampling::Random),
        (Elements::XorXor16, InputSampling::Random),
        (Elements::XorXor32, InputSampling::Random),
        (Elements::XorXor64, InputSampling::Random),
        // ch
        (Elements::Ch8, InputSampling::Random),
        (Elements::Ch16, InputSampling::Random),
        (Elements::Ch32, InputSampling::Random),
        (Elements::Ch64, InputSampling::Random),
        // some
        (Elements::Some8, InputSampling::Random),
        (Elements::Some16, InputSampling::Random),
        (Elements::Some32, InputSampling::Random),
        (Elements::Some64, InputSampling::Random),
        // all
        (Elements::All8, InputSampling::Random),
        (Elements::All16, InputSampling::Random),
        (Elements::All32, InputSampling::Random),
        (Elements::All64, InputSampling::Random),
        // one
        (Elements::One8, InputSampling::Random),
        (Elements::One16, InputSampling::Random),
        (Elements::One32, InputSampling::Random),
        (Elements::One64, InputSampling::Random),
        // eq, just sample random values. It is possible
        // that worst case is possible when both
        // numbers are same. This is small cost jet
        // and we don't care as much here. Also, likely for
        // x86_64, upto eq64 it would be one instruction.
        // we do test eq_256 separately.
        (Elements::Eq8, InputSampling::Random),
        (Elements::Eq16, InputSampling::Random),
        (Elements::Eq32, InputSampling::Random),
        (Elements::Eq64, InputSampling::Random),
        (Elements::Eq256, InputSampling::Custom(Arc::new(eq_256))),
        // Arithmetic
        // add
        (Elements::Add8, InputSampling::Random),
        (Elements::Add16, InputSampling::Random),
        (Elements::Add32, InputSampling::Random),
        (Elements::Add64, InputSampling::Random),
        // full add
        (Elements::FullAdd8, InputSampling::Random),
        (Elements::FullAdd16, InputSampling::Random),
        (Elements::FullAdd32, InputSampling::Random),
        (Elements::FullAdd64, InputSampling::Random),
        // full increment
        (Elements::FullIncrement8, InputSampling::Random),
        (Elements::FullIncrement16, InputSampling::Random),
        (Elements::FullIncrement32, InputSampling::Random),
        (Elements::FullIncrement64, InputSampling::Random),
        // increment
        (Elements::Increment8, InputSampling::Random),
        (Elements::Increment16, InputSampling::Random),
        (Elements::Increment32, InputSampling::Random),
        (Elements::Increment64, InputSampling::Random),
        // subtract
        (Elements::Subtract8, InputSampling::Random),
        (Elements::Subtract16, InputSampling::Random),
        (Elements::Subtract32, InputSampling::Random),
        (Elements::Subtract64, InputSampling::Random),
        // negate
        (Elements::Negate8, InputSampling::Random),
        (Elements::Negate16, InputSampling::Random),
        (Elements::Negate32, InputSampling::Random),
        (Elements::Negate64, InputSampling::Random),
        // full decrement
        (Elements::FullDecrement8, InputSampling::Random),
        (Elements::FullDecrement16, InputSampling::Random),
        (Elements::FullDecrement32, InputSampling::Random),
        (Elements::FullDecrement64, InputSampling::Random),
        // multiply
        (Elements::Multiply8, InputSampling::Random),
        (Elements::Multiply16, InputSampling::Random),
        (Elements::Multiply32, InputSampling::Random),
        (Elements::Multiply64, InputSampling::Random),
        // full multiply
        (Elements::FullMultiply8, InputSampling::Random),
        (Elements::FullMultiply16, InputSampling::Random),
        (Elements::FullMultiply32, InputSampling::Random),
        (Elements::FullMultiply64, InputSampling::Random),
        // is zero
        (Elements::IsZero8, InputSampling::Random),
        (Elements::IsZero16, InputSampling::Random),
        (Elements::IsZero32, InputSampling::Random),
        (Elements::IsZero64, InputSampling::Random),
        // is one
        (Elements::IsOne8, InputSampling::Random),
        (Elements::IsOne16, InputSampling::Random),
        (Elements::IsOne32, InputSampling::Random),
        (Elements::IsOne64, InputSampling::Random),
        // le
        (Elements::Le8, InputSampling::Random),
        (Elements::Le16, InputSampling::Random),
        (Elements::Le32, InputSampling::Random),
        (Elements::Le64, InputSampling::Random),
        // lt
        (Elements::Lt8, InputSampling::Random),
        (Elements::Lt16, InputSampling::Random),
        (Elements::Lt32, InputSampling::Random),
        (Elements::Lt64, InputSampling::Random),
        // min
        (Elements::Min8, InputSampling::Random),
        (Elements::Min16, InputSampling::Random),
        (Elements::Min32, InputSampling::Random),
        (Elements::Min64, InputSampling::Random),
        // max
        (Elements::Max8, InputSampling::Random),
        (Elements::Max16, InputSampling::Random),
        (Elements::Max32, InputSampling::Random),
        (Elements::Max64, InputSampling::Random),
        // median
        (Elements::Median8, InputSampling::Random),
        (Elements::Median16, InputSampling::Random),
        (Elements::Median32, InputSampling::Random),
        (Elements::Median64, InputSampling::Random),
        // div mod
        (Elements::DivMod8, InputSampling::Random),
        (Elements::DivMod16, InputSampling::Random),
        (Elements::DivMod32, InputSampling::Random),
        (Elements::DivMod64, InputSampling::Random),
        // divide
        (Elements::Divide8, InputSampling::Random),
        (Elements::Divide16, InputSampling::Random),
        (Elements::Divide32, InputSampling::Random),
        (Elements::Divide64, InputSampling::Random),
        // modulo
        (Elements::Modulo8, InputSampling::Random),
        (Elements::Modulo16, InputSampling::Random),
        (Elements::Modulo32, InputSampling::Random),
        (Elements::Modulo64, InputSampling::Random),
        // divides
        (Elements::Divides8, InputSampling::Random),
        (Elements::Divides16, InputSampling::Random),
        (Elements::Divides32, InputSampling::Random),
        (Elements::Divides64, InputSampling::Random),

        // Hashes
        (Elements::Sha256Iv, InputSampling::Random),
        (Elements::Sha256Block, InputSampling::Random),
        (Elements::Sha256Ctx8Init, InputSampling::Random),
        (Elements::Sha256Ctx8Add1, InputSampling::Custom(Arc::new(|| ctx8_add_n(1)))),
        (Elements::Sha256Ctx8Add2, InputSampling::Custom(Arc::new(|| ctx8_add_n(2)))),
        (Elements::Sha256Ctx8Add4, InputSampling::Custom(Arc::new(|| ctx8_add_n(4)))),
        (Elements::Sha256Ctx8Add8, InputSampling::Custom(Arc::new(|| ctx8_add_n(8)))),
        (Elements::Sha256Ctx8Add16, InputSampling::Custom(Arc::new(|| ctx8_add_n(16)))),
        (Elements::Sha256Ctx8Add32, InputSampling::Custom(Arc::new(|| ctx8_add_n(32)))),
        (Elements::Sha256Ctx8Add64, InputSampling::Custom(Arc::new(|| ctx8_add_n(64)))),
        (Elements::Sha256Ctx8Add128, InputSampling::Custom(Arc::new(|| ctx8_add_n(128)))),
        (Elements::Sha256Ctx8Add256, InputSampling::Custom(Arc::new(|| ctx8_add_n(256)))),
        (Elements::Sha256Ctx8Add512, InputSampling::Custom(Arc::new(|| ctx8_add_n(512)))),
        (Elements::Sha256Ctx8AddBuffer511, InputSampling::Custom(Arc::new(|| ctx8_add_n(511)))),
        (Elements::Sha256Ctx8Finalize, InputSampling::Custom(Arc::new(|| SimplicityCtx8::with_len(511).value()))),
        // Jets for secp FE
        (Elements::FeNormalize, InputSampling::Custom(Arc::new(|| SimplicityFe::sample().value()))),
        (Elements::FeNegate, InputSampling::Custom(Arc::new(|| SimplicityFe::sample().value()))),
        (Elements::FeAdd, InputSampling::Custom(Arc::new(fe_pair))),
        (Elements::FeSquare, InputSampling::Custom(Arc::new(|| SimplicityFe::sample().value()))),
        (Elements::FeMultiply, InputSampling::Custom(Arc::new(fe_pair))),
        (Elements::FeMultiplyBeta, InputSampling::Custom(Arc::new(|| SimplicityFe::sample().value()))),
        (Elements::FeInvert, InputSampling::Custom(Arc::new(|| SimplicityFe::sample().value()))),
        (Elements::FeSquareRoot, InputSampling::Custom(Arc::new(|| SimplicityFe::sample().value()))), // FIXME: Make this a perfect square
        (Elements::FeIsZero, InputSampling::Custom(Arc::new(|| SimplicityFe::zero().value()))),
        (Elements::FeIsOdd, InputSampling::Custom(Arc::new(|| SimplicityFe::sample().value()))),
        // Jets for secp scalars
        (Elements::ScalarNormalize, InputSampling::Custom(Arc::new(|| SimplicityScalar::sample().value()))),
        (Elements::ScalarNegate, InputSampling::Custom(Arc::new(|| SimplicityScalar::sample().value()))),
        (Elements::ScalarAdd, InputSampling::Custom(Arc::new(scalar_pair))),
        (Elements::ScalarSquare, InputSampling::Custom(Arc::new(|| SimplicityScalar::sample().value()))),
        (Elements::ScalarMultiply, InputSampling::Custom(Arc::new(scalar_pair))),
        (Elements::ScalarMultiplyLambda, InputSampling::Custom(Arc::new(|| SimplicityScalar::sample().value()))),
        (Elements::ScalarInvert, InputSampling::Custom(Arc::new(|| SimplicityScalar::sample().value()))),
        (Elements::ScalarIsZero, InputSampling::Custom(Arc::new(|| SimplicityScalar([0u8; 32]).value()))),
        // Jets for secp gej points
        (Elements::GejInfinity, InputSampling::Random),
        (Elements::GejRescale, InputSampling::Custom(Arc::new(gej_fe_pair))),
        (Elements::GejNormalize, InputSampling::Custom(Arc::new(|| SimplicityGej::sample().value()))),
        (Elements::GejNegate, InputSampling::Custom(Arc::new(|| SimplicityGej::sample().value()))),
        (Elements::GeNegate, InputSampling::Custom(Arc::new(|| SimplicityGe::sample().value()))),
        (Elements::GejDouble, InputSampling::Custom(Arc::new(|| SimplicityGej::sample().value()))),
        (Elements::GejAdd, InputSampling::Custom(Arc::new(gej_pair))),
        (Elements::GejGeAddEx, InputSampling::Custom(Arc::new(gej_ge_pair))),
        (Elements::GejGeAdd, InputSampling::Custom(Arc::new(gej_ge_pair))),
        (Elements::GejIsInfinity, InputSampling::Custom(Arc::new(|| SimplicityGej{ ge: SimplicityGe::sample(), z: SimplicityFe::zero() }.value()))),
        (Elements::GejXEquiv, InputSampling::Custom(Arc::new(fe_gej_pair))),
        (Elements::GejYIsOdd, InputSampling::Custom(Arc::new(|| SimplicityGej::sample().value()))),
        (Elements::GejIsOnCurve, InputSampling::Custom(Arc::new(|| SimplicityGej::sample().value()))),
        // Other jets
        (Elements::GeIsOnCurve, InputSampling::Custom(Arc::new(|| SimplicityGe::sample().value()))),
        (Elements::Scale, InputSampling::Custom(Arc::new(scalar_gej_pair))),
        (Elements::Generate, InputSampling::Custom(Arc::new(|| SimplicityScalar::sample().value()))),
        (Elements::LinearCombination1, InputSampling::Custom(Arc::new(scalar_gej_scalar_pair))),
        (Elements::LinearVerify1, InputSampling::Custom(Arc::new(linear_verify))),
        (Elements::Decompress, InputSampling::Custom(Arc::new(|| SimplicityPoint::sample().value()))),
        (Elements::PointVerify1, InputSampling::Custom(Arc::new(point_verify))),
        // Signature jets
        (Elements::CheckSigVerify, InputSampling::Custom(Arc::new(check_sig_verify))),
        (Elements::Bip0340Verify, InputSampling::Custom(Arc::new(bip_0340))),

        // Timelock parsing jets
        (Elements::ParseLock, InputSampling::Random), // all values take same time
        (Elements::ParseSequence, InputSampling::Custom(Arc::new(sequence))),
    ];
    for (jet, sample) in arr {
        simplicity_bench::check_all_jets::record(jet);

        let (src_ty, tgt_ty) = jet_arrow(jet);

        let mut group = c.benchmark_group(jet.to_string());
        let env = EnvSampling::null().env();
        if is_heavy_jet(jet) {
            group.measurement_time(std::time::Duration::from_secs(5));
        };
        for i in 0..NUM_RANDOM_SAMPLES {
            let params = JetParams::with_rand_aligns(sample.clone());
            // Assumption: `buffer.write` is non-negligible
            let bench_name = format!("{}", i);
            group.bench_with_input(bench_name, &params,|b, params| {
                b.iter_batched(
                    || {
                        let mut buffer = JetBuffer::new(&src_ty, &tgt_ty, params);
                        let (src, dst) = buffer.write(&src_ty, params, &mut rng);
                        (dst, src, &env, buffer)
                    },
                    |(mut dst, src, env, _buffer)| jet.c_jet_ptr()(&mut dst, src, env.c_tx_env()),
                    BatchSize::SmallInput,
                )
            });
        }
        group.finish();
    }

    // SIGHALL jets chapter jets with unit src type
    let jets = [
        // The jets below just read data from pre-computed cache which is fixed
        // regardless of the input data. That is, we just read hashed values
        // from the cache and return them.
        (Elements::SigAllHash, ElementsBenchEnvType::Random),
        (Elements::TxHash, ElementsBenchEnvType::Random),
        (Elements::TapEnvHash, ElementsBenchEnvType::Random),
        (Elements::InputsHash, ElementsBenchEnvType::Random),
        (Elements::OutputsHash, ElementsBenchEnvType::Random),
        (Elements::IssuancesHash, ElementsBenchEnvType::Random),
        (Elements::InputUtxosHash, ElementsBenchEnvType::Random),
        (Elements::OutputAmountsHash, ElementsBenchEnvType::Random),
        (Elements::OutputScriptsHash, ElementsBenchEnvType::Random),
        (Elements::OutputNoncesHash, ElementsBenchEnvType::Random),
        (Elements::OutputRangeProofsHash, ElementsBenchEnvType::Random),
        (Elements::OutputSurjectionProofsHash, ElementsBenchEnvType::Random),
        (Elements::InputOutpointsHash, ElementsBenchEnvType::Random),
        (Elements::InputSequencesHash, ElementsBenchEnvType::Random),
        (Elements::InputAnnexesHash, ElementsBenchEnvType::Random),
        (Elements::InputScriptSigsHash, ElementsBenchEnvType::Random),
        (Elements::IssuanceAssetAmountsHash, ElementsBenchEnvType::Random),
        (Elements::IssuanceTokenAmountsHash, ElementsBenchEnvType::Random),
        (Elements::IssuanceRangeProofsHash, ElementsBenchEnvType::Random),
        (Elements::IssuanceBlindingEntropyHash, ElementsBenchEnvType::Random),
        (Elements::InputAmountsHash, ElementsBenchEnvType::Random),
        (Elements::InputScriptsHash, ElementsBenchEnvType::Random),
        (Elements::TapleafHash, ElementsBenchEnvType::Random),
        (Elements::TappathHash, ElementsBenchEnvType::Random),
        //
        // ------------------------------------
        // Jets with no environment required. But no custom sampling
        (Elements::BuildTapleafSimplicity, ElementsBenchEnvType::None),
        (Elements::BuildTapbranch, ElementsBenchEnvType::None),
        // ------------------------------------
        // Timelock jets
        // No need to specially consider issuances or pegins
        (Elements::CheckLockHeight, ElementsBenchEnvType::Random),
        (Elements::CheckLockTime, ElementsBenchEnvType::Random),
        (Elements::CheckLockDistance, ElementsBenchEnvType::Random),
        (Elements::CheckLockDuration, ElementsBenchEnvType::Random),
        (Elements::TxLockHeight, ElementsBenchEnvType::Random),
        (Elements::TxLockTime, ElementsBenchEnvType::Random),
        (Elements::TxLockDistance, ElementsBenchEnvType::Random),
        (Elements::TxLockDuration, ElementsBenchEnvType::Random),
        (Elements::TxIsFinal, ElementsBenchEnvType::Random),
        // -----------------------------------------
        // Jets with no environment required, but no custom sampling required
        // These jets just perform some calculation
        (Elements::CalculateIssuanceEntropy, ElementsBenchEnvType::None),
        (Elements::CalculateAsset, ElementsBenchEnvType::None),
        (Elements::CalculateExplicitToken, ElementsBenchEnvType::None),
        (Elements::CalculateConfidentialToken, ElementsBenchEnvType::None),
        // Jets with tx introspection
        // Nothing special about issuances or pegins
        (Elements::ScriptCMR, ElementsBenchEnvType::Random),
        (Elements::InternalKey, ElementsBenchEnvType::Random),
        (Elements::CurrentIndex, ElementsBenchEnvType::Random),
        (Elements::NumInputs, ElementsBenchEnvType::Random),
        (Elements::NumOutputs, ElementsBenchEnvType::Random),
        (Elements::LockTime, ElementsBenchEnvType::Random),
        // // -----------------------------------------
        // Current Input
        // Each jet has worst case dependent on whether it is pegin or issuance
        // or none
        (Elements::CurrentPegin, ElementsBenchEnvType::Pegin),
        (Elements::CurrentPrevOutpoint, ElementsBenchEnvType::Random),
        (Elements::CurrentAsset, ElementsBenchEnvType::Random),
        (Elements::CurrentAmount, ElementsBenchEnvType::Random),
        (Elements::CurrentScriptHash, ElementsBenchEnvType::Random),
        (Elements::CurrentSequence, ElementsBenchEnvType::Random),
        // Annex note: We explicitly add annex in inputs
        (Elements::CurrentAnnexHash, ElementsBenchEnvType::Random),
        (Elements::CurrentScriptSigHash, ElementsBenchEnvType::Random),
        (Elements::CurrentReissuanceBlinding, ElementsBenchEnvType::Issuance),
        (Elements::CurrentNewIssuanceContract, ElementsBenchEnvType::Issuance),
        (Elements::CurrentReissuanceEntropy, ElementsBenchEnvType::Issuance),
        (Elements::CurrentIssuanceAssetAmount, ElementsBenchEnvType::Issuance),
        (Elements::CurrentIssuanceTokenAmount, ElementsBenchEnvType::Issuance),
        (Elements::CurrentIssuanceAssetProof, ElementsBenchEnvType::Issuance),
        (Elements::CurrentIssuanceTokenProof, ElementsBenchEnvType::Issuance),
        // -----------------------------------------
        // General tx jets
        (Elements::TapleafVersion, ElementsBenchEnvType::None),
        (Elements::Version, ElementsBenchEnvType::None),
        (Elements::GenesisBlockHash, ElementsBenchEnvType::None),
    ];

    // Elements environment jets
    for (jet, env_sampler) in jets {
        simplicity_bench::check_all_jets::record(jet);

        let (src_ty, tgt_ty) = jet_arrow(jet);
        let env = env_sampler.env();

        let mut group = c.benchmark_group(jet.to_string());
        for i in 0..NUM_RANDOM_SAMPLES {
            let params = JetParams::with_rand_aligns(InputSampling::Random);
            let name = format!("{}", i);
            group.bench_with_input(&name, &params,|b, params| {
                b.iter_batched(
                    || {
                        let mut buffer = JetBuffer::new(&src_ty, &tgt_ty, params);
                        let (src, dst) = buffer.write(&src_ty, params, &mut rng);
                        (dst, src, buffer)
                    },
                    |(mut dst, src, _buffer)| jet.c_jet_ptr()(&mut dst, src, env.c_tx_env()),
                    BatchSize::SmallInput,
                )
            });
        }
        group.finish();
    }

    // Input to outpoint hash jet
    fn outpoint_hash() -> Arc<Value> {
        let ctx8 = SimplicityCtx8::with_len(511).value();
        let genesis_pegin = genesis_pegin();
        let outpoint = elements::OutPoint::sample().value();
        Value::prod(ctx8, Value::prod(genesis_pegin, outpoint))
    }

    fn asset_amount_hash() -> Arc<Value> {
        let ctx8 = SimplicityCtx8::with_len(511).value();
        let asset = confidential::Asset::sample().value();
        let amount = confidential::Value::sample().value();
        Value::prod(ctx8, Value::prod(asset, amount))
    }

    fn nonce_hash() -> Arc<Value> {
        let ctx8 = SimplicityCtx8::with_len(511).value();
        let nonce = confidential::Nonce::sample().value();
        Value::prod(ctx8, nonce)
    }

    fn annex_hash() -> Arc<Value> {
        let ctx8 = SimplicityCtx8::with_len(511).value();
        let annex = if rand::random() {
            Value::sum_r(Value::u256_from_slice(&rand::random::<[u8; 32]>()))
        } else {
            Value::sum_l(Value::unit())
        };
        Value::prod(ctx8, annex)
    }
    let arr: [(Elements, Arc<dyn Fn() -> Arc<Value>>); 4] = [
        (Elements::OutpointHash, Arc::new(&outpoint_hash)),
        (Elements::AssetAmountHash, Arc::new(&asset_amount_hash)),
        (Elements::NonceHash, Arc::new(nonce_hash)),
        (Elements::AnnexHash, Arc::new(annex_hash)),
    ];

    for (jet, inp_fn) in arr {
        simplicity_bench::check_all_jets::record(jet);

        let (src_ty, tgt_ty) = jet_arrow(jet);
        let env = EnvSampling::null().env();

        let mut group = c.benchmark_group(jet.to_string());
        for i in 0..NUM_RANDOM_SAMPLES {
            let params = JetParams::with_rand_aligns(InputSampling::Custom(inp_fn.clone()));
            let name = format!("{}", i);
            group.bench_with_input(&name, &params, |b, params| {
                b.iter_batched(
                    || {
                        // Elements sighash chapter jets with non-unit src type
                        let mut buffer = JetBuffer::new(&src_ty, &tgt_ty, params);
                        let (src, dst) = buffer.write(&src_ty, params, &mut rng);
                        (dst, src, buffer)
                    },
                    |(mut dst, src, _buffer)| jet.c_jet_ptr()(&mut dst, src, env.c_tx_env()),
                    BatchSize::SmallInput,
                )
            });
        }
        group.finish()
    }

    // Operations that use tx input or output index.
    fn index_value(bound: u32) -> Arc<Value> {
        let v = rand::random::<u32>() % bound;
        Value::u32(v)
    }

    #[allow(clippy::enum_variant_names)]
    enum Index {
        // Select the input index 0. This is where we do pegin/issuance/annex etc.
        InputIdx0,
        // Select random input
        Input,
        // Select random output
        Output,
        // Markle branch index
        MarkleBranchIndex,
    }
    // Jets that operate on index
    // arr contains a tuple of three things.
    // arr[i].0 = jet
    // arr[i].1 = The input index to select. Two choices: (input, output)
    // arr[i].2 = The environment type to use.
    //
    // For jets that depend on current index, we have made the environment such that
    // index 0 would be input with pegin/issuance/annex etc.
    // For jets that merely introspect,
    let arr = [
        // Issuance chapter jets with index
        (Elements::Issuance, Index::InputIdx0, ElementsBenchEnvType::Issuance),
        (Elements::IssuanceAsset, Index::InputIdx0, ElementsBenchEnvType::Issuance),
        (Elements::IssuanceToken, Index::InputIdx0, ElementsBenchEnvType::Issuance),
        (Elements::IssuanceEntropy, Index::InputIdx0, ElementsBenchEnvType::Issuance),
        // // Transaction chapter jets with output index
        (Elements::OutputAsset, Index::Output, ElementsBenchEnvType::Random),
        (Elements::OutputAmount, Index::Output, ElementsBenchEnvType::Random),
        (Elements::OutputNonce, Index::Output, ElementsBenchEnvType::Random),
        (Elements::OutputScriptHash, Index::Output, ElementsBenchEnvType::Random),
        (Elements::OutputNullDatum, Index::Output, ElementsBenchEnvType::Random), // I Don't know what this is, and how to test this, presumably related to how pegouts work
        (Elements::OutputSurjectionProof, Index::Output, ElementsBenchEnvType::Random),
        (Elements::OutputRangeProof, Index::Output, ElementsBenchEnvType::Random),
        // // Transaction chapter jets with input index
        (Elements::InputPegin, Index::InputIdx0, ElementsBenchEnvType::Pegin),
        (Elements::InputPrevOutpoint, Index::Input, ElementsBenchEnvType::Random),
        (Elements::InputAsset, Index::Input, ElementsBenchEnvType::Random),
        (Elements::InputAmount, Index::Input, ElementsBenchEnvType::Random),
        (Elements::InputScriptHash, Index::Input, ElementsBenchEnvType::Random),
        (Elements::InputSequence, Index::Input, ElementsBenchEnvType::Random),
        (Elements::InputAnnexHash, Index::InputIdx0, ElementsBenchEnvType::Annex),
        (Elements::InputScriptSigHash, Index::Input, ElementsBenchEnvType::Random),
        (Elements::ReissuanceBlinding, Index::InputIdx0, ElementsBenchEnvType::Issuance),
        (Elements::NewIssuanceContract, Index::InputIdx0, ElementsBenchEnvType::Issuance),
        (Elements::ReissuanceEntropy, Index::InputIdx0, ElementsBenchEnvType::Issuance),
        (Elements::IssuanceAssetAmount, Index::InputIdx0, ElementsBenchEnvType::Issuance),
        (Elements::IssuanceTokenAmount, Index::InputIdx0, ElementsBenchEnvType::Issuance),
        (Elements::IssuanceAssetProof, Index::InputIdx0, ElementsBenchEnvType::Issuance),
        (Elements::IssuanceTokenProof, Index::InputIdx0, ElementsBenchEnvType::Issuance),
        (Elements::Tappath, Index::MarkleBranchIndex, ElementsBenchEnvType::Random),
    ];

    for (jet, index, env_type) in arr {
        simplicity_bench::check_all_jets::record(jet);

        let (src_ty, tgt_ty) = jet_arrow(jet);
        let env = env_type.env();
        let mut group = c.benchmark_group(jet.to_string());

        for i in 0..NUM_RANDOM_SAMPLES {
            // We always select the current input because this is where we
            // are doing issuances/pegins/etc.
            let v = match index {
                Index::InputIdx0 => index_value(1),
                Index::Input => index_value(NUM_TX_INPUTS as u32),
                Index::Output => index_value(NUM_TX_OUTPUTS as u32), // any output
                Index::MarkleBranchIndex => Value::u8(0), // 0 index
            };
            let params = JetParams::with_rand_aligns(InputSampling::Fixed(v));
            let name = format!("{}", i);
            group.bench_with_input(&name, &params, |b, params| {
                b.iter_batched(
                    || {
                        let mut buffer = JetBuffer::new(&src_ty, &tgt_ty, params);
                        let (src, dst) = buffer.write(&src_ty, params, &mut rng);
                        (dst, src, buffer)
                    },
                    |(mut dst, src, _buffer)| jet.c_jet_ptr()(&mut dst, src, env.c_tx_env()),
                    BatchSize::SmallInput,
                )
            });
        }
    }

    simplicity_bench::check_all_jets::check_all_covered();
}

criterion_group! {
    name = benches;
    config = Criterion::default()
        // For simpler benchmarks, we don't need to run for long
        // We care most about secp jets
        .measurement_time(std::time::Duration::from_millis(50))
        .warm_up_time(std::time::Duration::from_millis(50))
        // .sample_size(100)
        // .nresamples(10_000)
        .plotting_backend(criterion::PlottingBackend::None)
        .without_plots();
    targets = bench,
}
criterion_main!(benches);
