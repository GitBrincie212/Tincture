use std::simd::{f64x4};
use crate::color::blending::BlendingMode;

pub enum BatchInstructionSet {
    LowerAddend(Vec<f64x4>),
    LowerSub(Vec<f64x4>),
    LowerMul(Vec<f64x4>),
    DivScalarToBatch(Vec<f64x4>),
    NthRootScalarToBatch(Vec<[f64; 4]>),
    BlendMode(Vec<f64x4>, Vec<BlendingMode>)
}