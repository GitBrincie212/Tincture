use crate::color::Color;
use num_bigint::{BigInt, Sign};
use pyo3::exceptions::{PyIndexError, PyValueError};
use pyo3::PyResult;
use std::f32::consts::PI;
use std::ops::Range;
use rand::prelude::ThreadRng;
use rand::Rng;

pub(crate) fn create_bigint_from_u8(val: u8) -> BigInt {
    BigInt::new(Sign::Plus, vec![val as u32])
}

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
    let l: f32 = (0.412_221_46 * &rgba.0) + (0.536_332_55 * &rgba.1) + (0.051_445_995 * &rgba.2);

    let a: f32 = (0.211_903_5 * &rgba.0) + (0.680_699_5 * &rgba.1) + (0.107_396_96 * &rgba.2);

    let b: f32 = (0.088_302_46 * rgba.0) + (0.281_718_85 * rgba.1) + (0.629_978_7 * rgba.2);

    let l_sqrt_cube: f32 = l.powf(3.333333);
    let a_sqrt_cube: f32 = a.powf(3.333333);
    let b_sqrt_cube: f32 = b.powf(3.333333);

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

pub(crate) fn to_whole_rgb(r: f32, g: f32, b: f32, a: f32) -> Color {
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

pub(crate) fn find_invalid_percentage_range(val: f32, name: &str) -> PyResult<()> {
    if !(0.0..=1.0).contains(&val) {
        return Err(PyValueError::new_err(format!(
            "{} percentage must be between 0.0 and 1.0",
            name
        )));
    }
    Ok(())
}

pub(crate) fn color_add_color(value: &Color, other: &Color, include_transparency: bool) -> Color {
    Color {
        r: ((value.r as u16) + (other.r as u16)).min(255) as u8,
        g: ((value.g as u16) + (other.g as u16)).min(255) as u8,
        b: ((value.b as u16) + (other.b as u16)).min(255) as u8,
        a: if include_transparency {
            ((value.a as u16) + (other.a as u16)).min(255) as u8
        } else {
            value.a
        },
    }
}

pub(crate) fn color_add_scalar(value: &Color, other: BigInt, include_transparency: bool) -> Color {
    Color {
        r: (wrap_around_bigint(create_bigint_from_u8(value.r) + &other).1).min(255) as u8,
        g: (wrap_around_bigint(create_bigint_from_u8(value.g) + &other).1).min(255) as u8,
        b: (wrap_around_bigint(create_bigint_from_u8(value.b) + &other).1).min(255) as u8,
        a: if include_transparency {
            (wrap_around_bigint(create_bigint_from_u8(value.a) + &other).1).min(255) as u8
        } else {
            value.a
        },
    }
}

pub(crate) fn color_sub_color(value: &Color, other: &Color, include_transparency: bool) -> Color {
    Color {
        r: ((value.r as i16) - (other.r as i16)).max(0) as u8,
        g: ((value.g as i16) - (other.g as i16)).max(0) as u8,
        b: ((value.b as i16) - (other.b as i16)).max(0) as u8,
        a: if include_transparency {
            ((value.a as i16) - (other.a as i16)).max(0) as u8
        } else {
            value.a
        },
    }
}

pub(crate) fn color_sub_scalar(value: &Color, other: BigInt, include_transparency: bool) -> Color {
    Color {
        r: (wrap_around_bigint(create_bigint_from_u8(value.r) - &other).1).min(255) as u8,
        g: (wrap_around_bigint(create_bigint_from_u8(value.g) - &other).1).min(255) as u8,
        b: (wrap_around_bigint(create_bigint_from_u8(value.b) - &other).1).min(255) as u8,
        a: if include_transparency {
            (wrap_around_bigint(create_bigint_from_u8(value.a) - &other).1).min(255) as u8
        } else {
            value.a
        },
    }
}

pub(crate) fn randomise_component(
    value: u8, start: Option<u8>, end: Option<u8>, rng: &mut ThreadRng, name: &str
) -> PyResult<u8> {
    match (start, end) {
        (Some(val1), Some(val2)) => {
            if val1 >= val2 {
                return Err(PyIndexError::new_err(format!(
                    "Starting & Ending Bounds Are Out Of Range For {}",
                    name
                )));
            }
            Ok(rng.gen_range(val1..val2))
        }
        (None, None) => { Ok(value) }
        _ => {
            Err(PyValueError::new_err(
                "Cannot have None & a integer fields on start & end at the same time",
            ))
        }
    }
}