#![allow(unused_must_use)]
#![allow(clippy::wrong_self_convention)]

mod color;
mod batch;

use pyo3::prelude::*;

#[pymodule]
fn tincture(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<color::Color>();
    m.add_class::<batch::ColorBatch>();
    m.add_class::<color::blending::BlendingMode>();
    m.add("WHITE", color::consts::WHITE.clone());
    m.add("RED", color::consts::RED.clone());
    m.add("BLUE", color::consts::BLUE.clone());
    m.add("GREEN", color::consts::GREEN.clone());
    m.add("BLACK", color::consts::BLACK.clone());
    m.add("CYAN", color::consts::CYAN.clone());
    m.add("YELLOW", color::consts::YELLOW.clone());
    m.add("MAGENTA", color::consts::MAGENTA.clone());
    m.add("PURPLE", color::consts::PURPLE.clone());
    m.add("LIME", color::consts::LIME.clone());
    m.add("ORANGE", color::consts::ORANGE.clone());
    m.add("TEAL", color::consts::TEAL.clone());
    m.add("PINK", color::consts::PINK.clone());
    m.add("DARK_RED", color::consts::DARK_RED.clone());
    m.add("DARK_BLUE", color::consts::DARK_BLUE.clone());
    m.add("DARK_GREEN", color::consts::DARK_GREEN.clone());
    m.add("DARK_CYAN", color::consts::DARK_CYAN.clone());
    m.add("DARK_YELLOW", color::consts::DARK_YELLOW.clone());
    m.add("DARK_MAGENTA", color::consts::DARK_MAGENTA.clone());
    m.add("DARK_PURPLE", color::consts::DARK_PURPLE.clone());
    m.add("DARK_LIME", color::consts::DARK_LIME.clone());
    m.add("DARK_ORANGE", color::consts::DARK_ORANGE.clone());
    m.add("DARK_TEAL", color::consts::DARK_TEAL.clone());
    m.add("DARK_PINK", color::consts::DARK_PINK.clone());
    m.add("LIGHT_RED", color::consts::LIGHT_RED.clone());
    m.add("LIGHT_BLUE", color::consts::LIGHT_BLUE.clone());
    m.add("LIGHT_GREEN", color::consts::LIGHT_GREEN.clone());
    m.add("LIGHT_CYAN", color::consts::LIGHT_CYAN.clone());
    m.add("LIGHT_YELLOW", color::consts::LIGHT_YELLOW.clone());
    m.add("LIGHT_MAGENTA", color::consts::LIGHT_MAGENTA.clone());
    m.add("LIGHT_PURPLE", color::consts::LIGHT_PURPLE.clone());
    m.add("LIGHT_LIME", color::consts::LIGHT_LIME.clone());
    m.add("LIGHT_ORANGE", color::consts::LIGHT_ORANGE.clone());
    m.add("LIGHT_TEAL", color::consts::LIGHT_TEAL.clone());
    m.add("LIGHT_PINK", color::consts::LIGHT_PINK.clone());
    m.add("VIVID_BLUE", color::consts::VIVID_BLUE.clone());
    Ok(())
}
