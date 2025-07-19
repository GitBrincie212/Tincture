use std::simd::{f64x4};
use crate::batch::ColorBatch;

pub enum BatchInstructionSet {
    AddBatches(Vec<ColorBatch>),
    SubBatches(Vec<ColorBatch>),
    TensorBatches(Vec<ColorBatch>),
    LowerAddend(Vec<f64x4>),
    LowerSub(Vec<f64x4>),
    LowerMul(Vec<f64x4>),
    DivScalarToBatch(Vec<f64>),
    NthRootScalarToBatch(Vec<f64>),
}