from _color import Color
from _blending import BlendingMode

__all__ = [
    "WHITE",
    "BLACK",
    "RED",
    "BLUE",
    "GREEN",
    "YELLOW",
    "CYAN",
    "MAGENTA",
    "PINK",
    "PURPLE",
    "ORANGE",
    "LIME",
    "TEAL",
    "VIVID_BLUE",
    "LIGHT_RED",
    "LIGHT_GREEN",
    "LIGHT_BLUE",
    "LIGHT_CYAN",
    "LIGHT_MAGENTA",
    "LIGHT_YELLOW",
    "LIGHT_PURPLE",
    "LIGHT_LIME",
    "LIGHT_ORANGE",
    "LIGHT_TEAL",
    "LIGHT_PINK",
    "DARK_RED",
    "DARK_BLUE",
    "DARK_GREEN",
    "DARK_YELLOW",
    "DARK_CYAN",
    "DARK_MAGENTA",
    "DARK_ORANGE",
    "DARK_PINK",
    "DARK_PURPLE",
    "DARK_LIME",
    "DARK_TEAL",
    "Color",
    "BlendingMode"
]

WHITE: Color = Color(255, 255, 255, 255)
BLACK: Color = Color(0, 0, 0, 255)
RED: Color = Color(255, 0, 0, 255)
BLUE: Color = Color(0, 0, 255, 255)
GREEN: Color = Color(0, 255, 0, 255)
YELLOW: Color = Color(255, 255, 0, 255)
CYAN: Color = Color(0, 255, 255, 255)
MAGENTA: Color = Color(255, 0, 255, 255)
PINK: Color = Color(255, 0, 125, 255)
PURPLE: Color = Color(125, 0, 255, 255)
ORANGE: Color = Color(255, 125, 0, 255)
LIME: Color = Color(125, 255, 0, 255)
TEAL: Color = Color(0, 255, 125, 255)
VIVID_BLUE: Color = Color(0, 125, 255, 255)
LIGHT_RED: Color = Color(255, 125, 125, 255)
LIGHT_GREEN: Color = Color(125, 255, 125, 255)
LIGHT_BLUE: Color = Color(125, 125, 255, 255)
LIGHT_CYAN: Color = Color(125, 255, 255, 255)
LIGHT_MAGENTA: Color = Color(255, 125, 255, 255)
LIGHT_YELLOW: Color = Color(255, 255, 125, 255)
LIGHT_PURPLE: Color = Color(203, 195, 227, 255)
LIGHT_LIME: Color = Color(174, 253, 108, 255)
LIGHT_ORANGE: Color = Color(255, 213, 128, 255)
LIGHT_TEAL: Color = Color(144, 228, 193, 255)
LIGHT_PINK: Color = Color(255, 182, 193, 255)
DARK_RED: Color = Color(125, 62, 62, 255)
DARK_BLUE: Color = Color(62, 62, 125, 255)
DARK_GREEN: Color = Color(62, 125, 62, 255)
DARK_YELLOW: Color = Color(125, 125, 62, 255)
DARK_CYAN: Color = Color(62, 125, 125, 255)
DARK_MAGENTA: Color = Color(125, 62, 125, 255)
DARK_ORANGE: Color = Color(125, 93, 62, 255)
DARK_PINK: Color = Color(125, 62, 93, 255)
DARK_PURPLE: Color = Color(93, 62, 125, 255)
DARK_LIME: Color = Color(93, 125, 62, 255)
DARK_TEAL: Color = Color(62, 125, 93, 255)