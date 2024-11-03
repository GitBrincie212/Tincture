from enum import Enum, auto

class BlendingMode(Enum):
    """
    Specifies the blending modes to be used when calling :func:`Color.blend() <tincture.Color.blend>`. It serves
    multiple different blending mode values you can pick from. It should be noted that if there are more than three
    colors supplied in the blend operation. Then the blending calculation goes from right to left where it picks
    the first color together with the second color to produce a blended result. This blended result then is used with
    the third color to produce another one, this is repeated until all colors are used
    """

    Darken = auto(),
    Multiply = auto(),
    ColorBurn = auto(),
    LinearBurn = auto(),
    Lighten = auto(),
    Screen = auto(),
    ColorDodge = auto(),
    LinearDodge = auto(),
    Overlay = auto(),
    SoftLight = auto(),
    HardLight = auto(),
    VividLight = auto(),
    LinearLight = auto(),
    PinLight = auto(),
    Difference = auto(),
    Exclusion = auto(),
    Divide = auto(),
    Subtract = auto(),
    Luminosity = auto(),
    Average = auto(),