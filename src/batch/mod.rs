use std::simd::{f64x4, StdFloat};
use std::simd::prelude::SimdFloat;
use std::sync::atomic::{Ordering};
use std::sync::{Arc, RwLock};
use crossbeam::queue::SegQueue;
use pyo3::{pyclass, pymethods};
use pyo3::prelude::*;
use crate::batch::batch_instructions::{BatchInstructionSet};
use crate::color::Color;
use rayon::prelude::*;
use crate::{color_to_packed, extract_rgba_channels_by_type, handle_lower_operation, scalar_to_packed};

mod utils;
mod batch_instructions;

#[repr(C)]
#[pyclass(frozen)]
pub struct ColorBatch {
    pub batch: RwLock<Vec<u32>>,
    pub queue: Arc<SegQueue<BatchInstructionSet>>
}

impl Clone for ColorBatch {
    fn clone(&self) -> Self {
        ColorBatch {
            batch: RwLock::new(self.batch.read().unwrap().clone()),
            queue: Arc::new(SegQueue::new())
        }
    }
}

impl ColorBatch {
    fn exec(&self) {
        let mut batch_write = self.batch.write().unwrap();
        let mut self_lanes = batch_write.par_iter().map(|x| {
            let values = x.to_be_bytes();
            f64x4::from_array([values[0] as f64, values[1] as f64, values[2] as f64, values[3] as f64])
        }).collect::<Vec<_>>();
        while let Some(instruction_type) = self.queue.pop() {
            match instruction_type {
                BatchInstructionSet::AddBatches(values) => {}
                BatchInstructionSet::SubBatches(_) => {}
                BatchInstructionSet::TensorBatches(other) => {}
                BatchInstructionSet::LowerAddend(other) => {
                    handle_lower_operation!(self_lanes, other, +=);
                }
                BatchInstructionSet::LowerSub(other) => {
                    handle_lower_operation!(self_lanes, other, -=);
                }
                BatchInstructionSet::LowerMul(other) => {
                    handle_lower_operation!(self_lanes, other, *=);
                }
                BatchInstructionSet::DivScalarToBatch(_) => {}
                BatchInstructionSet::NthRootScalarToBatch(_) => {}
            }
        }
        *batch_write = self_lanes.into_par_iter().map(|x| {
            let result = x.simd_clamp(f64x4::splat(0f64), f64x4::splat(255f64))
                .round();
            u32::from_be_bytes([result[0] as u8, result[1] as u8, result[2] as u8, result[3] as u8])
        }).collect::<Vec<_>>();
    }
}

#[pymethods]
impl ColorBatch {
    #[new]
    pub fn new(args: Vec<Color>) -> PyResult<Self> {
        Ok(ColorBatch {
            batch: RwLock::new(args.par_iter().map(|x| x.clone().0.load(Ordering::Relaxed)).collect()),
            queue: Arc::new(SegQueue::new())
        })
    }

    #[pyo3(signature = (colors, include_transparency=true))]
    pub fn add(slf: PyRef<'_, Self>, colors: Vec<Color>, include_transparency: bool) -> PyRef<'_, Self> {
        let packed: Vec<f64x4> = color_to_packed!(colors, |x: u32| {
            ((x as u8) & (include_transparency as u8).wrapping_neg()) as f64
        });
        slf.queue.push(BatchInstructionSet::LowerAddend(packed));
        slf
    }

    #[pyo3(signature = (scalars, include_transparency=true))]
    pub fn add_scalar(slf: PyRef<'_, Self>, scalars: Vec<f64>, include_transparency: bool) -> PyRef<'_, Self> {
        let packed: Vec<f64x4> = scalar_to_packed!(scalars, |x| x * f64::from(include_transparency));
        slf.queue.push(BatchInstructionSet::LowerAddend(packed));
        slf
    }

    #[pyo3(signature = (colors, include_transparency=true))]
    pub fn sub(slf: PyRef<'_, Self>, colors: Vec<Color>, include_transparency: bool) -> PyRef<'_, Self> {
        let packed: Vec<f64x4> = color_to_packed!(colors, |x: u32| {
            ((x as u8) & (include_transparency as u8).wrapping_neg()) as f64
        });
        slf.queue.push(BatchInstructionSet::LowerSub(packed));
        slf
    }

    #[pyo3(signature = (scalars, include_transparency=true))]
    pub fn sub_scalar(slf: PyRef<'_, Self>, scalars: Vec<f64>, include_transparency: bool) -> PyRef<'_, Self> {
        let packed: Vec<f64x4> = scalar_to_packed!(scalars, |x| x * f64::from(include_transparency));
        slf.queue.push(BatchInstructionSet::LowerSub(packed));
        slf
    }

    #[pyo3(signature = (colors, include_transparency=true))]
    pub fn mul(slf: PyRef<'_, Self>, colors: Vec<Color>, include_transparency: bool) -> PyRef<'_, Self> {
        let packed: Vec<f64x4> = color_to_packed!(colors, |x: u32| {
            ((x as u8) & (include_transparency as u8).wrapping_neg()) as f64
        });
        slf.queue.push(BatchInstructionSet::LowerSub(packed));
        slf
    }

    #[pyo3(signature = (scalars, include_transparency=true))]
    pub fn mul_scalar(slf: PyRef<'_, Self>, scalars: Vec<f64>, include_transparency: bool) -> PyRef<'_, Self> {
        let packed: Vec<f64x4> = scalar_to_packed!(scalars, |x| x * f64::from(include_transparency));
        slf.queue.push(BatchInstructionSet::LowerSub(packed));
        slf
    }

    pub fn __repr__(&self) -> String {
        format!("ColorBatch({})", String::from(self.batch.read().unwrap().iter().map(|x| {
            let rgba = x.to_be_bytes();
            format!("Color({}, {}, {}, {})", rgba[0], rgba[1], rgba[2], rgba[3])
        }).collect::<Vec<_>>().join(", ")))
    }

    pub fn operate(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
        slf.exec();
        slf
    }
}