#![allow(unused_must_use)]
#![allow(clippy::wrong_self_convention)]

mod color;

use pyo3::prelude::*;

#[pymodule]
#[pyo3(name="tincture")]
fn tincture(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<color::Color>();
    m.add_class::<color::blending::BlendingMode>();
    m.add("WHITE", color::consts::WHITE);
    m.add("RED", color::consts::RED);
    m.add("BLUE", color::consts::BLUE);
    m.add("GREEN", color::consts::GREEN);
    m.add("BLACK", color::consts::BLACK);
    m.add("CYAN", color::consts::CYAN);
    m.add("YELLOW", color::consts::YELLOW);
    m.add("MAGENTA", color::consts::MAGENTA);
    m.add("PURPLE", color::consts::PURPLE);
    m.add("LIME", color::consts::LIME);
    m.add("ORANGE", color::consts::ORANGE);
    m.add("TEAL", color::consts::TEAL);
    m.add("PINK", color::consts::PINK);
    m.add("DARK_RED", color::consts::DARK_RED);
    m.add("DARK_BLUE", color::consts::DARK_BLUE);
    m.add("DARK_GREEN", color::consts::DARK_GREEN);
    m.add("DARK_CYAN", color::consts::DARK_CYAN);
    m.add("DARK_YELLOW", color::consts::DARK_YELLOW);
    m.add("DARK_MAGENTA", color::consts::DARK_MAGENTA);
    m.add("DARK_PURPLE", color::consts::DARK_PURPLE);
    m.add("DARK_LIME", color::consts::DARK_LIME);
    m.add("DARK_ORANGE", color::consts::DARK_ORANGE);
    m.add("DARK_TEAL", color::consts::DARK_TEAL);
    m.add("DARK_PINK", color::consts::DARK_PINK);
    m.add("LIGHT_RED", color::consts::LIGHT_RED);
    m.add("LIGHT_BLUE", color::consts::LIGHT_BLUE);
    m.add("LIGHT_GREEN", color::consts::LIGHT_GREEN);
    m.add("LIGHT_CYAN", color::consts::LIGHT_CYAN);
    m.add("LIGHT_YELLOW", color::consts::LIGHT_YELLOW);
    m.add("LIGHT_MAGENTA", color::consts::LIGHT_MAGENTA);
    m.add("LIGHT_PURPLE", color::consts::LIGHT_PURPLE);
    m.add("LIGHT_LIME", color::consts::LIGHT_LIME);
    m.add("LIGHT_ORANGE", color::consts::LIGHT_ORANGE);
    m.add("LIGHT_TEAL", color::consts::LIGHT_TEAL);
    m.add("LIGHT_PINK", color::consts::LIGHT_PINK);
    m.add("VIVID_BLUE", color::consts::VIVID_BLUE);
    Ok(())
}
