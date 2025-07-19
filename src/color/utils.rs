use std::sync::atomic::Ordering;
use crate::extract_rgba_channels;
use std::simd::{f64x4, u8x4, StdFloat};
use pyo3::{PyObject, PyResult, Python};
use crate::color::Color;

#[macro_export]
macro_rules! to_decimal_rgba {
    ($color: expr) => {{
        let channels = $color.0.load(Ordering::Relaxed).to_be_bytes();
        (
            ((channels[0] as f64) / 255.0),
            ((channels[1] as f64) / 255.0),
            ((channels[2] as f64) / 255.0),
            ((channels[3] as f64) / 255.0)
        )
    }};
}

#[macro_export]
macro_rules! interpret_to_hex {
    ($adjusted_str: expr, $range: expr) => {{
        u8::from_str_radix(&$adjusted_str[$range], 16).ok()
    }};
}

#[macro_export]
macro_rules! to_oklab {
    ($color: expr) => {{
        let rgba = to_decimal_rgba!($color);
        let l: f64 = (0.4122_214_708 * &rgba.0) + (0.536_332_536 * &rgba.1) + (0.051_445_995 * &rgba.2);
        let a: f64 = (0.211_903_5 * &rgba.0) + (0.680_699_5 * &rgba.1) + (0.107_396_96 * &rgba.2);
        let b: f64 = (0.088_302_46 * rgba.0) + (0.281_718_85 * rgba.1) + (0.629_978_7 * rgba.2);

        let l_sqrt_cube = l.cbrt();
        let a_sqrt_cube = a.cbrt();
        let b_sqrt_cube = b.cbrt();

        (
            (0.210_454_26 * l_sqrt_cube) + (0.793_617_8 * a_sqrt_cube) - (0.004_072_047 * b_sqrt_cube),
            (1.977_998_5 * l_sqrt_cube) - (2.428_592_2 * a_sqrt_cube) + (0.450_593_7 * b_sqrt_cube),
            (0.025_904_037 * l_sqrt_cube) + (0.782_771_77 * a_sqrt_cube) - (0.808_675_77 * b_sqrt_cube),
        )
    }};
}

#[macro_export]
macro_rules! to_lch {
    ($color: expr) => {{
        let lab = to_oklab!($color);

        let c = (lab.1.powf(2.0) + lab.2.powf(2.0)).sqrt();

        let mut h = lab.1.atan2(lab.0);
        h = if h > 0.0 {
            (h / PI) * 180.0
        } else {
            360.0 - (h.abs() / PI) * 180.0
        };

        (lab.0, c, h.floor() as u16)
    }};
}

#[macro_export]
macro_rules! calc_hue_saturation {
    ($color: expr) => {{
        let rgb = to_decimal_rgba!($color);

        let c_max = rgb.0.max(rgb.1).max(rgb.2);
        let c_min = rgb.0.min(rgb.1).min(rgb.2);
        let delta = c_max - c_min;
        let mut h = 0.0;

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

        let s = if c_max != 0.0 { delta / c_max } else { 0.0 };

        (h.round() as u16, s, c_max, c_min)
    }};
}

#[macro_export]
macro_rules! to_hsv {
    ($color: expr) => {{
        let values = calc_hue_saturation!($color);
        (values.0, values.1, values.2)
    }};
}

#[macro_export]
macro_rules! create_color {
    ($val: expr) => {{
        Color(AtomicU32::new($val))
    }};
}

#[macro_export]
macro_rules! to_unit_rgba {
    ($r: expr, $g: expr, $b: expr, $a: expr) => {{
        Color(AtomicU32::from(u32::from_be_bytes([
            ($r * 255.0).round() as u8,
            ($g * 255.0).round() as u8,
            ($b * 255.0).round() as u8,
            ($a * 255.0).round() as u8,
        ])))
    }};
}

#[macro_export]
macro_rules! extract_rgb_channel {
    ($self: expr, $index: expr) => {
        (($self.0.load(Ordering::Relaxed) >> (8 * $index)) & 0xFF) as u8
    };
}

#[macro_export]
macro_rules! extract_rgba_channels_by_type {
    ($self: expr, $channel_type: tt, $identity_func: expr) => {{
        let value = $self.0.load(Ordering::Relaxed);
        [
            ((value >> 24) & 0xFF) as $channel_type,
            ((value >> 16) & 0xFF) as $channel_type,
            ((value >> 8)  & 0xFF) as $channel_type,
            $identity_func(value) as $channel_type,
        ]
    }};
}

#[macro_export]
macro_rules! extract_rgba_channels {
    ($self: expr, $include_transparency: expr, $identity_func: expr) => {{
        if $include_transparency {
            extract_rgba_channels!($self)
        } else {extract_rgba_channels_by_type!($self, u8, $identity_func)}
    }};

    ($self: expr) => {{
        $self.0.load(Ordering::Relaxed).to_be_bytes()
    }};
}

#[inline(always)]
pub(crate) fn color_to_color_operation<Op>(
    a: &Color,
    b: &Color,
    include_transparency: bool,
    identity_fn: impl Fn(u32) -> u8,
    op: Op,
) -> u32 where Op: Fn(u8x4, u8x4) -> u8x4, {
    let a_bits = extract_rgba_channels!(a);
    let b_bits = extract_rgba_channels!(b, include_transparency, identity_fn);
    let a_vec = u8x4::from_array(a_bits);
    let b_vec = u8x4::from_array(b_bits);
    u32::from_be_bytes(*op(a_vec, b_vec).as_array())
}

#[inline(always)]
pub(crate) fn color_to_scalar_operation<Op>(
    a: &Color,
    b: f64,
    include_transparency: bool,
    identity_fn: impl Fn(u32) -> u8,
    op: Op,
) -> u32 where Op: Fn(f64x4, f64x4) -> f64x4, {
    let a_bits = extract_rgba_channels_by_type!(a, f64, |value| {
        if include_transparency {value as u8} else {identity_fn(value)}
    });
    let a_vec = f64x4::from_array(a_bits);
    let b_vec = f64x4::splat(b);
    let result = op(a_vec, b_vec).round();
    u32::from_be_bytes([
        result[0] as u8,
        result[1] as u8,
        result[2] as u8,
        result[3] as u8
    ])
}

#[inline(always)]
pub(crate) fn color_to_unknown_operation<ScalarOP, ColorOP>(
    py: Python<'_>,
    a: &Color,
    b: PyObject,
    include_transparency: bool,
    identity_fn: impl Fn(u32) -> u8,
    scalar_op: ScalarOP,
    color_op: ColorOP,
) -> PyResult<u32> where ScalarOP: Fn(f64x4, f64x4) -> f64x4, ColorOP: Fn(u8x4, u8x4) -> u8x4 {
    if let Ok(scalar) = b.extract::<isize>(py) {
        return Ok(color_to_scalar_operation(a, scalar as f64, include_transparency, identity_fn, scalar_op))
    }
    let other = b.extract::<Color>(py)?;
    Ok(color_to_color_operation(a, &other, include_transparency, identity_fn, color_op))
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
macro_rules! unpack_rgba {
    ($slf: expr, $include_transparency: expr) => {{
        if $include_transparency {$slf.0.load(Ordering::Relaxed)} else {($slf.0.load(Ordering::Relaxed) >> 8) & 0x00FFFFFF}
    }};
}

#[macro_export]
macro_rules! approx_equal_field  {
    ($value: expr, $diff: expr, $value2: expr) => {
        $value - $diff <= $value2 && $value2 <= $value + $diff
    };
}

#[macro_export]
macro_rules! shift_impl {
    ($self: expr, $places: expr) => {{
        let rgba = $self.0.load(Ordering::Relaxed).to_be_bytes();
        let shift = (($places % 3 + 3) % 3) as usize;

        let rotated = [
            rgba[(0 + shift) % 3],
            rgba[(1 + shift) % 3],
            rgba[(2 + shift) % 3],
            rgba[(3 + shift) % 3],
        ];

        let shifted = u32::from_be_bytes([rotated[0], rotated[1], rotated[2], rotated[3]]);
        create_color!(shifted)
    }};
}

#[macro_export]
macro_rules! clerp_impl {
    ($start: expr, $end: expr, $t: expr) => {{
        find_invalid_range!($t, "t");
        let t_inverted = 1.0 - $t;
        let a = $start.0.load(Ordering::Relaxed);
        let b = $end.0.load(Ordering::Relaxed) as f64;
        Ok(((t_inverted * (a as f64)) + $t * b).round() as u32)
    }};
}