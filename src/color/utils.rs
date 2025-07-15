use crate::color::{Color, RNG};
use pyo3::exceptions::{PyIndexError, PyValueError};
use pyo3::PyResult;
use std::f32::consts::PI;
use std::ops::Range;
use rand::Rng;

pub(crate) fn interpret_to_hex(adjusted_str: &str, range: Range<usize>) -> Result<u8, String> {
    match u8::from_str_radix(&adjusted_str[range], 16) {
        Ok(r) => Ok(r),
        Err(_) => Err(String::from("")),
    }
}

pub(crate) fn color_to_decimal_rgb(color: Color) -> (f32, f32, f32) {
    (
        color.r as f32 / 255.0,
        color.g as f32 / 255.0,
        color.b as f32 / 255.0,
    )
}

pub(crate) fn color_to_oklab(color: Color) -> (f32, f32, f32) {
    let rgba = color_to_decimal_rgb(color);
    let l: f32 = (0.4122_214_708 * &rgba.0) + (0.536_332_536 * &rgba.1) + (0.051_445_995 * &rgba.2);
    let a: f32 = (0.211_903_5 * &rgba.0) + (0.680_699_5 * &rgba.1) + (0.107_396_96 * &rgba.2);
    let b: f32 = (0.088_302_46 * rgba.0) + (0.281_718_85 * rgba.1) + (0.629_978_7 * rgba.2);

    let l_sqrt_cube: f32 = l.cbrt();
    let a_sqrt_cube: f32 = a.cbrt();
    let b_sqrt_cube: f32 = b.cbrt();

    (
        (0.210_454_26 * l_sqrt_cube) + (0.793_617_8 * a_sqrt_cube) - (0.004_072_047 * b_sqrt_cube),
        (1.977_998_5 * l_sqrt_cube) - (2.428_592_2 * a_sqrt_cube) + (0.450_593_7 * b_sqrt_cube),
        (0.025_904_037 * l_sqrt_cube) + (0.782_771_77 * a_sqrt_cube) - (0.808_675_77 * b_sqrt_cube),
    )
}

pub(crate) fn color_to_lch(color: Color) -> (f32, f32, u16) {
    let lab: (f32, f32, f32) = color_to_oklab(color);

    let c: f32 = (lab.1.powf(2.0) + lab.2.powf(2.0)).sqrt();

    let mut h: f32 = lab.1.atan2(lab.0);
    h = if h > 0.0 {
        (h / PI) * 180.0
    } else {
        360.0 - (h.abs() / PI) * 180.0
    };

    (lab.0, c, h.floor() as u16)
}

pub(crate) fn color_to_hsv(color: Color) -> (u16, f32, f32) {
    let values: (u16, f32, f32, f32) = calculate_hs(color);
    (values.0, values.1, values.2)
}

pub(crate) fn calculate_hs(color: Color) -> (u16, f32, f32, f32) {
    let rgb: (f32, f32, f32) = color_to_decimal_rgb(color);

    let c_max: f32 = rgb.0.max(rgb.1).max(rgb.2);
    let c_min: f32 = rgb.0.min(rgb.1).min(rgb.2);
    let delta: f32 = c_max - c_min;
    let mut h: f32 = 0.0;

    if delta == 0.0 {
        h = 0.0
    } else if c_max == rgb.0 {
        h = ((rgb.1 - rgb.2) / delta) % 6.0;
    } else if c_max == rgb.1 {
        h = ((rgb.2 - rgb.0) / delta) + 2.0;
    } else if c_max == rgb.2 {
        h = ((rgb.0 - rgb.1) / delta) + 4.0;
    }

    h *= 60.0;
    h = h.rem_euclid(360.0);

    let s: f32 = if c_max != 0.0 { delta / c_max } else { 0.0 };

    (h.round() as u16, s, c_max, c_min)
}

pub(crate) fn to_unit_rgb(r: f32, g: f32, b: f32, a: f32) -> Color {
    Color {
        r: (r * 255.0).floor() as u8,
        g: (g * 255.0).floor() as u8,
        b: (b * 255.0).floor() as u8,
        a: (a * 255.0).floor() as u8,
    }
}

pub(crate) fn unwrap_color(color: Color) -> (u8, u8, u8, u8) {
    (color.r, color.g, color.b, color.a)
}

#[macro_export]
macro_rules! find_invalid_range {
    ($val: expr, $name: expr) => {
        find_invalid_range!($val, concat!($name, " percentage"), 0.0, 1.0)
    };

    ($val: expr, $name: expr, $min: expr, $max: expr) => {
        if !($min..=$max).contains(&$val) {
            return Err(PyValueError::new_err(format!(
                "{} must be between {} and {}",
                $name, $min, $max
            )));
        }
    }
}

#[macro_export]
macro_rules! implement_add_sub_operations {
    ($self: expr, $py: expr, $other: expr, $include_transparency: expr, $sign: tt) => {{
        if let Ok(scalar) = $other.extract::<isize>($py) {
            let a: usizex4 = usizex4::from_array([$self.r as usize, $self.g as usize, $self.b as usize, $self.a as usize]);
            let b: usizex4 = usizex4::from_array([scalar as usize, scalar as usize, scalar as usize, scalar as usize]);
            let result = (a $sign b).simd_clamp(Simd::from_array([0, 0, 0, 0]), Simd::from_array([255, 255, 255, 255]));
            return Ok(
                Color {
                    r: result[0] as u8,
                    g: result[1] as u8,
                    b: result[2] as u8,
                    a: result[3] as u8
                }
            );
        }


        let col = $other.extract::<Color>($py)?;
        let a: usizex4 = usizex4::from_array([$self.r as usize, $self.g as usize, $self.b as usize, $self.a as usize]);
        let b: usizex4 = usizex4::from_array([col.r as usize, col.g as usize, col.b as usize, if $include_transparency {col.a as usize} else {0}]);
        let result = (a $sign b).simd_clamp(Simd::from_array([0, 0, 0, 0]), Simd::from_array([255, 255, 255, 255]));

        Ok(Color {
            r: result[0] as u8,
            g: result[1] as u8,
            b: result[2] as u8,
            a: result[3] as u8
        })
    }};
}

#[macro_export]
macro_rules! implement_tensor_operation {
    ($self: expr, $other: expr, $include_transparency: expr) => {{
        let a: u16x4 = u16x4::from_array([$self.r as u16, $self.g as u16, $self.b as u16, $self.a as u16]);
        let b: u16x4 = u16x4::from_array([$other.r as u16, $other.g as u16, $other.b as u16, if $include_transparency {$other.a as u16} else {0}]);
        let result = (a * b).simd_clamp(Simd::from_array([0, 0, 0, 0]), Simd::from_array([255, 255, 255, 255]));

        Color {
            r: result[0] as u8,
            g: result[1] as u8,
            b: result[2] as u8,
            a: result[3] as u8
        }
    }};
}

#[macro_export]
macro_rules! implement_multiply_operation {
    ($self: expr, $other: expr, $include_transparency: expr) => {{
        let a: f32x4 = f32x4::from_array([$self.r as f32, $self.g as f32, $self.b as f32, $self.a as f32]);
        let b: f32x4 = f32x4::from_array([$other, $other, $other, if $include_transparency {$other} else {1.0}]);
        let result = (a * b)
            .simd_clamp(Simd::from_array([0.0, 0.0, 0.0, 0.0]), Simd::from_array([255.0, 255.0, 255.0, 255.0]))
            .round();

        Color {
            r: result[0] as u8,
            g: result[1] as u8,
            b: result[2] as u8,
            a: result[3] as u8
        }
    }};
}

#[macro_export]
macro_rules! implement_div_operation {
    ($self: expr, $scalar: expr, $include_transparency: expr) => {{
        if $scalar == 0.0 {
            return Err(PyZeroDivisionError::new_err("Scalar division by zero"));
        }
        let a: f32x4 = f32x4::from_array([$self.r as f32, $self.g as f32, $self.b as f32, $self.a as f32]);
        let b: f32x4 = f32x4::from_array([$scalar, $scalar, $scalar, if $include_transparency {$scalar} else {1.0}]);
        let result = (a / b)
            .simd_clamp(Simd::from_array([0.0, 0.0, 0.0, 0.0]), Simd::from_array([255.0, 255.0, 255.0, 255.0]))
            .round();

        Ok(Color {
            r: result[0] as u8,
            g: result[1] as u8,
            b: result[2] as u8,
            a: result[3] as u8
        })
    }};
}

pub(crate) fn randomise_component(
    value: u8, start: Option<u8>, end: Option<u8>, name: &str
) -> PyResult<u8> {
    match (start, end) {
        (Some(val1), Some(val2)) => {
            if val1 >= val2 {
                return Err(PyIndexError::new_err(format!(
                    "Starting & Ending Bounds Are Out Of Range For {}",
                    name
                )));
            }
            Ok(RNG.lock().unwrap().random_range(val1..=val2))
        }
        (None, None) => { Ok(value) }
        _ => {
            Err(PyValueError::new_err(
                "Cannot have None & a integer fields on start & end at the same time",
            ))
        }
    }
}