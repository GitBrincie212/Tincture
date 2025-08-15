use std::sync::atomic::Ordering;
use crate::extract_rgba_channels;
use pyo3::{PyObject, PyResult, Python};
use simba::simd::{AutoF32x4, AutoU8x4, SimdComplexField, SimdValue};
use crate::color::Color;
use crate::color::simd_casts::{f32x4_to_u32};

#[macro_export]
macro_rules! to_decimal_rgba {
    ($color: expr) => {{
        AutoF32x4::from(extract_rgba_channels_by_type!($color, f32, |x| x)) / AutoF32x4::splat(255.0)
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
        let red_weights = AutoF32x4::from([
            0.4122_214_708,
            0.211_903_5,
            0.088_302_46,
            0.0
        ]).mul(AutoF32x4::splat(rgba.0[0]));

        let green_weights = AutoF32x4::from([
            0.536_332_536,
            0.680_699_5,
            0.281_718_85,
            0.0
        ]).mul(AutoF32x4::splat(rgba.0[1]));

        let blue_weights = AutoF32x4::from([
            0.051_445_995,
            0.107_396_96,
            0.629_978_7,
            0.0
        ]).mul(AutoF32x4::splat(rgba.0[2]));

        let lab = red_weights.add(blue_weights).add(green_weights);

        let l_weights = AutoF32x4::from([
            0.210_454_26,
            1.977_998_5,
            0.025_904_037,
            0.0
        ]).mul(AutoF32x4::splat(lab.0[0].cbrt()));

        let a_weights = AutoF32x4::from([
            0.793_617_8,
            2.428_592_2,
            0.782_771_77,
            0.0
        ]).mul(AutoF32x4::splat(lab.0[1].cbrt()));

        let b_weights = AutoF32x4::from([
            0.004_072_047,
            0.450_593_7,
            0.808_675_77,
            0.0
        ]).mul(AutoF32x4::splat(lab.0[2].cbrt()));

        l_weights.add(a_weights).add(b_weights)
    }};
}

#[macro_export]
macro_rules! to_lch {
    ($color: expr) => {{
        let lab = to_oklab!($color);

        let c = (lab.0[1].powf(2.0) + lab.0[2].powf(2.0)).sqrt();

        let mut h = lab.0[1].atan2(lab.0[0]);
        h = if h > 0.0 {
            (h / f32_PI) * 180.0
        } else {
            360.0 - (h.abs() / f32_PI) * 180.0
        };

        (lab.0[0], c, h.floor() as u16)
    }};
}

#[macro_export]
macro_rules! calc_hue_saturation {
    ($color: expr) => {{
        let rgb = to_decimal_rgba!($color).0;

        let c_max = AutoF32x4::from([rgb[0], rgb[1], rgb[2], f32::MIN]).simd_horizontal_max();
        let c_min = AutoF32x4::from([rgb[0], rgb[1], rgb[2], f32::MAX]).simd_horizontal_min();
        let delta = c_max - c_min;
        let mut h: f32 = 0.0;

        if delta == 0.0 {
            h = 0.0
        } else if c_max == rgb[0] {
            h = ((rgb[1] - rgb[2]) / delta) % 6.0;
        } else if c_max == rgb[1] {
            h = ((rgb[2] - rgb[0]) / delta) + 2.0;
        } else if c_max == rgb[2] {
            h = ((rgb[0] - rgb[1]) / delta) + 4.0;
        }

        h *= 60.0;
        h = h.rem_euclid(360.0f32);

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
        Color(AtomicU32::from(f32x4_to_u32(
            AutoF32x4::new($r, $g, $b, $a)
                .mul(AutoF32x4::splat(255f32))
                .simd_round()
        )))
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
) -> u32 where Op: Fn(AutoU8x4, AutoU8x4) -> u32, {
    let a_bits = extract_rgba_channels!(a);
    let b_bits = extract_rgba_channels!(b, include_transparency, identity_fn);
    let a_vec = AutoU8x4::from(a_bits);
    let b_vec = AutoU8x4::from(b_bits);
    op(a_vec, b_vec)
}

#[inline(always)]
pub(crate) fn color_to_scalar_operation<Op>(
    a: &Color,
    b: f32,
    include_transparency: bool,
    identity_fn: impl Fn(u32) -> u8,
    op: Op,
) -> u32 where Op: Fn(AutoF32x4, AutoF32x4) -> AutoF32x4, {
    let a_bits = extract_rgba_channels_by_type!(a, f32, |value| {
        if include_transparency {value as u8} else {identity_fn(value)}
    });
    let a_vec = AutoF32x4::from(a_bits);
    let b_vec = AutoF32x4::splat(b);
    f32x4_to_u32(op(a_vec, b_vec).simd_round())
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
) -> PyResult<u32> where ScalarOP: Fn(AutoF32x4, AutoF32x4) -> AutoF32x4, ColorOP: Fn(AutoU8x4, AutoU8x4) -> u32 {
    if let Ok(scalar) = b.extract::<isize>(py) {
        return Ok(color_to_scalar_operation(a, scalar as f32, include_transparency, identity_fn, scalar_op))
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
        let shift_indexes = AutoUsizex4::new(0, 1, 2, 3)
            .add(AutoUsizex4::splat(shift))
            .rem(AutoUsizex4::splat(3)).0;

        let rotated = [
            rgba[shift_indexes[0]],
            rgba[shift_indexes[1]],
            rgba[shift_indexes[2]],
            rgba[shift_indexes[3]],
        ];

        let shifted = u32::from_be_bytes([rotated[0], rotated[1], rotated[2], rotated[3]]);
        create_color!(shifted)
    }};
}

#[macro_export]
macro_rules! clerp_impl {
    ($start: expr, $alpha_start: expr, $end: expr, $end_alpha: expr, $t: expr) => {{
        AutoF32x4::new($start.0, $start.1, $start.2 as f32, $alpha_start)
            .mul(AutoF32x4::splat(1.0 - $t))
            .add(AutoF32x4::splat($t).mul(AutoF32x4::new(
                $end.0, $end.1, $end.2 as f32, $end_alpha
            ))).0
    }};
}