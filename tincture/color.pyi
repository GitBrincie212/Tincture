class Color:
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

    DEFAULT_BACKGROUND: Color = Color(27, 29, 33, 255)


    def __init__(self, r: int, g: int, b: int, a: int = 255) -> None:
        """
        Color stores RGBA values that can be freely manipulated with all sorts of operation.
        These operations can change the color or can get specific values such as saturation
        in the process. You can convert from any color space to RGBA or even from RGBA to another color space

        :param r: The Red Component Of The Color.
        :param g: The Green Component Of The Color.
        :param b: The Blue Component Of The Color.
        :param a: The Alpha Component Of The Color
        """

    @staticmethod
    def from_srgb(r: int, g: int, b: int) -> "Color":
        """
         Construct a new color based on provided RGB values. This is mostly intended
         to be a function to be used in case it isn't known that this can also be done

         ```
         Color(213, 85, 120)
         ```

         It accepts **u8** parameters for the R, G, B. If you are looking for decimal values,
         then there is a function for that, just called: [from_decimal_rgba]

        :param r: The Red Component Of The Color
        :param g: The Green Component Of The Color
        :param b: The Blue Component Of The Color
        """
        ...

    @staticmethod
    def from_decimal_rgba(r: float, g: float, b: float) -> "Color":
        """
        Construct a new color based on provided Decimal RGB values. These values
        (R, G, B) are percentages that range from 0.0 to 1.0 (inclusion on both).
        If the values do not meet the specified range, then the code will return an error

        :param r: The Red Component Of The Color As Percentage (from 0.0 to 1.0)
        :param g: The Green Component Of The Color As Percentage (from 0.0 to 1.0)
        :param b: The Blue Component Of The Color As Percentage (from 0.0 to 1.0)
        """
        ...

    @staticmethod
    def from_cmyk(c: float, y: float, m: float, k: float, transparency: bool) -> "Color":
        """
        Construct a new color based on provided CMYK values. Where it
        is an acronym for **C**yan, Magenta, Yellow, and Key (Black).
        The values supplied should be in the range of 0.0 to 1.0 (inclusion on both)
        since they are percentage values. Otherwise, the code will return an error

        :param c: The Cyan Component Of The CMYK Color As Percentage (from 0.0 to 1.0)
        :param y: The Yellow Component Of The CMYK Color As Percentage (from 0.0 to 1.0)
        :param m: The Magenta Component Of The CMYK Color As Percentage (from 0.0 to 1.0)
        :param k: The Key (also known as Black) Component Of The CMYK Color As Percentage (from 0.0 to 1.0)
        :param transparency : The transparency value that ranges from [0.0, 1.0]
        """
        ...

    @staticmethod
    def from_xyz(x: float, y: float, z: float, transparency: float) -> "Color":
        """
        Construct a new color based on provided XYZ values. The X value
        ranges from [0.0, 95.047], Y value ranges from [0.0, 100.0], and
        finally Z ranges from [0.0, 108.883]. If they have invalid ranges,
        then the code returns an error

        :param x: The "X" Component Which Ranges [0.0, 95.047]
        :param y: The "Y" Component Which Ranges [0.0, 100.0]
        :param z: The "Z" Component Which Ranges [0.0, 108.883]
        :param transparency : The transparency value that ranges from [0.0, 1.0]
        """
        ...

    @staticmethod
    def from_hsv(h: int, s: float, v: float, transparency: float) -> "Color":
        """
        Construct a new color based on provided CMYK values.
        Where it is an acronym for Hue, Saturation, and Value.
        The s and v values supplied should be in the range of 0.0 to 1.0 (inclusion on both)
        since they are percentage values. Otherwise, the code will return an error

        Note: the hue value will be rounded to the range [0, 360] if it is negative

        :param h: The Hue Component Which is In Degrees And Ranges [0, 360]
        :param s: The Saturation Component As A Percentage (from 0.0 to 1.0)
        :param v: The Value Component As A Percentage (from 0.0 to 1.0)
        :param transparency : The transparency value that ranges from [0.0, 1.0]
        """
        ...

    @staticmethod
    def from_hsl(h: int, s: float, l: float, transparency: float) -> "Color":
        """
        Construct a new color based on provided HSL values.
        Where it is an acronym for Hue, Saturation, and Lightness.
        The s and l values supplied should be in the range of 0.0 to 1.0 (inclusion on both)
        since they are percentage values. Otherwise, the code will return an error

        Note: the hue value will be rounded to the range [0, 360] if it is negative

        :param h: The Hue Component Which is In Degrees And Ranges [0, 360]
        :param s: The Saturation Component As A Percentage (from 0.0 to 1.0)
        :param l: The Lightness Component As A Percentage (from 0.0 to 1.0)
        :param transparency: The transparency value that ranges from [0.0, 1.0]
        """
        ...

    @staticmethod
    def from_lch(l: float, c: float, h: int, transparency: float) -> "Color":
        """
        Construct a new color based on provided LCH values.
        Where it is an acronym for Lightness, Chroma, and Hue.
        The hue ranges from [0, 360], the chroma ranges from [0, 200],
        and the lightness ranges from [0, 100]

        Note: the hue value will be rounded to the range [0, 360] if it is negative

        :param l: The Lightness Component Which Ranges From [0, 100]
        :param c: The Chroma Component Which Ranges From [0, 200]
        :param h: The Hue Component As A Degrees Ranging From [0, 360]
        :param transparency : The transparency value that ranges from [0.0, 1.0]
        """
        ...

    @staticmethod
    def from_hex(hex_string: str) -> "Color":
        """
        Construct a new color based on provided Hex String. Where it
        is a short form for Hexadecimal. Strings can have a leading "#"
        which will be automatically ignored, and they have to be valid hexadecimal
        digits in order for the conversation to happen. Otherwise, an error would be thrown

        :param hex_string: This is the hex string that is being inputted
        """
        ...

    @staticmethod
    def from_oklab(l: float, a: float, b: float, transparency: float) -> "Color":
        """
        Construct a new color based on provided Oklab values. Where it
        "l" ranges from [0.0, 1.0] as well as "a" and b ranges from [-0.5, 0.5]

        :param l: The "L" Component Ranges [0.0, 1.0]
        :param a: The "A" Component Ranges [-0.5, 0.5]
        :param b: The "B" Component Ranges [-0.5, 0.5]
        :param transparency : The transparency value that ranges from [0.0, 1.0]
        """
        ...

    @staticmethod
    def mlerp(start: "Color", end: "Color", t: float) -> "Color":
        """
        Construct a new color based on a mathematical lerp(linear interpolation). Given a
        starting color, an ending color and a **t** value which is a percentage value. It creates
        the color half-way. If the t value is out of range, then it throws an error

        **Important Note:** This version interpolates the 3 RGB channels which is prone to
        some errors, for a better alternative it is best to use [clerp].

        Note: There is a function that does this in place as opposed to generating a new color value.
        The function is called [mlerp_inplace]

        :param start: The beginning color that will be used in the operation.
        :param end: The ending color that will be used
        :param t: A "t" value that is a percentage and is used to produce the intermediate color
        """

    @staticmethod
    def clerp(start: Color, end: Color, t: float) -> Color:
        """
        Construct a new color based on a more accurate color lerp model but a bit heavier to compute.
        Given a starting color, an ending color and a **t** value which is a percentage value. It creates
        the color half-way. If the t value is out of range, then it throws an error

        Important Note: This differs from [mlerp] which interpolates the 3 RGB channels.
        The operation uses the LCH color space which can be.a tiny bit heavier on the computer

        Note: There is a function that does this in place as opposed to generating a new color value.
        The function is called [clerp_inplace]

        :param start: The beginning color that will be used in the operation.
        :param end: The ending color that will be used
        :param t: A "t" value that is a percentage and is used to produce the intermediate color
        """
        ...

    def clerp_inplace(self, end: "Color", t: float) -> None:
        """
        Perform a more accurate color lerp version of mlerp operation on this specific color and
        modify the RGB channels. It needs an ending color and a **t** value, which is a percentage value. It creates
        the color half-way. If the t value is out of range, then it throws an error

        Note: There is a function that generates a new color value as opposed to perform in place.
        The function is called [clerp]

        **Important Note:** This differs from [mlerp] which interpolates the 3 RGB channels.
        The operation uses the LCH color space which might involve a tiny bit of more computation to lerp
        as opposed to [mlerp] but produces a more pleasing result at the end

        :param end: The ending color that will be used
        :param t: A "t" value that is a percentage and is used to produce the intermediate color
        """
        ...

    def mlerp_inplace(self, end: Color, t: float) -> None:
        """
        Perform an RGB color lerp operation on this specific color and modify the RGB channels.
        It needs an ending color and a **t** value, which is a percentage value. It creates
        the color half-way. If the t value is out of range, then it throws an error

        **Important Note:** This version interpolates the 3 RGB channels which is prone to
        some errors, for a better alternative it is best to use [clerp_inplace].

        Note: There is a function that generates a new color value as opposed to perform in place.
        The function is called [mlerp]

        :param end: The ending color that will be used
        :param t: A "t" value that is a percentage and is used to produce the intermediate color
        """
        ...

    def add(self, other: "Color", include_transparency: bool = False) -> "Color":
        """
        Performs an addition operation between this color and the other color, then it returns
        a new color value. The RGB values are maxed to 255, for subtraction it is recommended
        to use the dedicated function called [Self::sub] since the RGB values cannot below 0.
        If you want, you can opt in to include the alpha channel as well

        :param other : The other color for the subtraction operation
        :param include_transparency : Performs the operation in addition to the alpha channel when set to true;
        By default, it is set to be false
        """
        ...

    def sub(self, other: "Color", include_transparency: bool = False) -> "Color":
        """
        Performs a subtraction operation between this color and the other color, then it returns
        a new color value. The RGB values are minimized to 0, for adding; it is recommended
        to use the dedicated function called [Self::add] since the RGB values cannot below 0.
        If you want, you can opt in to include the alpha channel as well

        :param other : The other color for the subtraction operation
        :param include_transparency : Performs the operation in addition to the alpha channel when set to true;
        By default, it is set to be false
        """
        ...

    def mul(self, scalar: float,  include_transparency: bool = False) -> "Color":
        """
        Performs a multiplication operation between this color and a scalar value, then it returns
        a new color value. The RGB values are clamped to the range of 0.0 and 255.0 (including both),
        the values are floored when multiplied by this scalar value. For division checkout [div].
        If you want, you can opt in to include the alpha channel as well

        :param scalar : The scalar value for the multiplication operation
        :param include_transparency : Performs the operation in addition to the alpha channel when set to true;
        By default, it is set to be false
        """
        ...

    def div(self, scalar: float, include_transparency: bool = False) -> "Color":
        """
        Performs a division operation between this color and a scalar value, then it returns
        a new color value. The RGB values are clamped to the range of 0.0 and 255.0 (including both),
        the values are floored when multiplied by this scalar value. If the scalar value is zero, then
        it throws an error indicating division by zero. For multiplication check out [mul]. If you want,
        you can opt in to include the alpha channel as well

        :param scalar : The scalar value for the division operation
        :param include_transparency : Performs the operation in addition to the alpha channel when set to true;
        By default, it is set to be false
        """
        ...

    def tensor(self, other: "Color", include_transparency: bool = False) -> "Color":
        """
        Performs a tensor operation between this color and another color, then it returns
        a new color value. The RGB values are clamped to the range of 0.0 and 255.0 (including both),
        each RGB channel is multiplied by the corresponding other RGB channel (R * R, G * G, B * B).
        If you want, you can opt in to include the alpha channel as well

        :param other : The other color to perform the tensor product operation with
        :param include_transparency : Performs the operation in addition to the alpha channel when set to true;
        By default, it is set to be false
        """
        ...

    def base_sqrt(self, base: int, include_transparency: bool = False) -> "Color":
        """
        Performs an n-th root operation between this color and another color, then it returns
        a new color value. The RGB values are clamped to the range of 0.0 and 255.0 (including both),
        The base of the square root has to be above 1 otherwise an error will be thrown. You can opt in
        if you want to include the alpha channel as well

        :param base: The square root base
        :param include_transparency : Performs the operation in addition to the alpha channel when set to true;
        By default, it is set to be false
        """
        ...

    def max(self, other: "Color", include_transparency: bool = False) -> "Color":
        """
        Performs the max operation on all 3 RGB channels together with another color to create
        a new color instance, this operation is mostly a short-form way to write it out, and nothing
        too fancy is happening under the hood. If you want, you can also include the alpha channel to be inverted

        :param other : The other color for the max operation.
        :param include_transparency : Performs the operation in addition to the alpha channel when set to true;
        By default, it is set to be false
        """
        ...

    def min(self, other: "Color", include_transparency: bool = False) -> "Color":
        """
        Performs the min operation on all 4 RGB channels together with another color to create
        a new color instance, this operation is mostly a short-form way to write it out, and nothing
        too fancy is happening under the hood. If you want, you can also include the alpha channel to be inverted

        :param other : The other color for the min operation.
        :param include_transparency : Performs the operation in addition to the alpha channel when set to true;
        By default, it is set to be false
        """
        ...

    def inverse(self, include_transparency: bool = False) -> "Color":
        """
        Performs a color inversion. This inverts all the 3 RGB channels.
        If you want, you can also include the alpha channel to be inverted

        :param include_transparency : Performs the operation in addition to the alpha channel when set to true;
        By default, it is set to be false
        """
        ...

    def grayscale(self) -> "Color":
        """ Performs a grayscale operation. This basically grayscales the color """
        ...

    def triadic_colors(self) -> list["Color"]:
        """
        Gets the two triadic colors based on this color, the tetradic colors have a difference of hue 120 degrees
        from the color used in this operation; it uses the HSL color space to do this specific operation which sometimes
        (very rarely) may not be accurate
        """
        ...

    def adjust_temperature(self, temperature: int):
        """
        Adjusts the temperature of the color where positive temperature makes the color warmer while
        negative temperature makes the color colder. With temperature zero, it does nothing to the color
        """
        ...

    def contrast(self, factor: float):
        """
        Adjusts the contrast of the color, the factor value is a percentage value and can range from [-1, 1] where
        negative numbers decrease the contrast and make the color grayer. Positive values add more contrast, and when
        the factor is equal to zero, it has no influence
        """
        ...

    def brightness(self, factor: float) -> "Color":
        """
        Performs a brightness adjustment operation. Where it adjusts the brightness of the Color.
        The factor value can range from [-1.0, 1.0] where 0.0 has no influence on the color,
        positive values brighten the color and negative values darken the color

        :param factor: The factor that influences the color's brightness. Where 0.0 has no influence, ranges [-1.0, 1.0]
        """
        ...

    def tint(self, degrees: int) -> "Color":
        """
        Performs a tint / hue shifting operation. Where it adjusts the hue of the color
        based on the provided degrees which is in the range of [0, 360] and
        is automatically rounded to that range

        Note: For simpler implementation it converts to HSV color space, modifies
        then back to RGB color space so it can be a bit expensive sometimes

        :param degrees: The degree offset for the hue
        """
        ...

    def saturate(self, factor: float) -> "Color":
        """
        Performs a saturation operation. Where it adjusts the saturation of the color
        based on the provided factor which is in the range of [-1.0, 1.0]. 0.0 Has no influence
        whereas positive numbers saturate more and negative numbers desaturate

        Note: For simpler implementation it converts to HSV color space, modifies
        then back to RGB color space so it can be a bit expensive sometimes
        """
        ...

    def randomise(self, start: list[int | None], end: list[int | None]) -> "Color":
        """
        Creates a randomized color based on the ranges provided. It's iterating both lists and indexing to the elements,
        so for the first field it grabs start[0] and end[0] then so on. If both fields are set to None, then there will be
        no randomization, it will default to the self-color value for this field while repeating for the other fields

        Note: The start range must be lower than the ending range in order for this to work otherwise it will error out

        Note: start and end must be both int or both None otherwise an error will appear

        :param start: The starting range for the randomization
        :param end: The ending range for the randomization
        """
        ...

    def get_luminance(self) -> float:
        """
        Gets the brightness / luminance of a color and returns a percentage value ranging
        from [0.0, 1.0] indicating how bright / how much luminance the color is
        """
        ...

    def get_saturation(self) -> float:
        """
        Gets the saturation of a color and returns a percentage value ranging
        from [0.0, 1.0] indicating how much saturated or desaturated the color is
        """
        ...

    def approx_equal(self, other: "Color", diff: int, include_transparency: bool = True) -> bool:
        """
        Returns true if the color is approximately / partially equal to the other color.
        Given this color with another color and a difference, it returns a boolean value
        indicating if its in fact partially equal, bigger difference values result in less precision
        and vice versa, with the smaller difference being more precise. Unlike normal equals, this
        operation is not commutative, which means


        col1.approx_equal(col2, diff) != col2.approx_equal(col1, diff)


        Note: If the colors are equal, it will output True

        :param other: The other color to compare against
        :param diff: The difference between this and the other color. Has to be in range [0, 255]
        :param include_transparency: Whenever or not to check the alpha channel as well. By default, it's set to true
        """
        ...

    def to_hex(self, include_transparency: bool) -> str:
        """
        Converts the color object into a Hexadecimal string format
        where it contains a leading "#" and then the hexadecimal parts.

        :param include_transparency: Whenever to include the transparent part on the HEX
        """
        ...

    def to_lch(self) -> str:
        """
        Converts the color object into the Lightness Chroma Hue color space format.
        Where lightness and chroma are percentage values that range from [0.0, 1.0] while hue
        is an angle value that ranges from [0.0, 360.0]. Includes transparency as a field
        """
        ...

    def to_hsv(self) -> tuple[int, float, float]:
        """
        Converts the color object into the Hue, Saturation, Value
        color space where hue is as an angle, and the saturation and value
        are percentage values that range from [0.0, 1.0]. Includes transparency as a field
        """
        ...

    def to_hsl(self) -> tuple[int, float, float]:
        """
        Converts the color object into the **H**ue, **S**aturion, **L**ightness
        color space where hue is as an angle, and the saturation and lightness
        are percentage values that range from [0.0, 1.0]. Includes transparency as a field
        """
        ...

    def to_decimal_rgba(self) -> tuple[float]:
        """
        Converts the color object into the decimal RGB color space format
        where all the R, G, B, A values are percentage values that range from
        [0.0, 1.0]
        """
        ...

    def to_decimal_rgb(self) -> tuple[float]:
        """
        Converts the color object into the decimal RGB color space format
        where all the R, G, B values are percentage values that range from
        [0.0, 1.0]
        """
        ...

    def to_cmyk(self) -> tuple[float]:
        """
        Converts the color object into the Cyan Magenta Yellow Key (Black)
        color space format. Where all the values are percentage values that
        range from [0.0, 1.0]. Includes transparency as a field
        """
        ...

    def to_xyz(self) -> tuple[float]:
        """
        Converts the color object into the XYZ color space format. Where
        The X values ranges from [0.0, 95.047], Y value ranges from [0.0, 100.0],
        and finally Z ranges from [0.0, 108.883]. Includes transparency as a field
        """
        ...

    def to_oklab(self) -> tuple[float]:
        """
        Converts the color object into the Lab color space format. Where
        "L" represents darkness to lightness ranging from [0.0, 100.0], "A" represents
        greenish to redness and ranges from [-128.0, 127.0], and finally "B" represents
        blueness to yellowness and ranges from [-128.0, 127.0]. Includes transparency as a field
        """
        ...

    def to_rgba_list(self) -> list[int]:
        """Converts the color object into a list that contains the RGBA values from [0, 255]"""
        ...

    def to_decimal_rgba_list(self) -> list[float]:
        """Converts the color object into a list that contains the Decimal RGBA values from [0, 1.0]"""
        ...

    def to_rgba_tuple(self) -> tuple[int]:
        """Converts the color object into a tuple that contains the RGBA values from [0, 255]"""
        ...

    def __add__(self, other: "Color") -> "Color": ...
    def __sub__(self, other: "Color") -> "Color": ...
    def __mul__(self, factor: float) -> "Color": ...
    def __truediv__(self, factor: float) -> "Color": ...
    def __floordiv__(self, factor: int) -> "Color": ...
    def __hash__(self) -> int: ...
    def __nonzero__(self) -> int: ...
    def __neg__(self) -> "Color": ...
    def __str__(self) -> str: ...
    def __repr__(self) -> str: ...
    def __pow__(self, color: "Color", base: float) -> "Color": ...
    def __rpow__(self, color: "Color", base: float) -> "Color": ...
    def __getitem__(self, a: str | int) -> int: ...
    def __setitem__(self, a: str | int, b: int) -> int: ...
    def __rshift__(self, places: int) -> "Color": ...
    def __lshift__(self, places: int) -> "Color": ...