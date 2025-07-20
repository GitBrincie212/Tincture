use std::ops::{Add, Div, Mul, Sub};
use std::simd::{f64x4, StdFloat};
use std::simd::prelude::SimdFloat;
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::{Arc, RwLock};
use crossbeam::queue::SegQueue;
use pyo3::{exceptions, pyclass, pymethods};
use pyo3::prelude::*;
use crate::batch::batch_instructions::{BatchInstructionSet};
use crate::color::Color;
use rayon::prelude::*;
use crate::{color_to_packed, create_color, extract_rgba_channels_by_type, handle_lower_operation, scalar_to_packed};

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
                BatchInstructionSet::LowerAddend(other) => {
                    handle_lower_operation!(self_lanes, other, add);
                }
                BatchInstructionSet::LowerSub(other) => {
                    handle_lower_operation!(self_lanes, other, sub);
                }
                BatchInstructionSet::LowerMul(other) => {
                    handle_lower_operation!(self_lanes, other, mul);
                }
                BatchInstructionSet::DivScalarToBatch(other) => {
                    handle_lower_operation!(self_lanes, other, div);
                }

                BatchInstructionSet::NthRootScalarToBatch(other) => {
                    self_lanes
                        .par_iter_mut()
                        .for_each(|lane| {
                            for src in &other {
                                // Fallback to scalar, as its impossible to do with SIMD
                                lane[0] = lane[0].powf(1.0 / src[0]);
                                lane[1] = lane[1].powf(1.0 / src[1]);
                                lane[2] = lane[2].powf(1.0 / src[2]);
                                lane[3] = lane[3].powf(1.0 / src[3]);
                            }
                        });
                }
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

    #[pyo3(signature = (scalars, include_transparency=true))]
    pub fn div_scalar(slf: PyRef<'_, Self>, scalars: Vec<f64>, include_transparency: bool) -> PyRef<'_, Self> {
        let packed: Vec<f64x4> = scalar_to_packed!(scalars, |x| x * f64::from(include_transparency));
        slf.queue.push(BatchInstructionSet::DivScalarToBatch(packed));
        slf
    }

    #[pyo3(signature = (scalars, include_transparency=true))]
    pub fn nth_root_scalar(slf: PyRef<'_, Self>, scalars: Vec<f64>, include_transparency: bool) -> PyRef<'_, Self> {
        let packed: Vec<[f64; 4]> = scalars
            .par_iter()
            .map(|x| [*x, *x, *x, x * f64::from(include_transparency)])
            .collect();
        slf.queue.push(BatchInstructionSet::NthRootScalarToBatch(packed));
        slf
    }

    pub fn __str__(&self) -> String {
        self.__repr__()
    }

    pub fn __repr__(&self) -> String {
        format!("ColorBatch({})", self.batch.read().unwrap().iter().map(|x| {
            let rgba = x.to_be_bytes();
            format!("Color({}, {}, {}, {})", rgba[0], rgba[1], rgba[2], rgba[3])
        }).collect::<Vec<_>>().join(", "))
    }

    fn __getitem__(&self, index: isize) -> PyResult<Color> {
        let read_batch = self.batch.read().unwrap();
        if index < 0 || index >= read_batch.len() as isize {
            return Err(exceptions::PyIndexError::new_err("Color batch index out of range"));
        }
        Ok(create_color!(read_batch[index as usize]))
    }

    fn __setitem__(&self, index: isize, value: Color) -> PyResult<()> {
        let mut write_batch = self.batch.write().unwrap();
        if index < 0 || index >= write_batch.len() as isize {
            return Err(exceptions::PyIndexError::new_err("Color batch index out of range"));
        }
        write_batch[index as usize] = value.0.load(Ordering::Relaxed);
        Ok(())
    }

    pub fn operate(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
        slf.exec();
        slf
    }
}