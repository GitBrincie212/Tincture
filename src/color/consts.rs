use crate::{create_color};
use crate::color::Color;
use crate::color::AtomicU32;

pub const WHITE: Color = create_color!(u32::from_be_bytes([
    255,
    255,
    255,
    255,
]));

pub const BLACK: Color = create_color!(u32::from_be_bytes([
    0,
    0,
    0,
    255,
]));
pub const RED: Color = create_color!(u32::from_be_bytes([
    255,
    0,
    0,
    255,
]));
pub const BLUE: Color = create_color!(u32::from_be_bytes([
    0,
    0,
    255,
    255,
]));
pub const GREEN: Color = create_color!(u32::from_be_bytes([
    0,
    255,
    0,
    255,
]));
pub const YELLOW: Color = create_color!(u32::from_be_bytes([
    255,
    255,
    0,
    255,
]));
pub const CYAN: Color = create_color!(u32::from_be_bytes([
    0,
    255,
    255,
    255,
]));
pub const MAGENTA: Color = create_color!(u32::from_be_bytes([
    255,
    0,
    255,
    255,
]));
pub const PINK: Color = create_color!(u32::from_be_bytes([
    255,
    0,
    125,
    255,
]));
pub const PURPLE: Color = create_color!(u32::from_be_bytes([
    125,
    0,
    255,
    255,
]));
pub const ORANGE: Color = create_color!(u32::from_be_bytes([
    255,
    125,
    0,
    255,
]));
pub const LIME: Color = create_color!(u32::from_be_bytes([
    125,
    255,
    0,
    255,
]));
pub const TEAL: Color = create_color!(u32::from_be_bytes([
    0,
    255,
    125,
    255,
]));
pub const VIVID_BLUE: Color = create_color!(u32::from_be_bytes([
    0,
    125,
    255,
    255,
]));
pub const LIGHT_RED: Color = create_color!(u32::from_be_bytes([
    255,
    125,
    125,
    255,
]));
pub const LIGHT_GREEN: Color = create_color!(u32::from_be_bytes([
    125,
    255,
    125,
    255,
]));
pub const LIGHT_BLUE: Color = create_color!(u32::from_be_bytes([
    125,
    125,
    255,
    255,
]));
pub const LIGHT_CYAN: Color = create_color!(u32::from_be_bytes([
    125,
    255,
    255,
    255,
]));
pub const LIGHT_MAGENTA: Color = create_color!(u32::from_be_bytes([
    255,
    125,
    255,
    255,
]));
pub const LIGHT_YELLOW: Color = create_color!(u32::from_be_bytes([
    255,
    255,
    125,
    255,
]));

pub const LIGHT_PURPLE: Color = create_color!(u32::from_be_bytes([
    203,
    195,
    227,
    255,
]));

pub const LIGHT_LIME: Color = create_color!(u32::from_be_bytes([
    174,
    253,
    108,
    255,
]));
pub const LIGHT_ORANGE: Color = create_color!(u32::from_be_bytes([
    255,
    213,
    128,
    255,
]));
pub const LIGHT_TEAL: Color = create_color!(u32::from_be_bytes([
    144,
    228,
    193,
    255,
]));
pub const LIGHT_PINK: Color = create_color!(u32::from_be_bytes([
    255,
    182,
    193,
    255,
]));

pub const DARK_RED: Color = create_color!(u32::from_be_bytes([
    125,
    62,
    62,
    255,
]));
pub const DARK_BLUE: Color = create_color!(u32::from_be_bytes([
    62,
    62,
    125,
    255,
]));
pub const DARK_GREEN: Color = create_color!(u32::from_be_bytes([
    62,
    125,
    62,
    255,
]));
pub const DARK_YELLOW: Color = create_color!(u32::from_be_bytes([
    125,
    125,
    62,
    255,
]));
pub const DARK_CYAN: Color = create_color!(u32::from_be_bytes([
    62,
    125,
    125,
    255,
]));
pub const DARK_MAGENTA: Color = create_color!(u32::from_be_bytes([
    125,
    62,
    125,
    255,
]));
pub const DARK_ORANGE: Color = create_color!(u32::from_be_bytes([
    125,
    93,
    62,
    255,
]));
pub const DARK_PINK: Color = create_color!(u32::from_be_bytes([
    125,
    62,
    93,
    255,
]));
pub const DARK_PURPLE: Color = create_color!(u32::from_be_bytes([
    93,
    62,
    125,
    255,
]));
pub const DARK_LIME: Color = create_color!(u32::from_be_bytes([
    93,
    125,
    62,
    255,
]));
pub const DARK_TEAL: Color = create_color!(u32::from_be_bytes([
    62,
    125,
    93,
    255,
]));