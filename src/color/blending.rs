use crate::color::*;
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
    PinLight,
    Difference,
    Exclusion,
    Divide,
    Subtract,
    Luminosity,
    Average,
}

fn hardlight_and_overlay(
    color1: (f32, f32, f32, f32),
    color2: (f32, f32, f32),
) -> (f32, f32, f32, f32) {
    if color1.0 < 0.5 && color1.1 < 0.5 && color2.0 < 0.5 {
        return (
            2.0 * color1.0 * color2.0,
            2.0 * color1.1 * color2.1,
            2.0 * color1.2 * color2.2,
            color1.3,
        );
    }
    (
        1.0 - (2.0 * (1.0 - color1.0) * (1.0 - color2.0)),
        1.0 - (2.0 * (1.0 - color1.1) * (1.0 - color2.1)),
        1.0 - (2.0 * (1.0 - color1.2) * (1.0 - color2.2)),
        color1.3,
    )
}

pub(crate) fn compute_blend(
    blending_mode: &BlendingMode,
    color1: (f32, f32, f32, f32),
    color2: (f32, f32, f32),
) -> (f32, f32, f32, f32) {
    match blending_mode {
        BlendingMode::Darken => (
            color1.0.min(color2.0),
            color1.1.min(color2.1),
            color1.2.min(color2.2),
            color1.3,
        ),
        BlendingMode::Multiply => (
            color1.0 * color2.0,
            color1.1 * color2.1,
            color1.2 * color2.2,
            color1.3,
        ),
        BlendingMode::ColorBurn => (
            (color1.0 + color2.0).max(1.0) - 1.0,
            (color1.1 + color2.1).max(1.0) - 1.0,
            (color1.2 + color2.2).max(1.0) - 1.0,
            color1.3,
        ),
        BlendingMode::LinearBurn => (
            1.0 - ((1.0 - color1.0) / color2.0),
            1.0 - ((1.0 - color1.1) / color2.1),
            1.0 - ((1.0 - color1.2) / color2.2),
            color1.3,
        ),
        BlendingMode::Lighten => (
            color1.0.max(color2.0),
            color1.1.max(color2.1),
            color1.2.max(color2.2),
            color1.3,
        ),
        BlendingMode::Screen => (
            1.0 - ((1.0 - color1.0) * (1.0 - color2.0)),
            1.0 - ((1.0 - color1.1) * (1.0 - color2.1)),
            1.0 - ((1.0 - color1.2) * (1.0 - color2.2)),
            color1.3,
        ),
        BlendingMode::LinearDodge => (
            color1.0 + color2.0,
            color1.1 + color2.1,
            color1.2 + color2.2,
            color1.3,
        ),
        BlendingMode::ColorDodge => (
            color2.0 / (1.0 - color1.0),
            color2.1 / (1.0 - color1.1),
            color2.2 / (1.0 - color1.2),
            color1.3,
        ),
        BlendingMode::HardLight => hardlight_and_overlay(color1, color2),
        BlendingMode::Overlay => hardlight_and_overlay(
            (color2.0, color2.1, color2.2, color1.3),
            (color1.0, color1.1, color1.2),
        ),
        BlendingMode::SoftLight => {
            if color1.0 < 0.5 && color1.1 < 0.5 && color2.0 < 0.5 {
                (
                    (1.0 - 2.0 * color1.0) * (color2.0.powi(2)) + 2.0 * color2.0 * color1.0,
                    (1.0 - 2.0 * color1.1) * (color2.1.powi(2)) + 2.0 * color2.1 * color1.1,
                    (1.0 - 2.0 * color1.2) * (color2.2.powi(2)) + 2.0 * color2.2 * color1.2,
                    color1.3,
                )
            } else {
                (
                    2.0 * color2.0 * (1.0 - color2.0) + color2.0.sqrt() * (2.0 * color1.0 - 1.0),
                    2.0 * color2.1 * (1.0 - color2.1) + color2.1.sqrt() * (2.0 * color1.1 - 1.0),
                    2.0 * color2.2 * (1.0 - color2.2) + color2.2.sqrt() * (2.0 * color1.2 - 1.0),
                    color1.3,
                )
            }
        }
        BlendingMode::VividLight => {
            if color1.0 < 0.5 && color1.1 < 0.5 && color2.0 < 0.5 {
                (
                    1.0 - (1.0 - color2.0) / (2.0 * color1.0),
                    1.0 - (1.0 - color2.1) / (2.0 * color1.1),
                    1.0 - (1.0 - color2.2) / (2.0 * color1.2),
                    color1.3,
                )
            } else {
                (
                    color2.0 / (2.0 * (1.0 - color1.0)),
                    color2.1 / (2.0 * (1.0 - color1.1)),
                    color2.2 / (2.0 * (1.0 - color1.2)),
                    color1.3,
                )
            }
        }
        BlendingMode::Average => (
            (color1.0 + color2.0) / 2.0,
            (color1.1 + color2.1) / 2.0,
            (color1.2 + color2.2) / 2.0,
            color1.3,
        ),
        BlendingMode::Exclusion => (
            color1.0 + color2.0 - 2.0 * (color1.0 * color2.0),
            color1.1 + color2.1 - 2.0 * (color1.1 * color2.1),
            color1.2 + color2.2 - 2.0 * (color1.2 * color2.2),
            color1.3,
        ),
        BlendingMode::Difference => (
            (color2.0 - color1.0).abs(),
            (color2.1 - color1.1).abs(),
            (color2.2 - color1.2).abs(),
            color1.3,
        ),
        BlendingMode::Divide => (
            color2.0 / color1.0,
            color2.1 / color1.1,
            color2.2 / color1.2,
            color1.3,
        ),
        BlendingMode::Subtract => (
            color2.0 - color1.0,
            color2.1 - color1.1,
            color2.2 - color1.2,
            color1.3,
        ),

        /*
        BlendingMode::LinearLight => {}
        BlendingMode::PinLight => {}
        BlendingMode::Luminosity => {}
         */
        _ => (0.0, 0.0, 0.0, 0.0),
    }
}
