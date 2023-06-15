use std::sync::Arc;

use rand::rngs::ThreadRng;
use rand::Rng;
use simplicity::ffi::c_jets::frame_ffi::c_writeBit;
use simplicity::ffi::CFrameItem;
use simplicity::types::{self, CompleteBound};
use simplicity::Value;

pub fn random_value(ty: &types::Final, rng: &mut ThreadRng) -> Value {
    enum StackItem<'a> {
        Type(&'a types::Final),
        LeftSum,
        RightSum,
        Product,
    }

    let mut call_stack = vec![StackItem::Type(ty)];
    let mut value_stack = Vec::new();

    while let Some(top) = call_stack.pop() {
        match top {
            StackItem::Type(ty) => match &ty.bound() {
                CompleteBound::Unit => value_stack.push(Value::Unit),
                CompleteBound::Sum(left, right) => {
                    if rng.gen() {
                        call_stack.push(StackItem::LeftSum);
                        call_stack.push(StackItem::Type(left));
                    } else {
                        call_stack.push(StackItem::RightSum);
                        call_stack.push(StackItem::Type(right));
                    }
                }
                CompleteBound::Product(left, right) => {
                    call_stack.push(StackItem::Product);
                    call_stack.push(StackItem::Type(right));
                    call_stack.push(StackItem::Type(left));
                }
            },
            StackItem::LeftSum => {
                let left = value_stack.pop().unwrap();
                value_stack.push(Value::sum_l(left));
            }
            StackItem::RightSum => {
                let right = value_stack.pop().unwrap();
                value_stack.push(Value::sum_r(right));
            }
            StackItem::Product => {
                let right = value_stack.pop().unwrap();
                let left = value_stack.pop().unwrap();
                value_stack.push(Value::prod(left, right));
            }
        }
    }

    debug_assert!(value_stack.len() == 1);
    value_stack.pop().unwrap()
}

#[derive(Clone)]
pub enum InputSampling {
    /// Uniform random distribution of bit strings based on encoded values over the source type
    Random,
    /// A given, fixed bit string (whose length is multiple of 8)
    /// Worst-case inputs
    Fixed(Value),
    /// Custom sampling method, read first src type bits from input
    /// Useful for cases where we want to sample inputs according to some distributions
    Custom(Arc<dyn Fn() -> Value>),
}

impl InputSampling {
    pub fn write_sample(
        &self,
        src_frame: &mut CFrameItem,
        src_ty: &types::Final,
        rng: &mut ThreadRng,
    ) {
        let write_bit = |bit: bool| unsafe { c_writeBit(src_frame, bit) };

        match self {
            InputSampling::Random => {
                let value = random_value(src_ty, rng);
                value.do_each_bit(write_bit);
            }
            InputSampling::Fixed(v) => {
                v.do_each_bit(write_bit);
            }
            InputSampling::Custom(gen_bytes) => {
                let value = gen_bytes();
                value.do_each_bit(write_bit);
            }
        }
    }
}
