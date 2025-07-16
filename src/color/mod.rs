use crate::color::utils::*;
use pyo3::exceptions::{PyIndexError, PyValueError, PyZeroDivisionError};
use pyo3::prelude::*;
use pyo3::types::{PyList, PyTuple};
use std::f64::consts::PI;
use std::hash::{Hash, Hasher};
use std::sync::{LazyLock, Mutex};
use rand::prelude::SmallRng;
use rand::{RngCore, SeedableRng};
use crate::{approx_equal_field, extract_rgb_channel, find_invalid_range, implement_color_to_color_operation, implement_color_to_scalar_operation, implement_color_to_unknown_operation, shift_impl, to_decimal_rgba, to_unit_rgb, unpack_rgba};

pub mod blending;
pub mod consts;
mod utils;

static RNG: LazyLock<Mutex<SmallRng>> = LazyLock::new(|| Mutex::new(SmallRng::from_os_rng()));


#[repr(C)]
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
#[pyclass]
pub struct Color(u32);

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
        Color (u32::from_be_bytes([r, g, b, a]))
    }

    #[staticmethod]
    pub fn from_srgb(r: u8, g: u8, b: u8) -> PyResult<Color> {
        Ok(Color(u32::from_be_bytes([r, g, b, 255])))
    }

    #[staticmethod]
    pub fn from_decimal_rgba(r: f64, g: f64, b: f64, a: f64) -> PyResult<Color> {
        find_invalid_range!(r, "Red");
        find_invalid_range!(b, "Blue");
        find_invalid_range!(g, "Green");
        find_invalid_range!(a, "Alpha");
        Ok(to_unit_rgb!(r, g, b, a))
    }

    #[staticmethod]
    pub fn from_cmyk(c: f64, m: f64, y: f64, k: f64, transparency: f64) -> PyResult<Color> {
        find_invalid_range!(c, "Cyan");
        find_invalid_range!(m, "Magenta");
        find_invalid_range!(y, "Yellow");
        find_invalid_range!(k, "Key (Black)");
        find_invalid_range!(transparency, "Transparency");
        Ok(Color(u32::from_be_bytes([
            (255.0 * (1.0 - c) * (1.0 - k)).round() as u8,
            (255.0 * (1.0 - m) * (1.0 - k)).round() as u8,
            (255.0 * (1.0 - y) * (1.0 - k)).round() as u8,
            (transparency * 255.0) as u8
        ])))
    }

    #[staticmethod]
    #[pyo3(signature = (x, y, z, transparency=1.0))]
    pub fn from_xyz(x: f64, y: f64, z: f64, transparency: f64) -> PyResult<Color> {
        find_invalid_range!(x, "X", 0.0, 95.047);
        find_invalid_range!(y, "Y", 0.0, 100.0);
        find_invalid_range!(z, "z", 0.0, 108.883);
        find_invalid_range!(transparency, "Transparency");
        let x = x / 100.0;
        let y = y / 100.0;
        let z = z / 100.0;

        let mut r = x * 3.2406 + y * -1.5372 + z * -0.4986;
        let mut g = x * -0.9689 + y * 1.8758 + z * 0.0415;
        let mut b = x * 0.0557 + y * -0.2040 + z * 1.0570;

        r = if r > 0.0031308 {
            1.055 * (r.powf(0.41666667)) - 0.055
        } else {
            12.92 * r
        };
        g = if g > 0.0031308 {
            1.055 * (g.powf(0.41666667)) - 0.055
        } else {
            12.92 * g
        };
        b = if b > 0.0031308 {
            1.055 * (b.powf(0.41666667)) - 0.055
        } else {
            12.92 * b
        };

        Ok(to_unit_rgb!(r, g, b, transparency))
    }

    #[staticmethod]
    pub fn from_lch(l: f64, c: f64, h: i16, transparency: f64) -> PyResult<Color> {
        find_invalid_range!(l, "L", 0.0, 100.0);
        find_invalid_range!(c, "C", 0.0, 200.0);
        find_invalid_range!(transparency, "Transparency");
        let mut h_scoped = (h as f64).rem_euclid(360.0);
        h_scoped *= PI / 180.0;
        let a = h_scoped.cos() * c;
        let b = h_scoped.sin() * c;
        Ok(Color::from_oklab(l, a, b, transparency))
    }

    #[staticmethod]
    #[pyo3(signature = (h, s, v, transparency=1.0))]
    pub fn from_hsv(h: i16, s: f64, v: f64, transparency: f64) -> PyResult<Color> {
        find_invalid_range!(s, "Saturation");
        find_invalid_range!(v, "Value");
        find_invalid_range!(transparency, "Transparency");
        let mut adjusted_h = (h as f64).rem_euclid(360.0);
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
        Ok(to_unit_rgb!(r, g, b, transparency))
    }

    #[staticmethod]
    #[pyo3(signature = (h, s, l, transparency=1.0))]
    pub fn from_hsl(h: i16, s: f64, l: f64, transparency: f64) -> PyResult<Color> {
        find_invalid_range!(s, "Saturation");
        find_invalid_range!(l, "Lightness");
        find_invalid_range!(transparency, "Transparency");
        let h_scoped = h.rem_euclid(360);
        let c = (1.0 - ((2.0 * l) - 1.0).abs()) * s;
        let x = c * (1.0 - ((((h_scoped as f64) / 60.0) % 2.0) - 1.0).abs());
        let m = l - (c / 2.0);
        let (r, g, b) = match h_scoped {
            0..60 => (c, x, 0.0),
            60..120 => (x, c, 0.0),
            120..180 => (0.0, c, x),
            180..240 => (0.0, x, c),
            240..300 => (x, 0.0, c),
            _ => (c, 0.0, x),
        };
        Ok(to_unit_rgb!(r + m, g + m, b + m, transparency))
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
        let r: Result<u8, String> = interpret_to_hex(&adjusted_str, 0..2);
        let g: Result<u8, String> = interpret_to_hex(&adjusted_str, 2..4);
        let b: Result<u8, String> = interpret_to_hex(&adjusted_str, 4..6);
        let mut a: Result<u8, String> = Ok(255);
        if adjusted_str.len() == 8 {
            a = interpret_to_hex(&adjusted_str, 4..6);
        }
        match (r, g, b, a) {
            (Ok(r), Ok(g), Ok(b), Ok(a)) => Ok(Color::new(r, g, b, a)),
            (Err(_), _, _, _) => Err(PyValueError::new_err(
                "Cannot Interpret The First Hexadecimal Part",
            )),
            (_, Err(_), _, _) => Err(PyValueError::new_err(
                "Cannot Interpret The Second Hexadecimal Part",
            )),
            (_, _, Err(_), _) => Err(PyValueError::new_err(
                "Cannot Interpret The Third Hexadecimal Part",
            )),
            (_, _, _, Err(_)) => Err(PyValueError::new_err(
                "Cannot Interpret The Fourth Hexadecimal Part",
            )),
        }
    }

    #[staticmethod]
    #[pyo3(signature = (l, a, b, transparency=1.0))]
    pub fn from_oklab(l: f64, a: f64, b: f64, transparency: f64) -> Color {
        let l_new = l + (0.396_337_78 * a) + (0.215_803_76 * b);
        let a_new = l - (0.105_561_346 * a) - (0.063_854_17 * b);
        let b_new = l - (0.089_484_18 * a) - (1.291_485_5 * b);

        let l_cubed = l_new.powi(3);
        let a_cubed = a_new.powi(3);
        let b_cubed = b_new.powi(3);

        let r = (4.076_741_7 * l_cubed) - (3.307_711_6 * a_cubed) + (0.230_969_94 * b_cubed);
        let g = (-1.268_438 * l_cubed) + (2.609_757_4 * a_cubed) - (0.341_319_38 * b_cubed);
        let b = (-0.0041960863 * l_cubed) - (0.703_418_6 * a_cubed) + (1.707_614_7 * b_cubed);

        to_unit_rgb!(r, g, b, transparency)
    }

    #[staticmethod]
    pub fn mlerp(start: Self, end: Self, t: f64) -> PyResult<Color> {
        find_invalid_range!(t, "t");
        let t_inverted = 1.0 - t;

        Ok(Color(((t_inverted * start.0 as f64) + t * (end.0 as f64)).round() as u32))
    }

    #[staticmethod]
    pub fn clerp(start: Self, end: Self, t: f64) -> PyResult<Color> {
        find_invalid_range!(t, "t");
        let lch_start = color_to_lch(start);
        let lch_end = color_to_lch(end);
        let one_minus_t = 1.0 - t;
        let new_values = (
            (one_minus_t * lch_start.0) + (t * lch_end.0),
            (one_minus_t * lch_start.1) + (t * lch_end.1),
            (one_minus_t * (lch_start.2 as f64)) + (t * (lch_end.2 as f64)),
            (one_minus_t * ((start.0 >> 24) & 0xFF) as f64) + (t * (((end.0 >> 24) & 0xFF) as f64)),
        );
        Color::from_lch(
            new_values.0,
            new_values.1,
            new_values.2.floor() as i16,
            new_values.3 / 255.0,
        )
    }

    pub fn mlerp_inplace(&mut self, end: Self, t: f64) -> PyResult<()> {
        find_invalid_range!(t, "t");
        let t_inverted = 1.0 - t;
        self.0 = ((t_inverted * self.0 as f64) + t * (end.0 as f64)).round() as u32;
        Ok(())
    }

    pub fn clerp_inplace(&mut self, end: Self, t: f64) -> PyResult<()> {
        let result: Self = Color::clerp(*self, end, t)?;
        self.0 = result.0;
        Ok(())
    }

    #[staticmethod]
    pub fn blend<'a>(
        mut slf: PyRefMut<'a, Self>,
        other: Self,
        blend_mode: blending::BlendingMode
    ) -> PyResult<PyRefMut<'a, Self>> {
        let rgba1 = to_decimal_rgba!(slf);
        let rgb2 = to_decimal_rgba!(other);
        let blended = blending::compute_blend(&blend_mode, rgba1, rgb2);
        let result = to_unit_rgb!(blended.0, blended.1, blended.2, blended.3);
        slf.0 = result.0;
        Ok(slf)
    }

    #[pyo3(signature = (other, include_transparency=false))]
    pub fn add<'a>(
        mut slf: PyRefMut<'a, Self>,
        other: Self,
        include_transparency: bool,
    ) -> PyResult<PyRefMut<'a, Self>> {
        slf.0 = implement_color_to_color_operation!(slf, other, include_transparency, +);
        Ok(slf)
    }

    #[pyo3(signature = (other, include_transparency=false))]
    pub fn sub<'a>(
        mut slf: PyRefMut<'a, Self>,
        py: Python<'_>,
        other: Py<PyAny>,
        include_transparency: bool
    ) -> PyResult<PyRefMut<'a, Self>> {
        let result: PyResult<u32> = implement_color_to_unknown_operation!(py, slf, other, include_transparency, -);
        slf.0 = result?;
        Ok(slf)
    }

    #[pyo3(signature = (scalar, include_transparency=false))]
    pub fn mul(mut slf: PyRefMut<'_, Self>, scalar: f32, include_transparency: bool) -> PyResult<PyRefMut<'_, Self>> {
        slf.0 = implement_color_to_scalar_operation!(slf, scalar, include_transparency, *);
        Ok(slf)
    }

    #[pyo3(signature = (scalar, include_transparency=false))]
    pub fn div<'a>(
        mut slf: PyRefMut<'a, Self>,
        scalar: f32,
        include_transparency: bool,
    ) -> PyResult<PyRefMut<'a, Self>> {
        if scalar == 0.0 {
            return Err(PyZeroDivisionError::new_err("Cannot divide a color by zero"))
        }
        slf.0 = implement_color_to_scalar_operation!(slf, scalar, include_transparency, /);
        Ok(slf)
    }

    #[pyo3(signature = (other, include_transparency=false))]
    pub fn tensor<'a>(
        mut slf: PyRefMut<'a, Self>,
        other: Self,
        include_transparency: bool
    ) -> PyRefMut<'a, Self> {
        slf.0 = implement_color_to_color_operation!(slf, other, include_transparency, *);
        slf
    }

    #[pyo3(signature = (base, include_transparency=false))]
    pub fn base_sqrt<'a>(
        mut slf: PyRefMut<'a, Self>,
        base: isize,
        include_transparency: bool,
    ) -> PyResult<PyRefMut<'a, Self>> {
        if base <= 1 {
            return Err(PyValueError::new_err("Square root base has to be above 1"));
        }
        let float_base: f32 = 1.0 / (base as f32);
        let channels = slf.0.to_be_bytes();
        let r = (channels[0] as f32).powf(float_base).clamp(0.0, 255.0).floor() as u8;
        let g = (channels[1] as f32).powf(float_base).clamp(0.0, 255.0).floor() as u8;
        let b = (channels[2] as f32).powf(float_base).clamp(0.0, 255.0).floor() as u8;
        let a = if include_transparency {
            (channels[3] as f32).powf(float_base).clamp(0.0, 255.0).floor() as u8
        } else {channels[3]};
        slf.0 = u32::from_be_bytes([r, g, b, a]);
        Ok(slf)
    }

    #[pyo3(signature = (other, include_transparency=false))]
    pub fn max(&self, other: Self, include_transparency: bool) -> Self {
        Color(unpack_rgba!(self, include_transparency).max(unpack_rgba!(other, include_transparency)))
    }

    #[pyo3(signature = (other, include_transparency=false))]
    pub fn min(&self, other: Self, include_transparency: bool) -> Self {
        Color(unpack_rgba!(self, include_transparency).min(unpack_rgba!(other, include_transparency)))
    }

    #[pyo3(signature = (include_transparency=false))]
    pub fn inverse(mut slf: PyRefMut<'_, Self>, include_transparency: bool) -> PyRefMut<'_, Self> {
        let channels: [u8; 4] = slf.0.to_be_bytes();
        slf.0 = u32::from_be_bytes([
            255 - channels[0],
            255 - channels[1],
            255 - channels[2],
            if include_transparency {255 - channels[3]} else {channels[3]}
        ]);
        slf
    }

    pub fn grayscale(mut slf: PyRefMut<'_, Self>) -> PyRefMut<'_, Self> {
        let channels: [u8; 4] = slf.0.to_be_bytes();
        let value: u8 = (
            0.299 * channels[0] as f32 +
                0.587 * channels[1] as f32 +
                0.114 * channels[2] as f32
        ).round() as u8;
        slf.0 = u32::from_be_bytes([
            value,
            value,
            value,
            channels[3]
        ]);
        slf
    }

    pub fn triadic_colors(&self) -> [Color; 2] {
        let results = self.to_hsl();
        let hue_one: i16 = (results.0 + 120).rem_euclid(360) as i16;
        let hue_two: i16 = ((results.0 as i16) - 120).rem_euclid(360);
        [
            Color::from_hsl(hue_one, results.1, results.2, results.3).unwrap(),
            Color::from_hsl(hue_two, results.1, results.2, results.3).unwrap(),
        ]
    }

    pub fn adjust_temperature(mut slf: PyRefMut<'_, Self>, temperature: isize) -> PyRefMut<'_, Self> {
        if temperature == 0 {
            return slf;
        }
        let adjusted_temp: u16 = temperature.clamp(-255, 255) as u16;
        let channels: [u8; 4] = slf.0.to_be_bytes();
        let r: u8 = ((channels[0] as u16) + adjusted_temp).clamp(0, 255) as u8;
        let b: u8 = ((channels[2] as u16) - adjusted_temp).clamp(0, 255) as u8;
        slf.0 = u32::from_be_bytes([
            r,
            channels[1],
            b,
            channels[3]
        ]);
        slf
    }

    pub fn contrast(mut slf: PyRefMut<'_, Self>, factor: f32) -> PyRefMut<'_, Self> {
        if factor == 0.0 {
            return slf;
        }
        let new_factor = factor + 1.0;
        let channels: [u8; 4] = slf.0.to_be_bytes();
        let r = (127.5 + ((channels[0] as f32) - 127.5) * new_factor)
            .clamp(0.0, 255.0)
            .round() as u8;
        let g = (127.5 + ((channels[1] as f32) - 127.5) * new_factor)
            .clamp(0.0, 255.0)
            .round() as u8;
        let b = (127.5 + ((channels[2] as f32) - 127.5) * new_factor)
            .clamp(0.0, 255.0)
            .round() as u8;
        slf.0 = u32::from_be_bytes([
            r,
            g,
            b,
            channels[3]
        ]);
        slf
    }

    pub fn brightness(mut slf: PyRefMut<'_, Self>, factor: f32) -> PyRefMut<'_, Self> {
        if factor == 0.0 {
            return slf;
        }
        let mut adjusted_factor: f32 = factor + 1.0;
        if factor < 0.0 {
            adjusted_factor = 1.0 / (factor.abs() + 1.0);
        }
        let channels: [u8; 4] = slf.0.to_be_bytes();
        let r = ((channels[0] as f32) * adjusted_factor).floor()
            .clamp(0.0, 255.0)
            .round() as u8;
        let g = ((channels[1] as f32) * adjusted_factor).floor()
            .clamp(0.0, 255.0)
            .round() as u8;
        let b = ((channels[2] as f32) * adjusted_factor).floor()
            .clamp(0.0, 255.0)
            .round() as u8;
        slf.0 = u32::from_be_bytes([
            r,
            g,
            b,
            channels[3]
        ]);
        slf
    }

    pub fn tint<'a>(mut slf: PyRefMut<'a, Self>, degrees: i16) -> PyResult<PyRefMut<'a, Self>> {
        let new_degrees = &degrees % 360;
        if new_degrees == 0 {
            return Ok(slf);
        }
        let hsl = slf.to_hsl();
        let hue = ((hsl.0 as i16) + new_degrees).rem_euclid(360);
        let new_color = Color::from_hsl(
            hue, hsl.1, hsl.2,
            (extract_rgb_channel!(slf, 3) as f64) / 255.0
        )?;
        slf.0 = new_color.0;
        Ok(slf)
    }

    pub fn saturate<'a>(mut slf: PyRefMut<'a, Self>, factor: f64) -> PyResult<PyRefMut<'a, Self>> {
        if factor == 0.0 {
            return Ok(slf);
        }
        let mut hsv = color_to_hsv(*slf);
        hsv.1 *= factor + 1.0;
        let new_color = Color::from_hsv(
            hsv.0 as i16, hsv.1, hsv.2,
            (extract_rgb_channel!(slf, 3) as f64) / 255.0
        )?;
        slf.0 = new_color.0;
        Ok(slf)
    }

    #[pyo3(signature = (start=[Some(0), Some(0), Some(0), Some(0)], end=[Some(255), Some(255), Some(255), Some(255)]))]
    pub fn randomise<'a>(
        mut slf: PyRefMut<'a, Self>,
        _python: Python,
        start: [Option<u8>; 4],
        end: [Option<u8>; 4],
    ) -> PyResult<PyRefMut<'a, Self>> {
        let rand_num = RNG.lock().unwrap().next_u32();
        let rand_bytes: Vec<u8> = start
            .into_iter()
            .enumerate()
            .map(|(index, x)| {
                let low: u8 = x.unwrap_or((slf.0 << (index * 8)) as u8);
                let high: u8 = end[index].unwrap_or((slf.0 << (index * 8)) as u8);
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

        slf.0 = u32::from_be_bytes([rand_bytes[0], rand_bytes[1], rand_bytes[2], rand_bytes[3]]);
        Ok(slf)
    }

    pub fn get_luminance(&self) -> f64 {
        let rgba = to_decimal_rgba!(self);
        let r = if rgba.0 <= 0.03928 {
            rgba.0 / 12.92
        } else {
            ((rgba.0 + 0.055) / 1.055).powf(2.4)
        };
        let g = if rgba.1 <= 0.03928 {
            rgba.1 / 12.92
        } else {
            ((rgba.1 + 0.055) / 1.055).powf(2.4)
        };
        let b = if rgba.2 <= 0.03928 {
            rgba.2 / 12.92
        } else {
            ((rgba.2 + 0.055) / 1.055).powf(2.4)
        };
        0.2126 * r + 0.7152 * g + 0.0722 * b
    }

    pub fn get_saturation(&self, _python: Python) -> f32 {
        let [r, g, b, _] = self.0.to_be_bytes();
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
        let self_channels = self.0.to_be_bytes();
        let other_channels = other.0.to_be_bytes();
        let alpha_part = if include_transparency {
            approx_equal_field!(self_channels[3] as i16, other_channels[3] as i16, diff_adjusted)
        } else { true };

        approx_equal_field!(self_channels[0] as i16, other_channels[0] as i16, diff_adjusted)
            && approx_equal_field!(self_channels[1] as i16, other_channels[1] as i16, diff_adjusted)
            && approx_equal_field!(self_channels[2] as i16, other_channels[2] as i16, diff_adjusted)
            && alpha_part
    }

    pub fn copy(&self) -> Color {
        Color(self.0.clone())
    }

    #[pyo3(signature = (include_transparency=false))]
    pub fn to_hex(&self, include_transparency: bool) -> String {
        let channels = self.0.to_be_bytes();
        let hex_str = format!("#{:02x?}{:02x?}{:02x?}", channels[0], channels[1], channels[2]);
        if include_transparency {
            return hex_str + &format!("{:02x?}", channels[3]);
        }
        hex_str
    }

    pub fn to_hsv(&self) -> (u16, f64, f64, f64) {
        let hsv = color_to_hsv(*self);
        (hsv.0, hsv.1, hsv.2, (extract_rgb_channel!(self, 3) as f64) / 255.0)
    }

    pub fn to_hsl(&self) -> (u16, f64, f64, f64) {
        let values = calculate_hs(*self);
        let l = (values.2 + values.3) / 2.0;
        let delta = values.2 - values.3;
        let s = if delta == 0.0 {
            0.0
        } else {
            delta / (1.0 - (2.0 * l - 1.0).abs())
        };
        (values.0, s, l, (extract_rgb_channel!(self, 3) as f64) / 255.0)
    }

    pub fn to_decimal_rgb(&self) -> (f64, f64, f64) {
        let result = to_decimal_rgba!(self);
        (result.0, result.1, result.2)
    }

    pub fn to_decimal_rgba(&self) -> (f64, f64, f64, f64) {
        to_decimal_rgba!(self)
    }

    pub fn to_cmyk(&self) -> (f64, f64, f64, f64, f64) {
        let rgb = to_decimal_rgba!(*self);
        let k = 1.0 - rgb.0.max(rgb.1).max(rgb.2);
        let k_invert = 1.0 - k;

        if k_invert == 0.0 {
            return (0.0, 0.0, 0.0, 1.0, ((self.0 >> 24) as f64) / 255.0);
        }
        let c = (k_invert - rgb.0) / k_invert;
        let m = (k_invert - rgb.1) / k_invert;
        let y = (k_invert - rgb.2) / k_invert;
        (c, m, y, k, ((self.0 >> 24) as f64) / 255.0)
    }

    pub fn to_xyz(&self) -> (f64, f64, f64, f64) {
        let rgba = to_decimal_rgba!(*self);

        let r = if rgba.0 > 0.04045 {
            ((rgba.0 + 0.055) / 1.055).powf(2.4)
        } else {
            rgba.0 / 12.92
        };
        let g = if rgba.1 > 0.04045 {
            ((rgba.1 + 0.055) / 1.055).powf(2.4)
        } else {
            rgba.1 / 12.92
        };
        let b = if rgba.2 > 0.04045 {
            ((rgba.2 + 0.055) / 1.055).powf(2.4)
        } else {
            rgba.2 / 12.92
        };

        let r = r * 100.0;
        let g = g * 100.0;
        let b = b * 100.0;

        (
            r * 0.4124 + g * 0.3576 + b * 0.1805,
            r * 0.2126 + g * 0.7152 + b * 0.0722,
            r * 0.0193 + g * 0.1192 + b * 0.9505,
            ((self.0 >> 24) as f64) / 255.0
        )
    }

    pub fn to_oklab(&self, _python: Python) -> (f64, f64, f64, f64) {
        let oklab = color_to_oklab(*self);
        (oklab.0, oklab.1, oklab.2, (extract_rgb_channel!(self, 3) as f64) / 255.0)
    }

    pub fn to_lch(&self, _python: Python) -> (f64, f64, u16, f64) {
        let lch = color_to_lch(*self);
        (lch.0, lch.1, lch.2, (extract_rgb_channel!(self, 3) as f64) / 255.0)
    }

    pub fn to_rgba_list<'a>(&self, python: Python<'a>) -> PyResult<Bound<'a, PyList>> {
        let rgba = self.0.to_be_bytes();
        PyList::new(python, vec![rgba[0], rgba[1], rgba[2], rgba[3]])
    }

    pub fn to_decimal_rgba_list<'a>(&self, python: Python<'a>) -> PyResult<Bound<'a, PyList>> {
        let rgba = to_decimal_rgba!(self);
        PyList::new(
            python,
            vec![
                rgba.0,
                rgba.1,
                rgba.2,
                rgba.3
            ],
        )
    }

    pub fn to_rgba_tuple<'a>(&self, python: Python<'a>) -> PyResult<Bound<'a, PyTuple>> {
        let rgba = self.0.to_be_bytes();
        PyTuple::new(python, vec![rgba[0], rgba[1], rgba[2], rgba[3]])
    }

    pub fn __str__(&self) -> String {
        let rgba = self.0.to_be_bytes();
        format!("({} : {} : {} : {})", rgba[0], rgba[1], rgba[2], rgba[3])
    }

    pub fn __repr__(&self) -> String {
        let rgba = self.0.to_be_bytes();
        format!("Color({}, {}, {}, {})", rgba[0], rgba[1], rgba[2], rgba[3])
    }

    pub fn __add__(&self, py: Python<'_>, other: Py<PyAny>) -> PyResult<Color> {
        let result: PyResult<u32> = implement_color_to_unknown_operation!(py, self, other, true, +);
        Ok(Color(result?))
    }

    pub fn __sub__(&self, py: Python<'_>, other: Py<PyAny>) -> PyResult<Color> {
        let result: PyResult<u32> = implement_color_to_unknown_operation!(py, self, other, true, -);
        Ok(Color(result?))
    }

    pub fn __mul__(&self, py: Python<'_>, other: Py<PyAny>) -> PyResult<Color> {
        let result: PyResult<u32> = implement_color_to_unknown_operation!(py, self, other, true, *);
        Ok(Color(result?))
    }

    pub fn __truediv__(&self, other: f32) -> PyResult<Color> {
        Ok(Color(implement_color_to_scalar_operation!(self, other, true, /)))
    }

    pub fn __floordiv__(&self, other: isize) -> Color {
        if other <= 0 {
            return Color(0u32);
        }
        let m = ((1u128 << 64) / other.abs() as u128) as u64 + 1;
        let t = (self.0 as u64).wrapping_mul(m);
        Color((t >> 32) as u32)
    }

    pub fn __hash__(&self) -> u64 {
        let mut hasher = ahash::AHasher::default();
        self.0.hash(&mut hasher);
        hasher.finish()
    }

    pub fn __nonzero__(&self) -> bool {
        self.0 != 0
    }

    pub fn __bool__(&self) -> bool {
        self.0 != 0
    }

    pub fn __eq__(&self, other: Self) -> bool {
        self.0 == other.0
    }
    pub fn __ne__(&self, other: Self) -> bool {
        self.0 != other.0
    }

    pub fn __neg__(&self) -> Color {
        let channels = self.0.to_be_bytes();
        Color(u32::from_be_bytes([
            255 - channels[0],
            255 - channels[1],
            255 - channels[2],
            255 - channels[3]
        ]))
    }

    pub fn __pow__(&self, color: Self, base: f32) -> Color {
        let channels = color.0.to_be_bytes();
        let r = (channels[0] as f32).powf(base).clamp(0.0, 255.0).floor() as u8;
        let g = (channels[1] as f32).powf(base).clamp(0.0, 255.0).floor() as u8;
        let b = (channels[2] as f32).powf(base).clamp(0.0, 255.0).floor() as u8;
        let a = (channels[3] as f32).powf(base).clamp(0.0, 255.0).floor() as u8;
        Color(u32::from_be_bytes([r, g, b, a]))
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
        mut slf: PyRefMut<'_, Self>,
        python: Python<'_>,
        access_code: PyObject,
        new_value: u8,
    ) -> PyResult<()> {
        let mut channels = slf.0.to_be_bytes();
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
        slf.0 = u32::from_be_bytes(channels);
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
        self.copy()
    }

    pub fn __sizeof__(&self) -> usize {
        32
    }
}
