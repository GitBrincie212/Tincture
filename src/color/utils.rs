use crate::color::{Color};
use std::f64::consts::PI;
use std::ops::Range;

#[macro_export]
macro_rules! to_decimal_rgba {
    ($color: expr) => {{
        let channels = $color.0.to_be_bytes();
        (
            ((channels[0] as f64) / 255.0),
            ((channels[1] as f64) / 255.0),
            ((channels[2] as f64) / 255.0),
            ((channels[3] as f64) / 255.0)
        )
    }};
}

pub(crate) fn interpret_to_hex(adjusted_str: &str, range: Range<usize>) -> Result<u8, String> {
    match u8::from_str_radix(&adjusted_str[range], 16) {
        Ok(r) => Ok(r),
        Err(_) => Err(String::from("")),
    }
}

pub(crate) fn color_to_oklab(color: Color) -> (f64, f64, f64) {
    let rgba = to_decimal_rgba!(color);
    let l = (0.4122_214_708 * &rgba.0) + (0.536_332_536 * &rgba.1) + (0.051_445_995 * &rgba.2);
    let a = (0.211_903_5 * &rgba.0) + (0.680_699_5 * &rgba.1) + (0.107_396_96 * &rgba.2);
    let b = (0.088_302_46 * rgba.0) + (0.281_718_85 * rgba.1) + (0.629_978_7 * rgba.2);

    let l_sqrt_cube = l.cbrt();
    let a_sqrt_cube = a.cbrt();
    let b_sqrt_cube = b.cbrt();

    (
        (0.210_454_26 * l_sqrt_cube) + (0.793_617_8 * a_sqrt_cube) - (0.004_072_047 * b_sqrt_cube),
        (1.977_998_5 * l_sqrt_cube) - (2.428_592_2 * a_sqrt_cube) + (0.450_593_7 * b_sqrt_cube),
        (0.025_904_037 * l_sqrt_cube) + (0.782_771_77 * a_sqrt_cube) - (0.808_675_77 * b_sqrt_cube),
    )
}

pub(crate) fn color_to_lch(color: Color) -> (f64, f64, u16) {
    let lab = color_to_oklab(color);

    let c = (lab.1.powf(2.0) + lab.2.powf(2.0)).sqrt();

    let mut h = lab.1.atan2(lab.0);
    h = if h > 0.0 {
        (h / PI) * 180.0
    } else {
        360.0 - (h.abs() / PI) * 180.0
    };

    (lab.0, c, h.floor() as u16)
}

pub(crate) fn color_to_hsv(color: Color) -> (u16, f64, f64) {
    let values = calculate_hs(color);
    (values.0, values.1, values.2)
}

pub(crate) fn calculate_hs(color: Color) -> (u16, f64, f64, f64) {
    let rgb = to_decimal_rgba!(color);

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
}

#[macro_export]
macro_rules! to_unit_rgb {
    ($r: expr, $g: expr, $b: expr, $a: expr) => {{
        Color(u32::from_be_bytes([
            ($r * 255.0).round() as u8,
            ($g * 255.0).round() as u8,
            ($b * 255.0).round() as u8,
            ($a * 255.0).round() as u8,
        ]))
    }};
}

#[macro_export]
macro_rules! implement_color_to_color_operation {
    ($slf: expr, $other: expr, $include_transparency: expr, $sign: tt) => {{
        let mask: u32 = if $include_transparency { 0xFFFFFFFF } else { 0x00FFFFFF };
        let b_masked = $other.0 & mask;

        let sum = $slf.0 $sign b_masked;

        let overflow = (( $slf.0 ^ b_masked ^ sum ) & 0x01010101) >> 7;
        (sum | overflow).wrapping_sub(overflow)
    }};
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
        if $include_transparency {$slf.0} else {($slf.0 >> 8) & 0x00FFFFFF}
    }};
}

#[macro_export]
macro_rules! implement_color_to_scalar_operation {
    ($self: expr, $other: expr, $include_transparency: expr, $sign: tt) => {{
        let channels: [u8; 4] = $self.0.to_be_bytes();
        u32::from_be_bytes([
            (f32::from(channels[0]) $sign $other).clamp(0.0, 255.0) as u8,
            (f32::from(channels[1]) $sign $other).clamp(0.0, 255.0) as u8,
            (f32::from(channels[2]) $sign $other).clamp(0.0, 255.0) as u8,
            if $include_transparency {
                (f32::from(channels[3]) $sign $other).clamp(0.0, 255.0) as u8
            } else {channels[3]}
        ])
    }};
}

#[macro_export]
macro_rules! implement_color_to_unknown_operation {
    ($py: expr, $slf: expr, $other: expr, $include_transparency: expr, $sign: tt) => {{
        if let Ok(scalar) = $other.extract::<isize>($py) {
            Ok(implement_color_to_scalar_operation!($slf, scalar as f32, $include_transparency, $sign))
        } else {
            let col = $other.extract::<Color>($py)?;
            Ok(implement_color_to_color_operation!($slf, col, $include_transparency, $sign))
        }
    }};
}

#[macro_export]
macro_rules! approx_equal_field  {
    ($value: expr, $diff: expr, $value2: expr) => {
        $value - $diff <= $value2 && $value2 <= $value + $diff
    };
}

#[macro_export]
macro_rules! extract_rgb_channel {
    ($self: expr, $index: expr) => {
        (($self.0 >> (8 * $index)) & 0xFF) as u8
    };
}

#[macro_export]
macro_rules! shift_impl {
    ($self: expr, $places: expr) => {{
        let rgba = $self.0.to_be_bytes();
        let shift = (($places % 3 + 3) % 3) as usize;

        let rotated = [
            rgba[(0 + shift) % 3],
            rgba[(1 + shift) % 3],
            rgba[(2 + shift) % 3],
            rgba[(3 + shift) % 3],
        ];

        let shifted = u32::from_be_bytes([rotated[0], rotated[1], rotated[2], rotated[3]]);
        Color(shifted)
    }};
}