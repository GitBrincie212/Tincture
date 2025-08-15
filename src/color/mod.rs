use crate::{clerp_impl, extract_rgba_channels};
use std::ops::Rem;
use crate::color::simd_casts::{f32x4_to_u32, u8x4_to_f32x4, u8x4_to_u32};
use simba::simd::{AutoUsizex4, AutoU8x4, AutoF32x4, SimdPartialOrd, SimdValue, SimdComplexField, SimdBool};
use pyo3::exceptions::{PyIndexError, PyValueError, PyZeroDivisionError};
use pyo3::prelude::*;
use pyo3::types::{PyList, PyTuple};
use std::f32::consts::PI as f32_PI;
use std::ffi::c_uint;
use std::hash::{Hash, Hasher};
use std::ops::{Add, Div, Mul, Neg, Sub};
use std::sync::{LazyLock, Mutex};
use std::sync::atomic::{AtomicU32, Ordering};
use pyo3::impl_::callback::IntoPyCallbackOutput;
use rand::prelude::SmallRng;
use rand::{RngCore, SeedableRng};
use crate::{approx_equal_field, create_color, calc_hue_saturation, find_invalid_range};
use crate::{extract_rgb_channel, extract_rgba_channels_by_type};
use crate::{shift_impl, to_decimal_rgba, to_lch, to_oklab, to_unit_rgba, unpack_rgba, to_hsv, interpret_to_hex};
use crate::color::utils::{color_to_color_operation, color_to_scalar_operation, color_to_unknown_operation};

pub mod blending;
pub mod consts;
mod utils;
pub mod simd_casts;

static RNG: LazyLock<Mutex<SmallRng>> = LazyLock::new(|| Mutex::new(SmallRng::from_os_rng()));
const DEG_CONV: f32 = f32_PI / 180f32;


#[repr(C)]
#[pyclass(frozen)]
pub struct Color(pub(crate) AtomicU32);

impl Clone for Color {
    fn clone(&self) -> Self {
        Color(AtomicU32::new(self.0.load(Ordering::Relaxed)))
    }
}

impl IntoPyCallbackOutput<'_, c_uint> for Color {
    #[inline]
    fn convert(self, _: Python<'_>) -> PyResult<c_uint> {
        Ok(self.0.load(Ordering::Relaxed) as c_uint)
    }
}


#[derive(FromPyObject)]
pub enum ColorAccessCode {
    Integer(u8),
    String(String),
}

#[pymethods]
impl Color {
    #[new]
    #[pyo3(signature = (r, g, b, a=255))]
        fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        create_color!(u32::from_be_bytes([r, g, b, a]))
    }

    #[staticmethod]
    pub fn from_srgb(r: u8, g: u8, b: u8) -> PyResult<Color> {
        Ok(create_color!(u32::from_be_bytes([r, g, b, 255])))
    }

    #[staticmethod]
    pub fn from_decimal_rgba(r: f32, g: f32, b: f32, a: f32) -> PyResult<Color> {
        find_invalid_range!(r, "Red");
        find_invalid_range!(b, "Blue");
        find_invalid_range!(g, "Green");
        find_invalid_range!(a, "Alpha");
        Ok(to_unit_rgba!(r, g, b, a))
    }

    #[staticmethod]
    pub fn from_cmyk(c: f32, m: f32, y: f32, k: f32, transparency: f32) -> PyResult<Color> {
        find_invalid_range!(c, "Cyan");
        find_invalid_range!(m, "Magenta");
        find_invalid_range!(y, "Yellow");
        find_invalid_range!(k, "Key (Black)");
        find_invalid_range!(transparency, "Transparency");
        let mut calc = AutoF32x4::new(c, m, y, 0f32)
            .neg()
            .add(AutoF32x4::splat(1f32))
            .mul(AutoF32x4::splat(255f32))
            .mul(AutoF32x4::splat(1f32 - k))
            .simd_round();
        calc.replace(3, transparency * 255.0);
        Ok(create_color!(f32x4_to_u32(calc)))
    }

    #[staticmethod]
    #[pyo3(signature = (x, y, z, transparency=1.0))]
    pub fn from_xyz(x: f32, y: f32, z: f32, transparency: f32) -> PyResult<Color> {
        find_invalid_range!(x, "X", 0.0, 95.047);
        find_invalid_range!(y, "Y", 0.0, 100.0);
        find_invalid_range!(z, "z", 0.0, 108.883);
        find_invalid_range!(transparency, "Transparency");
        let xyz = AutoF32x4::new(x, y, z, 0f32).div(AutoF32x4::splat(100f32));

        let r = xyz.mul(AutoF32x4::new(3.2406f32, -1.5372f32, -0.4986f32, 0f32)).simd_horizontal_sum();
        let g = xyz.mul(AutoF32x4::new(-0.9689f32, 1.8758f32, 0.0415f32, 0f32)).simd_horizontal_sum();
        let b = xyz.mul(AutoF32x4::new(0.0557f32, -0.2040f32, 1.0570f32, 0f32)).simd_horizontal_sum();
        let rgb = AutoF32x4::new(r, g, b, 0f32);
        let converted = rgb.simd_gt(AutoF32x4::splat(0.0031308f32)).if_else(|| {
            rgb.simd_powf(AutoF32x4::splat(0.41666667f32))
                .mul(AutoF32x4::splat(1.055f32))
                .sub(AutoF32x4::splat(0.055f32))
        }, || {rgb.mul(AutoF32x4::splat(12.92f32))}).0;

        Ok(to_unit_rgba!(converted[0], converted[1], converted[2], transparency))
    }

    #[staticmethod]
    pub fn from_lch(l: f32, c: f32, h: i16, transparency: f32) -> PyResult<Color> {
        find_invalid_range!(l, "L", 0.0, 100.0);
        find_invalid_range!(c, "C", 0.0, 200.0);
        find_invalid_range!(transparency, "Transparency");
        let mut h_scoped = (h as f32).rem_euclid(360.0);
        h_scoped *= DEG_CONV;
        let a = h_scoped.cos() * c;
        let b = h_scoped.sin() * c;
        Ok(Color::from_oklab(l, a, b, transparency))
    }

    #[staticmethod]
    #[pyo3(signature = (h, s, v, transparency=1.0))]
    pub fn from_hsv(h: i16, s: f32, v: f32, transparency: f32) -> PyResult<Color> {
        find_invalid_range!(s, "Saturation");
        find_invalid_range!(v, "Value");
        find_invalid_range!(transparency, "Transparency");
        let mut adjusted_h = (h as f32).rem_euclid(360.0);
        adjusted_h /= 360.0;
        let floored_h = adjusted_h.floor();
        let diff = adjusted_h - floored_h;
        let a = v * (1.0 - s);
        let b = v * (1.0 - diff * s);
        let c = v * (1.0 - (1.0 - diff) * s);
        let index: usize = (floored_h % 6.0).floor() as usize;
        let r = [v, b, a, a, c, v][index];
        let g = [c, v, v, b, a, a][index];
        let b = [a, a, c, v, v, b][index];
        Ok(to_unit_rgba!(r, g, b, transparency))
    }

    #[staticmethod]
    #[pyo3(signature = (h, s, l, transparency=1.0))]
    pub fn from_hsl(h: i16, s: f32, l: f32, transparency: f32) -> PyResult<Color> {
        find_invalid_range!(s, "Saturation");
        find_invalid_range!(l, "Lightness");
        find_invalid_range!(transparency, "Transparency");
        let h_scoped = h.rem_euclid(360);
        let c = (1.0 - ((2.0 * l) - 1.0).abs()) * s;
        let x = c * (1.0 - ((((h_scoped as f32) / 60.0) % 2.0) - 1.0).abs());
        let m = l - (c / 2.0);
        let (r, g, b) = match h_scoped {
            0..60 => (c, x, 0.0),
            60..120 => (x, c, 0.0),
            120..180 => (0.0, c, x),
            180..240 => (0.0, x, c),
            240..300 => (x, 0.0, c),
            _ => (c, 0.0, x),
        };
        Ok(to_unit_rgba!(r + m, g + m, b + m, transparency))
    }

    #[staticmethod]
    pub fn from_hex(hex_string: &str) -> PyResult<Color> {
        let mut adjusted_str: String = hex_string.to_string();
        if hex_string.starts_with("#") {
            adjusted_str = hex_string.strip_prefix("#").unwrap().to_string();
        }
        if adjusted_str.len() != 6 || adjusted_str.len() != 8 {
            return Err(PyValueError::new_err("Invalid Hex String Length"));
        }
        let r: Option<u8> = interpret_to_hex!(&adjusted_str, 0..2);
        let g: Option<u8> = interpret_to_hex!(&adjusted_str, 2..4);
        let b: Option<u8> = interpret_to_hex!(&adjusted_str, 4..6);
        let a: Option<u8> = if adjusted_str.len() == 8 {
            interpret_to_hex!(&adjusted_str, 4..6)
        } else {Some(255)};
        match (r, g, b, a) {
            (Some(r), Some(g), Some(b), Some(a)) => Ok(Color::new(r, g, b, a)),
            (None, _, _, _) => Err(PyValueError::new_err(
                "Cannot Interpret The First Hexadecimal Part",
            )),
            (_, None, _, _) => Err(PyValueError::new_err(
                "Cannot Interpret The Second Hexadecimal Part",
            )),
            (_, _, None, _) => Err(PyValueError::new_err(
                "Cannot Interpret The Third Hexadecimal Part",
            )),
            (_, _, _, None) => Err(PyValueError::new_err(
                "Cannot Interpret The Fourth Hexadecimal Part",
            )),
        }
    }

    #[staticmethod]
    #[pyo3(signature = (l, a, b, transparency=1.0))]
    pub fn from_oklab(l: f32, a: f32, b: f32, transparency: f32) -> Color {
        let cubed = AutoF32x4::splat(l).add(
            AutoF32x4::new(0.396_337_78, 0.105_561_346, 0.089_484_18, 0f32)
                .mul(AutoF32x4::splat(a))
                .mul(AutoF32x4::new(1f32, -1f32, -1f32, 0f32))
        ).add(
            AutoF32x4::new(0.215_803_76, 0.063_854_17, 1.291_485_5, 0f32)
                .mul(AutoF32x4::splat(b))
                .mul(AutoF32x4::new(1f32, -1f32, -1f32, 0f32))
        ).simd_powi(3i32);
        let l_c = AutoF32x4::splat(cubed.0[0]).mul(AutoF32x4::new(
            4.076_741_7f32,
            -1.268_438f32,
            -0.0041960863f32,
            0f32
        ));

        let a_c = AutoF32x4::splat(cubed.0[1]).mul(AutoF32x4::new(
            -3.307_711_6,
            2.609_757_4,
                -0.703_418_6,
            0f32
        ));

        let b_c = AutoF32x4::splat(cubed.0[2]).mul(AutoF32x4::new(
            0.230_969_94,
            -0.341_319_38,
            1.707_614_7,
            0f32
        ));

        let rgb = l_c.add(a_c).add(b_c).0;

        to_unit_rgba!(rgb[0], rgb[1], rgb[2], transparency)
    }

    #[staticmethod]
    pub fn mlerp(start: PyRef<'_, Self>, end: PyRef<'_, Self>, t: f32) -> PyResult<Color> {
        find_invalid_range!(t, "t");
        let t_inverted = AutoF32x4::splat(1.0 - t);
        let a = u8x4_to_f32x4(AutoU8x4::from(start.0.load(Ordering::Relaxed).to_be_bytes()));
        let b = u8x4_to_f32x4(AutoU8x4::from(end.0.load(Ordering::Relaxed).to_be_bytes()));

        Ok(create_color!(f32x4_to_u32(t_inverted.mul(a).add(AutoF32x4::splat(t).mul(b)).simd_round())))
    }

    #[staticmethod]
    pub fn clerp(start: PyRef<'_, Self>, end: PyRef<'_, Self>, t: f32) -> PyResult<Color> {
        find_invalid_range!(t, "t");
        let a = start.0.load(Ordering::Relaxed);
        let b = end.0.load(Ordering::Relaxed);
        let lch_start = to_lch!(start);
        let lch_end = to_lch!(end);
        let converted = clerp_impl!(lch_start, ((a >> 24) & 0xFF) as f32, lch_end, ((b >> 24) & 0xFF) as f32, t);
        Color::from_lch(
            converted[0],
            converted[1],
            converted[2].floor() as i16,
            converted[3] / 255.0,
        )
    }

    pub fn mlerp_inplace(slf: PyRef<'_, Self>, end: PyRef<'_, Self>, t: f32) -> PyResult<()> {
        let t_inverted = AutoF32x4::splat(1.0 - t);
        let a = u8x4_to_f32x4(AutoU8x4::from(slf.0.load(Ordering::Relaxed).to_be_bytes()));
        let b = u8x4_to_f32x4(AutoU8x4::from(end.0.load(Ordering::Relaxed).to_be_bytes()));

        slf.0.store(f32x4_to_u32(t_inverted.mul(a).add(AutoF32x4::splat(t).mul(b)).simd_round()), Ordering::Relaxed);
        Ok(())
    }

    pub fn clerp_inplace(slf: PyRef<'_, Self>, end: PyRef<'_, Self>, t: f32) -> PyResult<()> {
        let a = slf.0.load(Ordering::Relaxed);
        let b = end.0.load(Ordering::Relaxed);
        let lch_start = to_lch!(slf);
        let lch_end = to_lch!(end);
        let converted = clerp_impl!(lch_start, ((a >> 24) & 0xFF) as f32, lch_end, ((b >> 24) & 0xFF) as f32, t);
        slf.0.store(Color::from_lch(
            converted[0],
            converted[1],
            converted[2].floor() as i16,
            converted[3] / 255.0,
        )?.0.load(Ordering::Relaxed), Ordering::Relaxed);
        Ok(())
    }

    #[staticmethod]
    pub fn blend<'a>(
        slf: PyRef<'a, Self>,
        other: PyRef<'_, Self>,
        blend_mode: blending::BlendingMode
    ) -> PyResult<PyRef<'a, Self>> {
        let rgba1 = to_decimal_rgba!(slf);
        let rgb2 = to_decimal_rgba!(other);
        let blended = blending::compute_blend(&blend_mode, rgba1, rgb2);
        let blended = blended.mul(AutoF32x4::splat(255.0)).simd_round();
        slf.0.store(f32x4_to_u32(blended), Ordering::Relaxed);
        Ok(slf)
    }

    #[pyo3(signature = (other, include_transparency=false))]
    pub fn add<'a>(
        slf: PyRef<'a, Self>,
        other: Self,
        include_transparency: bool,
    ) -> PyResult<PyRef<'a, Self>> {
        slf.0.store(color_to_color_operation(
            &slf, &other, include_transparency, |_| 0, |x, y| {
                u8x4_to_u32(x.add(y).simd_clamp(AutoU8x4::splat(0), AutoU8x4::splat(255)))
            }
        ), Ordering::Relaxed);
        Ok(slf)
    }

    #[pyo3(signature = (other, include_transparency=false))]
    pub fn sub<'a>(
        slf: PyRef<'a, Self>,
        py: Python<'_>,
        other: Py<PyAny>,
        include_transparency: bool
    ) -> PyResult<PyRef<'a, Self>> {
        let result: PyResult<u32> = color_to_unknown_operation(
            py, &slf, other, include_transparency,
            |_| 0, AutoF32x4::sub, |x, y| {
                u8x4_to_u32(x.sub(y).simd_clamp(AutoU8x4::splat(0), AutoU8x4::splat(255)))
            }
        );
        slf.0.store(result?, Ordering::Relaxed);
        Ok(slf)
    }

    #[pyo3(signature = (scalar, include_transparency=false))]
    pub fn mul(slf: PyRef<'_, Self>, scalar: f32, include_transparency: bool) -> PyResult<PyRef<'_, Self>> {
        slf.0.store(
            color_to_scalar_operation(&slf, scalar, include_transparency, |_| 1, AutoF32x4::mul),
            Ordering::Relaxed
        );
        Ok(slf)
    }

    #[pyo3(signature = (scalar, include_transparency=false))]
    pub fn div<'a>(
        slf: PyRef<'a, Self>,
        scalar: f32,
        include_transparency: bool,
    ) -> PyResult<PyRef<'a, Self>> {
        if scalar == 0.0 {
            return Err(PyZeroDivisionError::new_err("Cannot divide a color by zero"))
        }
        slf.0.store(
            color_to_scalar_operation(&slf, scalar, include_transparency, |_| 1, AutoF32x4::div),
            Ordering::Relaxed
        );
        Ok(slf)
    }

    #[pyo3(signature = (other, include_transparency=false))]
    pub fn tensor<'a>(
        slf: PyRef<'a, Self>,
        other: Self,
        include_transparency: bool
    ) -> PyRef<'a, Self> {
        slf.0.store(
            color_to_color_operation(&slf, &other, include_transparency, |_| 1, |a, b| {
                f32x4_to_u32(
                    u8x4_to_f32x4(a).mul(u8x4_to_f32x4(b)).simd_round()
                        .simd_max(AutoF32x4::splat(0f32))
                        .simd_min(AutoF32x4::splat(255f32))
                )
            }),
            Ordering::Relaxed
        );
        slf
    }

    #[pyo3(signature = (include_transparency=false))]
    pub fn sqrt<'a>(
        slf: PyRef<'a, Self>,
        include_transparency: bool,
    ) -> PyResult<PyRef<'a, Self>> {
        let channels = AutoF32x4::from(extract_rgba_channels_by_type!(slf, f32, |v| v))
            .simd_sqrt()
            .simd_clamp(AutoF32x4::splat(0.0), AutoF32x4::splat(255.0))
            .simd_round();
        slf.0.store(f32x4_to_u32(AutoF32x4::new(
            channels.0[0],
            channels.0[1],
            channels.0[2],
            if include_transparency {channels.0[3]} else {slf.0.load(Ordering::Relaxed) as u8 as f32}
        )), Ordering::Relaxed);
        Ok(slf)
    }

    #[pyo3(signature = (base, include_transparency=false))]
    pub fn nth_root<'a>(
        slf: PyRef<'a, Self>,
        base: isize,
        include_transparency: bool,
    ) -> PyResult<PyRef<'a, Self>> {
        if base <= 1 {
            return Err(PyValueError::new_err("Square root base has to be above 1"));
        }
        let float_base: f32 = 1.0 / (base as f32);
        let channels = slf.0.load(Ordering::Relaxed).to_be_bytes();
        let packed = f32x4_to_u32(u8x4_to_f32x4(AutoU8x4::from(channels)).simd_powf(AutoF32x4::new(
            float_base,
            float_base,
            float_base,
            if include_transparency {float_base} else {1f32}
        )).simd_clamp(
            AutoF32x4::splat(0.0),
            AutoF32x4::splat(255.0)
        ).simd_round());
        slf.0.store(packed, Ordering::Relaxed);
        Ok(slf)
    }

    #[pyo3(signature = (other, include_transparency=false))]
    pub fn max(&self, other: Self, include_transparency: bool) -> Self {
        let color1 = AutoU8x4::from(unpack_rgba!(self, include_transparency).to_be_bytes());
        let color2 = AutoU8x4::from(unpack_rgba!(other, include_transparency).to_be_bytes());
        create_color!(u32::from_be_bytes(color1.simd_max(color2).0))
    }

    #[pyo3(signature = (other, include_transparency=false))]
    pub fn min(&self, other: Self, include_transparency: bool) -> Self {
        let color1 = AutoU8x4::from(unpack_rgba!(self, include_transparency).to_be_bytes());
        let color2 = AutoU8x4::from(unpack_rgba!(other, include_transparency).to_be_bytes());
        create_color!(u32::from_be_bytes(color1.simd_max(color2).0))
    }

    #[pyo3(signature = (include_transparency=false))]
    pub fn inverse(slf: PyRef<'_, Self>, include_transparency: bool) -> PyRef<'_, Self> {
        let channels: [u8; 4] = slf.0.load(Ordering::Relaxed).to_be_bytes();
        slf.0.store(u32::from_be_bytes(
            AutoU8x4::new(
                255u8,
                255u8,
                255u8,
                255u8 * (include_transparency as u8)
            ).sub(AutoU8x4::from(channels)).0
        ), Ordering::Relaxed);
        slf
    }

    pub fn grayscale(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
        let channels: [u8; 4] = slf.0.load(Ordering::Relaxed).to_be_bytes();
        let value = AutoF32x4::new(0.299, 0.587, 0.114, 0f32)
            .mul(u8x4_to_f32x4(AutoU8x4::from(channels)))
            .simd_horizontal_sum()
            .round() as u8;
        slf.0.store(u32::from_be_bytes([
            value,
            value,
            value,
            channels[3]
        ]), Ordering::Relaxed);
        slf
    }

    pub fn triadic_colors(&self) -> [Self; 2] {
        let results = self.to_hsl();
        let hue_one: i16 = (results.0 + 120).rem_euclid(360) as i16;
        let hue_two: i16 = ((results.0 as i16) - 120).rem_euclid(360);
        [
            Color::from_hsl(hue_one, results.1, results.2, results.3).unwrap(),
            Color::from_hsl(hue_two, results.1, results.2, results.3).unwrap(),
        ]
    }

    pub fn tetradic_colors(&self) -> [Self; 3] {
        let results = self.to_hsl();
        let hue_one: i16 = (results.0 + 90).rem_euclid(360) as i16;
        let hue_two: i16 = ((results.0 as i16) - 90).rem_euclid(360);
        let hue_three: i16 = ((results.0 as i16) + 180).rem_euclid(360);
        [
            Color::from_hsl(hue_one, results.1, results.2, results.3).unwrap(),
            Color::from_hsl(hue_two, results.1, results.2, results.3).unwrap(),
            Color::from_hsl(hue_three, results.1, results.2, results.3).unwrap(),
        ]
    }

    pub fn complementary_color(&self) -> Self {
        let results = self.to_hsl();
        let hue_one: i16 = (results.0 + 180).rem_euclid(360) as i16;
        Color::from_hsl(hue_one, results.1, results.2, results.3).unwrap()
    }

    pub fn adjust_temperature(slf: PyRef<'_, Self>, temperature: isize) -> PyRef<'_, Self> {
        if temperature == 0 {
            return slf;
        }
        let adjusted_temp: u16 = temperature.clamp(-255, 255) as u16;
        let channels: [u8; 4] = slf.0.load(Ordering::Relaxed).to_be_bytes();
        let r: u8 = ((channels[0] as u16) + adjusted_temp).clamp(0, 255) as u8;
        let b: u8 = ((channels[2] as u16) - adjusted_temp).clamp(0, 255) as u8;
        slf.0.store(u32::from_be_bytes([
            r,
            channels[1],
            b,
            channels[3]
        ]), Ordering::Relaxed);
        slf
    }

    pub fn contrast(slf: PyRef<'_, Self>, factor: f32) -> PyRef<'_, Self> {
        if factor == 0.0 {
            return slf;
        }
        let new_factor = factor + 1.0;
        let channels: [u8; 4] = slf.0.load(Ordering::Relaxed).to_be_bytes();
        let r = (127.5 + ((channels[0] as f32) - 127.5) * new_factor)
            .clamp(0.0, 255.0)
            .round() as u8;
        let g = (127.5 + ((channels[1] as f32) - 127.5) * new_factor)
            .clamp(0.0, 255.0)
            .round() as u8;
        let b = (127.5 + ((channels[2] as f32) - 127.5) * new_factor)
            .clamp(0.0, 255.0)
            .round() as u8;
        slf.0.store(u32::from_be_bytes([
            r,
            g,
            b,
            channels[3]
        ]), Ordering::Relaxed);
        slf
    }

    pub fn contrast_ratio(&self, background: PyRef<'_, Self>) -> f32 {
        let self_luminance = self.get_luminance();
        let background_luminance = background.get_luminance();
        (self_luminance.max(background_luminance) + 0.05) / (self_luminance.min(background_luminance) + 0.05)
    }

    pub fn brightness(slf: PyRef<'_, Self>, factor: f32) -> PyRef<'_, Self> {
        if factor == 0.0 {
            return slf;
        }
        let mut adjusted_factor: f32 = factor + 1.0;
        if factor < 0.0 {
            adjusted_factor = 1.0 / (factor.abs() + 1.0);
        }
        let channels: [u8; 4] = slf.0.load(Ordering::Relaxed).to_be_bytes();
        let r = ((channels[0] as f32) * adjusted_factor).floor()
            .clamp(0.0, 255.0)
            .round() as u8;
        let g = ((channels[1] as f32) * adjusted_factor).floor()
            .clamp(0.0, 255.0)
            .round() as u8;
        let b = ((channels[2] as f32) * adjusted_factor).floor()
            .clamp(0.0, 255.0)
            .round() as u8;
        slf.0.store(u32::from_be_bytes([
            r,
            g,
            b,
            channels[3]
        ]), Ordering::Relaxed);
        slf
    }

    pub fn tint<'a>(slf: PyRef<'a, Self>, degrees: i16) -> PyResult<PyRef<'a, Self>> {
        let new_degrees = &degrees % 360;
        if new_degrees == 0 {
            return Ok(slf);
        }
        let hsl = slf.to_hsl();
        let hue = ((hsl.0 as i16) + new_degrees).rem_euclid(360);
        let new_color = Color::from_hsl(
            hue, hsl.1, hsl.2,
            (extract_rgb_channel!(slf, 3) as f32) / 255.0
        )?;
        slf.0.store(new_color.0.into_inner(), Ordering::Relaxed);
        Ok(slf)
    }

    pub fn saturate<'a>(slf: PyRef<'a, Self>, factor: f32) -> PyResult<PyRef<'a, Self>> {
        if factor == 0.0 {
            return Ok(slf);
        }
        let mut hsv = to_hsv!(slf);
        hsv.1 *= factor + 1.0;
        let new_color = Color::from_hsv(
            hsv.0 as i16, hsv.1, hsv.2,
            (extract_rgb_channel!(slf, 3) as f32) / 255.0
        )?;
        slf.0.store(new_color.0.into_inner(), Ordering::Relaxed);
        Ok(slf)
    }

    #[pyo3(signature = (start=[Some(0), Some(0), Some(0), Some(0)], end=[Some(255), Some(255), Some(255), Some(255)]))]
    pub fn randomise<'a>(
        slf: PyRef<'a, Self>,
        start: [Option<u8>; 4],
        end: [Option<u8>; 4],
    ) -> PyResult<PyRef<'a, Self>> {
        let rand_num = RNG.lock().unwrap().next_u32();
        let slf_num = slf.0.load(Ordering::Relaxed);
        let rand_bytes: Vec<u8> = start
            .into_iter()
            .enumerate()
            .map(|(index, x)| {
                let low: u8 = x.unwrap_or((slf_num << (index * 8)) as u8);
                let high: u8 = end[index].unwrap_or((slf_num << (index * 8)) as u8);
                if low > high {
                    return Err(PyValueError::new_err(
                        format!(
                            "start and end do not make up a range of values in the {}# item",
                            index + 1
                        )
                    ));
                }
                let span = (high as u16) - (low as u16) + 1;

                let v = ((rand_num as u16 * span) / 256) as u8;
                Ok(low.saturating_add(v))
            })
            .collect::<PyResult<Vec<_>>>()?;

        slf.0.store(u32::from_be_bytes([
            rand_bytes[0],
            rand_bytes[1],
            rand_bytes[2],
            rand_bytes[3]
        ]), Ordering::Relaxed);
        Ok(slf)
    }

    pub fn get_luminance(&self) -> f32 {
        let rgba = to_decimal_rgba!(self);
        let mask = rgba.simd_lt(AutoF32x4::splat(0.03928));
        mask.if_else(|| rgba / AutoF32x4::splat(12.92), || {
            (rgba + AutoF32x4::splat(0.055)) / AutoF32x4::splat(1.055)
        })
            .simd_powf(AutoF32x4::splat(2.4))
            .mul(AutoF32x4::new(0.2126, 0.7152, 0.0722, 0f32))
            .simd_horizontal_sum()
    }

    pub fn get_saturation(&self) -> f32 {
        let [r, g, b, _] = self.0.load(Ordering::Relaxed).to_be_bytes();
        let rgb_max: f32 = r.max(g).max(b) as f32;
        if rgb_max == 0.0 {
            return 0.0;
        }
        let rgb_min: f32 = r.min(g).min(b) as f32;

        (rgb_max - rgb_min) / rgb_max
    }

    #[pyo3(signature = (other, diff, include_transparency=false))]
    pub fn approx_equal(
        &self,
        other: Self,
        diff: u8,
        include_transparency: bool,
    ) -> bool {
        let diff_adjusted: i16 = diff as i16;
        let self_channels = self.0.load(Ordering::Relaxed).to_be_bytes();
        let other_channels = other.0.load(Ordering::Relaxed).to_be_bytes();
        let alpha_part = if include_transparency {
            approx_equal_field!(self_channels[3] as i16, other_channels[3] as i16, diff_adjusted)
        } else { true };

        approx_equal_field!(self_channels[0] as i16, other_channels[0] as i16, diff_adjusted)
            && approx_equal_field!(self_channels[1] as i16, other_channels[1] as i16, diff_adjusted)
            && approx_equal_field!(self_channels[2] as i16, other_channels[2] as i16, diff_adjusted)
            && alpha_part
    }

    pub fn copy(&self) -> Self {
        self.clone()
    }


    #[pyo3(signature = (min, max, include_transparency=false))]
    pub fn clamp(&self, min: u8, max: u8, include_transparency: bool) -> PyResult<Self> {
        if min > max {
            return Err(PyValueError::new_err("The supplied min argument is greater than the supplied max argument"))
        }

        let min_u8x4 = AutoU8x4::new(
            min,
            min,
            min,
            if include_transparency {min} else {extract_rgb_channel!(self, 3)}
        );

        let max_u8x4 = AutoU8x4::new(
            min,
            min,
            min,
            if include_transparency {max} else {extract_rgb_channel!(self, 3)}
        );

        Ok(create_color!(
            u8x4_to_u32(
                AutoU8x4::from(self.0.load(Ordering::Relaxed).to_be_bytes())
                    .simd_clamp(min_u8x4, max_u8x4)
            )
        ))
    }

    pub fn clamp_by_color(&self, min: PyRef<'_, Self>, max: PyRef<'_, Self>, include_transparency: bool) -> PyResult<Self> {
        let min_u8x4 = AutoU8x4::from(
            extract_rgba_channels!(min, include_transparency, |_| extract_rgb_channel!(self, 3))
        );

        let max_u8x4 = AutoU8x4::from(
            extract_rgba_channels!(max, include_transparency, |_| extract_rgb_channel!(self, 3))
        );

        if min_u8x4.0[0] > max_u8x4.0[0] {
            return Err(PyValueError::new_err("The supplied min color argument is greater than the supplied max color argument in the red field"))
        } else if min_u8x4.0[1] > max_u8x4.0[1] {
            return Err(PyValueError::new_err("The supplied min color argument is greater than the supplied max color argument in the green field"))
        } else if min_u8x4.0[2] > max_u8x4.0[2] {
            return Err(PyValueError::new_err("The supplied min color argument is greater than the supplied max color argument in the blue field"))
        } else if min_u8x4.0[0] > max_u8x4.0[0] && include_transparency {
            return Err(PyValueError::new_err("The supplied min color argument is greater than the supplied max color argument in the alpha field"))
        }


        Ok(create_color!(
            u8x4_to_u32(
                AutoU8x4::from(self.0.load(Ordering::Relaxed).to_be_bytes())
                    .simd_clamp(min_u8x4, max_u8x4)
            )
        ))
    }

    pub fn alpha_composite(&self, other: PyRef<'_, Self>) -> Self {
        let decimal_self = to_decimal_rgba!(self);
        let decimal_other = to_decimal_rgba!(other);
        let self_alpha = decimal_self.0[3];
        let other_alpha = decimal_other.0[3];
        let coefficient = other_alpha * (1f32 - self_alpha);
        let result_alpha = self_alpha + coefficient;
        create_color!(
            f32x4_to_u32(
                decimal_self
                    .add(AutoF32x4::splat(self_alpha))
                    .add(decimal_other.add(AutoF32x4::splat(coefficient)))
                    .div(AutoF32x4::splat(result_alpha))
                    .simd_round()
                    .simd_clamp(AutoF32x4::splat(0f32), AutoF32x4::splat(255f32))
            )
        )
    }

    #[pyo3(signature = (include_transparency=false))]
    pub fn to_hex(&self, include_transparency: bool) -> String {
        let channels = self.0.load(Ordering::Relaxed).to_be_bytes();
        let hex_str = format!("#{:02x?}{:02x?}{:02x?}", channels[0], channels[1], channels[2]);
        if include_transparency {
            return hex_str + &format!("{:02x?}", channels[3]);
        }
        hex_str
    }

    pub fn to_hsv(&self) -> (u16, f32, f32, f32) {
        let hsv = to_hsv!(self);
        (hsv.0, hsv.1, hsv.2, (extract_rgb_channel!(self, 3) as f32) / 255.0)
    }

    pub fn to_hsl(&self) -> (u16, f32, f32, f32) {
        let values = calc_hue_saturation!(self);
        let l = (values.2 + values.3) / 2.0;
        let delta = values.2 - values.3;
        let s = if delta == 0.0 {
            0.0
        } else {
            delta / (1.0 - (2.0 * l - 1.0).abs())
        };
        (values.0, s, l, (extract_rgb_channel!(self, 3) as f32) / 255.0)
    }

    pub fn to_decimal_rgb(&self) -> (f32, f32, f32) {
        let result = to_decimal_rgba!(self);
        (result.0[0], result.0[1], result.0[2])
    }

    pub fn to_decimal_rgba(&self) -> (f32, f32, f32, f32) {
        let result = to_decimal_rgba!(self).0;
        (result[0], result[1], result[2], result[3])
    }

    pub fn to_cmyk(&self) -> (f32, f32, f32, f32, f32) {
        let rgba = to_decimal_rgba!(*self);
        let k = 1.0 - rgba.simd_horizontal_max();
        let k_invert = 1.0 - k;

        if k_invert == 0.0 {
            return (0.0, 0.0, 0.0, 1.0, rgba.0[3]);
        }
        let splat_invert_k = AutoF32x4::splat(k_invert);
        let cmy = (splat_invert_k - rgba) / splat_invert_k;
        (cmy.0[0], cmy.0[1], cmy.0[2], k, rgba.0[3])
    }

    pub fn to_xyz(&self) -> (f32, f32, f32, f32) {
        let rgba = to_decimal_rgba!(*self);
        let mask = rgba.simd_gt(AutoF32x4::splat(0.04045));
        let result = mask.if_else(|| {
            (rgba + AutoF32x4::splat(0.055)) / AutoF32x4::splat(1.055)
        }, || rgba / AutoF32x4::splat(12.92)) * AutoF32x4::splat(100.0);
        let red_weights = AutoF32x4::new(
            0.4124,
            0.2126,
            0.0193,
            0.0
        ) * AutoF32x4::splat(result.0[0].powf(2.4));

        let green_weights = AutoF32x4::new(
            0.3576,
            0.7152,
            0.1192,
            0.0
        ) * AutoF32x4::splat(result.0[1].powf(2.4));

        let blue_weights = AutoF32x4::new(
            0.1805,
            0.0722,
            0.9505,
            0.0
        ) * AutoF32x4::splat(result.0[1].powf(2.4));

        let final_result = red_weights + green_weights + blue_weights;

        (
            final_result.0[0],
            final_result.0[1],
            final_result.0[2],
            rgba.0[3]
        )
    }

    pub fn to_oklab(slf: PyRef<'_, Self>) -> (f32, f32, f32, f32) {
        let oklab = to_oklab!(slf);
        (oklab.0[0], oklab.0[1], oklab.0[2], (extract_rgb_channel!(&slf, 3) as f32) / 255.0)
    }

    pub fn to_lch(slf: PyRef<'_, Self>) -> (f32, f32, u16, f32) {
        let lch = to_lch!(slf);
        (lch.0, lch.1, lch.2, (extract_rgb_channel!(&slf, 3) as f32) / 255.0)
    }

    pub fn to_rgba_list<'a>(&self, python: Python<'a>) -> PyResult<Bound<'a, PyList>> {
        let rgba = self.0.load(Ordering::Relaxed).to_be_bytes();
        PyList::new(python, vec![rgba[0], rgba[1], rgba[2], rgba[3]])
    }

    pub fn to_decimal_rgba_list<'a>(&self, python: Python<'a>) -> PyResult<Bound<'a, PyList>> {
        let rgba = to_decimal_rgba!(self);
        PyList::new(
            python,
            vec![
                rgba.0[0] as u8,
                rgba.0[1] as u8,
                rgba.0[2] as u8,
                rgba.0[3] as u8
            ],
        )
    }

    pub fn to_rgba_tuple<'a>(&self, python: Python<'a>) -> PyResult<Bound<'a, PyTuple>> {
        let rgba = self.0.load(Ordering::Relaxed).to_be_bytes();
        PyTuple::new(python, vec![rgba[0], rgba[1], rgba[2], rgba[3]])
    }

    pub fn __str__(&self) -> String {
        let rgba = self.0.load(Ordering::Relaxed).to_be_bytes();
        format!("({} : {} : {} : {})", rgba[0], rgba[1], rgba[2], rgba[3])
    }

    pub fn __repr__(&self) -> String {
        let rgba = self.0.load(Ordering::Relaxed).to_be_bytes();
        format!("Color({}, {}, {}, {})", rgba[0], rgba[1], rgba[2], rgba[3])
    }

    pub fn __add__(&self, py: Python<'_>, other: Py<PyAny>) -> PyResult<Color> {
        let result: PyResult<u32> = color_to_unknown_operation(
            py, self, other, true,
            |_| 0, AutoF32x4::add, |a, b| u8x4_to_u32(
                a.add(b).simd_clamp(AutoU8x4::splat(0u8), AutoU8x4::splat(255u8))
            )
        );
        Ok(create_color!(result?))
    }

    pub fn __sub__(&self, py: Python<'_>, other: Py<PyAny>) -> PyResult<Color> {
        let result: PyResult<u32> = color_to_unknown_operation(
            py, self, other, true,
            |_| 0, AutoF32x4::sub, |a, b| u8x4_to_u32(
                a.sub(b).simd_clamp(AutoU8x4::splat(0u8), AutoU8x4::splat(255u8))
            )
        );
        Ok(create_color!(result?))
    }

    pub fn __mul__(&self, py: Python<'_>, other: Py<PyAny>) -> PyResult<Color> {
        let result: PyResult<u32> = color_to_unknown_operation(
            py, self, other, true,
            |_| 0, AutoF32x4::mul, |a, b| u8x4_to_u32(
                a.mul(b).simd_clamp(AutoU8x4::splat(0u8), AutoU8x4::splat(255u8))
            )
        );
        Ok(create_color!(result?))
    }

    pub fn __truediv__(&self, other: f32) -> PyResult<Color> {
        Ok(create_color!(color_to_scalar_operation(
            self, other, true, |_| 1, AutoF32x4::div
        )))
    }

    pub fn __floordiv__(&self, other: isize) -> Color {
        if other <= 0 {
            return create_color!(0u32);
        }
        let m = ((1u128 << 64) / other.unsigned_abs() as u128) as u64 + 1;
        let t = (self.0.load(Ordering::Relaxed) as u64).wrapping_mul(m);
        create_color!((t >> 32) as u32)
    }

    pub fn __hash__(&self) -> u64 {
        let mut hasher = ahash::AHasher::default();
        self.0.load(Ordering::Relaxed).hash(&mut hasher);
        hasher.finish()
    }

    pub fn __nonzero__(&self) -> bool {
        self.0.load(Ordering::Relaxed) != 0
    }

    pub fn __bool__(&self) -> bool {
        self.0.load(Ordering::Relaxed) != 0
    }

    pub fn __eq__(&self, other: Self) -> bool {
        self.0.load(Ordering::Relaxed) == other.0.load(Ordering::Relaxed)
    }
    pub fn __ne__(&self, other: Self) -> bool {
        self.0.load(Ordering::Relaxed) != other.0.load(Ordering::Relaxed)
    }

    pub fn __neg__(&self) -> Color {
        let channels = self.0.load(Ordering::Relaxed).to_be_bytes();
        create_color!(u32::from_be_bytes([
            255 - channels[0],
            255 - channels[1],
            255 - channels[2],
            255 - channels[3]
        ]))
    }

    pub fn __pow__(&self, color: Self, base: f32) -> Color {
        let channels = color.0.load(Ordering::Relaxed).to_be_bytes();
        let r = (channels[0] as f32).powf(base).clamp(0.0, 255.0).floor() as u8;
        let g = (channels[1] as f32).powf(base).clamp(0.0, 255.0).floor() as u8;
        let b = (channels[2] as f32).powf(base).clamp(0.0, 255.0).floor() as u8;
        let a = (channels[3] as f32).powf(base).clamp(0.0, 255.0).floor() as u8;
        create_color!(u32::from_be_bytes([r, g, b, a]))
    }

    pub fn __getitem__(&self, access_code: ColorAccessCode) -> PyResult<u8> {
        let adjusted_access_code = access_code;
        if let ColorAccessCode::String(value) = adjusted_access_code {
            return match value.to_lowercase().as_str() {
                "red" | "r" => Ok(extract_rgb_channel!(self, 0)),
                "green" | "g" => Ok(extract_rgb_channel!(self, 1)),
                "blue" | "b" => Ok(extract_rgb_channel!(self, 2)),
                "alpha" | "a" => Ok(extract_rgb_channel!(self, 3)),
                _ => Err(PyIndexError::new_err(
                    "Cannot access a value outside of the color's reach",
                )),
            };
        }
        match adjusted_access_code {
            ColorAccessCode::Integer(0) => Ok(extract_rgb_channel!(self, 0)),
            ColorAccessCode::Integer(1) => Ok(extract_rgb_channel!(self, 1)),
            ColorAccessCode::Integer(2) => Ok(extract_rgb_channel!(self, 2)),
            ColorAccessCode::Integer(3) => Ok(extract_rgb_channel!(self, 3)),
            _ => Err(PyIndexError::new_err(
                "Cannot access a value outside of the color's reach",
            )),
        }
    }

    pub fn __setitem__(
        slf: PyRef<'_, Self>,
        python: Python<'_>,
        access_code: PyObject,
        new_value: u8,
    ) -> PyResult<()> {
        let mut channels = slf.0.load(Ordering::Relaxed).to_be_bytes();
        let index: usize = if let Ok(index) = access_code.extract::<isize>(python) {
            if !(0isize..3isize).contains(&index) {
                return Err(PyIndexError::new_err(
                    "Cannot set a value outside of the color's reach",
                ));
            }
            index as usize
        } else {
            let string_code: String = access_code.extract(python)?;
            match string_code.to_lowercase().as_str() {
                "red" | "r" => 0,
                "green" | "g" => 1,
                "blue" | "b" => 2,
                "alpha" | "a" => 3,
                _ => return Err(PyIndexError::new_err(
                    "Cannot set a value outside of the color's reach",
                )),
            }
        };
        channels[index] = new_value;
        slf.0.store(u32::from_be_bytes(channels), Ordering::Relaxed);
        Ok(())
    }

    fn shift(&self, places: isize) -> Color {
        shift_impl!(self, places)
    }

    pub fn __rshift__(&self, places: isize) -> Color {
        shift_impl!(self, places)
    }

    pub fn __lshift__(&self, places: isize) -> Color {
        shift_impl!(self, -places)
    }

    pub fn __copy__(&self) -> Color {
        self.clone()
    }

    pub fn __sizeof__(&self) -> usize {
        size_of_val(self)
    }
}
