use simba::simd::AutoF32x4;
use crate::color::blending::BlendingMode;

pub enum BatchInstructionSet {
    LowerAddend(Vec<AutoF32x4>),
    LowerSub(Vec<AutoF32x4>),
    LowerMul(Vec<AutoF32x4>),
    DivScalarToBatch(Vec<AutoF32x4>),
    NthRootScalarToBatch(Vec<[f32; 4]>),
    BlendMode(Vec<AutoF32x4>, Vec<BlendingMode>)
}