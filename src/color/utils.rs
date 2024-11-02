use crate::color::Color;
use num_bigint::{BigInt, Sign};
use pyo3::exceptions::PyValueError;
use pyo3::PyResult;
use std::f32::consts::PI;
use std::ops::Range;

pub(crate) fn interpret_to_hex(adjusted_str: &str, range: Range<usize>) -> Result<u8, String> {
    match u8::from_str_radix(&adjusted_str[range], 16) {
        Ok(r) => Ok(r),
        Err(_) => Err(String::from("")),
    }
}

pub(crate) fn wrap_around_bigint(value: BigInt) -> (Sign, u32) {
    let sign_and_digits: (Sign, Vec<u32>) = value.to_u32_digits();
    if sign_and_digits.0 == Sign::NoSign && sign_and_digits.1 == Vec::<u32>::new() {
        return (Sign::NoSign, 0);
    }
    (sign_and_digits.0, sign_and_digits.1[0])
}

pub(crate) fn wrap_around_bigint_as_i16(value: BigInt) -> i16 {
    let result = wrap_around_bigint(value);
    if result.0 == Sign::Minus {
        return -(result.1 as i16);
    }
    result.1 as i16
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
    let l: f32 = (0.4122214708 * &rgba.0)
        + (0.5363325363 * &rgba.1)
        + (0.0514459929 * &rgba.2);

    let a: f32 = (0.2119034982 * &rgba.0)
        + (0.6806995451 * &rgba.1)
        + (0.1073969566 * &rgba.2);

    let b: f32 = (0.0883024619 * rgba.0)
        + (0.2817188376 * rgba.1)
        + (0.6299787005 * rgba.2);

    let l_sqrt_cube: f32 = l.powf(3.333333);
    let a_sqrt_cube: f32 = a.powf(3.333333);
    let b_sqrt_cube: f32 = b.powf(3.333333);

    (
        (0.2104542553 * l_sqrt_cube) + (0.7936177850 * a_sqrt_cube) - (0.0040720468 * b_sqrt_cube),
        (1.9779984951 * l_sqrt_cube) - (2.4285922050 * a_sqrt_cube) + (0.4505937099 * b_sqrt_cube),
        (0.0259040371 * l_sqrt_cube) + (0.7827717662 * a_sqrt_cube) - (0.8086757660 * b_sqrt_cube),
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
    let mut h: f32;

    if c_max == rgb.0 {
        h = ((rgb.1 - rgb.2) / delta) % 6.0;
    } else if c_max == rgb.1 {
        h = ((rgb.2 - rgb.0) / delta) + 2.0;
    } else if c_max == rgb.2 {
        h = ((rgb.0 - rgb.1) / delta) + 4.0;
    } else {
        h = 0.0
    }

    h *= 60.0;
    h = h.rem_euclid(360.0);

    let s: f32 = if c_max != 0.0 { delta / c_max } else { 0.0 };

    (h.round() as u16, s, c_max, c_min)
}

pub(crate) fn to_whole_rgb(r: f32, g: f32, b: f32, a: f32) -> Color {
    Color {
        r: (r * 255.0).floor() as u8,
        g: (g * 255.0).floor() as u8,
        b: (b * 255.0).floor() as u8,
        a: (a * 255.0).floor() as u8,
    }
}

pub(crate) fn find_invalid_percentage_range(val: f32, name: &str) -> PyResult<()> {
    if val < 0.0 || val > 1.0 {
        return Err(PyValueError::new_err(format!(
            "{} percentage must be between 0 and 1",
            name
        )));
    }
    Ok(())
}
