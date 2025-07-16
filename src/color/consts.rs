use crate::color;

pub const WHITE: color::Color = color::Color(u32::from_be_bytes([
    255,
    255,
    255,
    255,
]));

pub const BLACK: color::Color = color::Color(u32::from_be_bytes([
    0,
    0,
    0,
    255,
]));
pub const RED: color::Color = color::Color(u32::from_be_bytes([
    255,
    0,
    0,
    255,
]));
pub const BLUE: color::Color = color::Color(u32::from_be_bytes([
    0,
    0,
    255,
    255,
]));
pub const GREEN: color::Color = color::Color(u32::from_be_bytes([
    0,
    255,
    0,
    255,
]));
pub const YELLOW: color::Color = color::Color(u32::from_be_bytes([
    255,
    255,
    0,
    255,
]));
pub const CYAN: color::Color = color::Color(u32::from_be_bytes([
    0,
    255,
    255,
    255,
]));
pub const MAGENTA: color::Color = color::Color(u32::from_be_bytes([
    255,
    0,
    255,
    255,
]));
pub const PINK: color::Color = color::Color(u32::from_be_bytes([
    255,
    0,
    125,
    255,
]));
pub const PURPLE: color::Color = color::Color(u32::from_be_bytes([
    125,
    0,
    255,
    255,
]));
pub const ORANGE: color::Color = color::Color(u32::from_be_bytes([
    255,
    125,
    0,
    255,
]));
pub const LIME: color::Color = color::Color(u32::from_be_bytes([
    125,
    255,
    0,
    255,
]));
pub const TEAL: color::Color = color::Color(u32::from_be_bytes([
    0,
    255,
    125,
    255,
]));
pub const VIVID_BLUE: color::Color = color::Color(u32::from_be_bytes([
    0,
    125,
    255,
    255,
]));
pub const LIGHT_RED: color::Color = color::Color(u32::from_be_bytes([
    255,
    125,
    125,
    255,
]));
pub const LIGHT_GREEN: color::Color = color::Color(u32::from_be_bytes([
    125,
    255,
    125,
    255,
]));
pub const LIGHT_BLUE: color::Color = color::Color(u32::from_be_bytes([
    125,
    125,
    255,
    255,
]));
pub const LIGHT_CYAN: color::Color = color::Color(u32::from_be_bytes([
    125,
    255,
    255,
    255,
]));
pub const LIGHT_MAGENTA: color::Color = color::Color(u32::from_be_bytes([
    255,
    125,
    255,
    255,
]));
pub const LIGHT_YELLOW: color::Color = color::Color(u32::from_be_bytes([
    255,
    255,
    125,
    255,
]));

pub const LIGHT_PURPLE: color::Color = color::Color(u32::from_be_bytes([
    203,
    195,
    227,
    255,
]));

pub const LIGHT_LIME: color::Color = color::Color(u32::from_be_bytes([
    174,
    253,
    108,
    255,
]));
pub const LIGHT_ORANGE: color::Color = color::Color(u32::from_be_bytes([
    255,
    213,
    128,
    255,
]));
pub const LIGHT_TEAL: color::Color = color::Color(u32::from_be_bytes([
    144,
    228,
    193,
    255,
]));
pub const LIGHT_PINK: color::Color = color::Color(u32::from_be_bytes([
    255,
    182,
    193,
    255,
]));

pub const DARK_RED: color::Color = color::Color(u32::from_be_bytes([
    125,
    62,
    62,
    255,
]));
pub const DARK_BLUE: color::Color = color::Color(u32::from_be_bytes([
    62,
    62,
    125,
    255,
]));
pub const DARK_GREEN: color::Color = color::Color(u32::from_be_bytes([
    62,
    125,
    62,
    255,
]));
pub const DARK_YELLOW: color::Color = color::Color(u32::from_be_bytes([
    125,
    125,
    62,
    255,
]));
pub const DARK_CYAN: color::Color = color::Color(u32::from_be_bytes([
    62,
    125,
    125,
    255,
]));
pub const DARK_MAGENTA: color::Color = color::Color(u32::from_be_bytes([
    125,
    62,
    125,
    255,
]));
pub const DARK_ORANGE: color::Color = color::Color(u32::from_be_bytes([
    125,
    93,
    62,
    255,
]));
pub const DARK_PINK: color::Color = color::Color(u32::from_be_bytes([
    125,
    62,
    93,
    255,
]));
pub const DARK_PURPLE: color::Color = color::Color(u32::from_be_bytes([
    93,
    62,
    125,
    255,
]));
pub const DARK_LIME: color::Color = color::Color(u32::from_be_bytes([
    93,
    125,
    62,
    255,
]));
pub const DARK_TEAL: color::Color = color::Color(u32::from_be_bytes([
    62,
    125,
    93,
    255,
]));