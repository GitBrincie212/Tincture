use crate::{create_color};
use crate::color::Color;
use crate::color::AtomicU32;

pub static WHITE: Color = create_color!(u32::from_be_bytes([
    255,
    255,
    255,
    255,
]));

pub static BLACK: Color = create_color!(u32::from_be_bytes([
    0,
    0,
    0,
    255,
]));
pub static RED: Color = create_color!(u32::from_be_bytes([
    255,
    0,
    0,
    255,
]));
pub static BLUE: Color = create_color!(u32::from_be_bytes([
    0,
    0,
    255,
    255,
]));
pub static GREEN: Color = create_color!(u32::from_be_bytes([
    0,
    255,
    0,
    255,
]));
pub static YELLOW: Color = create_color!(u32::from_be_bytes([
    255,
    255,
    0,
    255,
]));
pub static CYAN: Color = create_color!(u32::from_be_bytes([
    0,
    255,
    255,
    255,
]));
pub static MAGENTA: Color = create_color!(u32::from_be_bytes([
    255,
    0,
    255,
    255,
]));
pub static PINK: Color = create_color!(u32::from_be_bytes([
    255,
    0,
    125,
    255,
]));
pub static PURPLE: Color = create_color!(u32::from_be_bytes([
    125,
    0,
    255,
    255,
]));
pub static ORANGE: Color = create_color!(u32::from_be_bytes([
    255,
    125,
    0,
    255,
]));
pub static LIME: Color = create_color!(u32::from_be_bytes([
    125,
    255,
    0,
    255,
]));
pub static TEAL: Color = create_color!(u32::from_be_bytes([
    0,
    255,
    125,
    255,
]));
pub static VIVID_BLUE: Color = create_color!(u32::from_be_bytes([
    0,
    125,
    255,
    255,
]));
pub static LIGHT_RED: Color = create_color!(u32::from_be_bytes([
    255,
    125,
    125,
    255,
]));
pub static LIGHT_GREEN: Color = create_color!(u32::from_be_bytes([
    125,
    255,
    125,
    255,
]));
pub static LIGHT_BLUE: Color = create_color!(u32::from_be_bytes([
    125,
    125,
    255,
    255,
]));
pub static LIGHT_CYAN: Color = create_color!(u32::from_be_bytes([
    125,
    255,
    255,
    255,
]));
pub static LIGHT_MAGENTA: Color = create_color!(u32::from_be_bytes([
    255,
    125,
    255,
    255,
]));
pub static LIGHT_YELLOW: Color = create_color!(u32::from_be_bytes([
    255,
    255,
    125,
    255,
]));

pub static LIGHT_PURPLE: Color = create_color!(u32::from_be_bytes([
    203,
    195,
    227,
    255,
]));

pub static LIGHT_LIME: Color = create_color!(u32::from_be_bytes([
    174,
    253,
    108,
    255,
]));
pub static LIGHT_ORANGE: Color = create_color!(u32::from_be_bytes([
    255,
    213,
    128,
    255,
]));
pub static LIGHT_TEAL: Color = create_color!(u32::from_be_bytes([
    144,
    228,
    193,
    255,
]));
pub static LIGHT_PINK: Color = create_color!(u32::from_be_bytes([
    255,
    182,
    193,
    255,
]));

pub static DARK_RED: Color = create_color!(u32::from_be_bytes([
    125,
    62,
    62,
    255,
]));
pub static DARK_BLUE: Color = create_color!(u32::from_be_bytes([
    62,
    62,
    125,
    255,
]));
pub static DARK_GREEN: Color = create_color!(u32::from_be_bytes([
    62,
    125,
    62,
    255,
]));
pub static DARK_YELLOW: Color = create_color!(u32::from_be_bytes([
    125,
    125,
    62,
    255,
]));
pub static DARK_CYAN: Color = create_color!(u32::from_be_bytes([
    62,
    125,
    125,
    255,
]));
pub static DARK_MAGENTA: Color = create_color!(u32::from_be_bytes([
    125,
    62,
    125,
    255,
]));
pub static DARK_ORANGE: Color = create_color!(u32::from_be_bytes([
    125,
    93,
    62,
    255,
]));
pub static DARK_PINK: Color = create_color!(u32::from_be_bytes([
    125,
    62,
    93,
    255,
]));
pub static DARK_PURPLE: Color = create_color!(u32::from_be_bytes([
    93,
    62,
    125,
    255,
]));
pub static DARK_LIME: Color = create_color!(u32::from_be_bytes([
    93,
    125,
    62,
    255,
]));
pub static DARK_TEAL: Color = create_color!(u32::from_be_bytes([
    62,
    125,
    93,
    255,
]));