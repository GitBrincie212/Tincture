use std::simd::{f64x4, Mask, StdFloat};
use std::simd::prelude::{SimdFloat, SimdPartialOrd};
use pyo3::pyclass;

#[pyclass(eq, eq_int)]
#[derive(Clone, PartialEq)]
pub enum BlendingMode {
    Darken,
    Multiply,
    ColorBurn,
    LinearBurn,
    Lighten,
    Screen,
    ColorDodge,
    LinearDodge,
    Overlay,
    SoftLight,
    HardLight,
    VividLight,
    LinearLight,
    Difference,
    Exclusion,
    Divide,
    Subtract,
    Luminosity,
    Average,
}

macro_rules! perform_alpha_composition {
    ($blended_color: expr, $color1: expr, $color2: expr) => {{
        let blended_alpha = $color1[3] + ($color2[3] * (1.0 - $color1[3]));
        let color1_alpha = f64x4::splat($color1[3]);
        let color2_alpha = f64x4::splat($color2[3]);
        let rgb = (
            $blended_color * color1_alpha + $color2 * color2_alpha * (f64x4::splat(1.0) - color1_alpha)
        ) / f64x4::splat(blended_alpha);
        f64x4::from_array([rgb[0], rgb[1], rgb[2], blended_alpha])
    }};
}

macro_rules! hardlight_and_overlay_common {
    ($condition: expr, $color1: expr, $color2: expr) => {{
        let one = f64x4::splat(1.0);
        let two = f64x4::splat(2.0);
        let z1 = two * $color1 * $color2;
        let z2 = one - two * (one - $color1) * (one - $color2);
        let mask: Mask<i64, 4> = $condition.simd_lt(f64x4::splat(0.5));
        mask.select(z1, z2)
    }};
}

pub(crate) fn compute_blend(
    blending_mode: &BlendingMode,
    color1: f64x4,
    color2: f64x4,
) -> f64x4 {
    match blending_mode {
        BlendingMode::Darken => {
            let val = color1.simd_min(color2);
            perform_alpha_composition!(val, color1, color2)
        },
        BlendingMode::Multiply => {
            let val = color1 * color2;
            perform_alpha_composition!(val, color1, color2)
        },
        BlendingMode::ColorBurn => {
            let val = (color1 + color2).simd_max(f64x4::splat(1.0)) - f64x4::splat(1.0);
            perform_alpha_composition!(val, color1, color2)
        },
        BlendingMode::LinearBurn => {
            let val = f64x4::splat(1.0) - (f64x4::splat(1.0) - color1 / color2);
            perform_alpha_composition!(val, color1, color2)
        },
        BlendingMode::Lighten => {
            let val = color1.simd_max(color2);
            perform_alpha_composition!(val, color1, color2)
        },
        BlendingMode::Screen => {
            let one = f64x4::splat(1.0);
            let val = one - ((one - color1) * (one - color2));
            perform_alpha_composition!(val, color1, color2)
        },
        BlendingMode::LinearDodge => {
            let val = color1 + color2;
            perform_alpha_composition!(val, color1, color2)
        },
        BlendingMode::ColorDodge => {
            let val = color2 / (f64x4::splat(1.0) - color1);
            perform_alpha_composition!(val, color1, color2)
        },
        BlendingMode::HardLight => {
            let val = hardlight_and_overlay_common!(color1, color1, color2);
            perform_alpha_composition!(val, color1, color2)
        },
        BlendingMode::Overlay => {
            let val = hardlight_and_overlay_common!(color2, color1, color2);
            perform_alpha_composition!(val, color1, color2)
        },
        BlendingMode::SoftLight => {
            let one = f64x4::splat(1.0);
            let two = f64x4::splat(2.0);
            let z1 = one - two * (one - color1) * (one - color2) + two * color2 * color1;
            let z2 = two * color2 * (one - color2) + color2.sqrt() * (two * color1 - one);
            let mask: Mask<i64, 4> = color1.simd_lt(f64x4::splat(0.5));
            perform_alpha_composition!(mask.select(z1, z2), color1, color2)
        }
        BlendingMode::VividLight => {
            let one = f64x4::splat(1.0);
            let two = f64x4::splat(2.0);
            let z1 = one - (one - color2) / (two * color1);
            let z2 = color2 / (two * (one - color1));
            let mask: Mask<i64, 4> = color1.simd_lt(f64x4::splat(0.5));
            perform_alpha_composition!(mask.select(z1, z2), color1, color2)
        }
        BlendingMode::Average => {
            let value = (color1 + color2) / f64x4::splat(2.0);
            perform_alpha_composition!(value, color1, color2)
        },
        BlendingMode::Exclusion => {
            let value = color1 + color2 - f64x4::splat(2.0) * (color1 * color2);
            perform_alpha_composition!(value, color1, color2)
        },
        BlendingMode::Difference => {
            let value = (color2 - color1).abs();
            perform_alpha_composition!(value, color1, color2)
        },
        BlendingMode::Divide => {
            let value = color2 / color1;
            perform_alpha_composition!(value, color1, color2)
        },
        BlendingMode::Subtract => {
            let value = color2 - color1;
            perform_alpha_composition!(value, color1, color2)
        },

        BlendingMode::LinearLight => {
            let value = color2 + (f64x4::splat(2.0) * color1) - f64x4::splat(1.0);
            perform_alpha_composition!(value, color1, color2)
        },

        BlendingMode::Luminosity => {
            let weights: f64x4 = f64x4::from_array([0.00117255, 0.00230196, 0.00044706, 0.0]);
            let prod = (color1 - color2) * weights;
            let gray = f64x4::splat(prod[0] + prod[1] + prod[2]);
            perform_alpha_composition!(gray + color2, color1, color2)
        }
    }
}
