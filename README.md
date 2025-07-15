<h1 style="text-align: center;">Tincture Python Color Library</h1>

![tincture](./assets/tincture_2560x1280.png)

### DISCLAIMER
As of now the library is not ready to be used for production. It is currently being developed and tested, only once it
gets published in PyPi it will be ready for production-use
---

Tincture is a relatively small library that provides a flexible color class allowing for all sorts of modifications.
These include color interpolation, temperature adjustment, contrasting, getting luminance of a color, adjusting the
brightness of a color... etc. It also has a wide range of color space conversations either converting from a color space
to a color object or vice versa. Tincture is written in rust to ensure performance over color management

Tincture is mostly meant to be used to provide a fully ready to use color class without having to write the code yourself
in supporting a bunch of operations, methods, conversations... etc. This library will be used throughout some of my other
python libraries to alleviate pain points

---
All the documentation in tincture is written in docstrings. Here are some examples of using tincture

_Basic Usage_

```python
from tincture import Color

# Some Basic Methods To Use(With Way More In The Module)
if __name__ == "__main__":
    color = Color.RED
    color.clerp_inplace(Color.BLUE, 0.3)
    inverse_color = color.inverse(False)
    shifted_color = color << 2
    grayscaled_color = color.grayscale()
    white = color.from_hex("#ffffff")  # or just color.from_hex("ffffff")
    max_color = white.max(Color.BLACK)
    randomised_color = color.WHITE.randomise(
        start=[0, 20, 40, 60],
        end=[40, 30, 50, 80]
    )
    tensored_color = color.DARK_RED.tensor(color.DARK_BLUE)
    added_color = color + inverse_color
    print(color)
    print(repr(color))
```

_Color Transitions Via Lerping_

```python
from tincture import Color

if __name__ == "__main__":
    amount = 100
    start_color = Color(30, 210, 255)
    end_color = Color.LIGHT_CYAN
    for i in range(amount):
        t = i / amount
        start_color.clerp_inplace(end_color, t)
        print(start_color)
```

_Color Conversion_

```python
from tincture import Color

if __name__ == "__main__":
    color = Color.from_hsv(30, 0.6, 1.0, 0.4)
    color2 = Color.from_lch(0.35, 0.83, 42, 0.24)
    color3 = Color.from_cmyk(0.35, 0.83, 0.234, 0.925, 0.32)
    color4 = Color.from_xyz(92.14, 23.321, 45.824, 0.312)
    color5 = Color.from_hex("ffffff")
    print(color.to_xyz())
    print(color.to_hex(include_transparency=False))
    print(color.to_hex(include_transparency=True))
    print(color.to_oklab())
    print(color.to_cmyk())
    print(color.to_decimal_rgb())
    print(color.to_hsl())
```

_Color Data_

```python
from tincture import Color

if __name__ == "__main__":
    color = Color.PURPLE
    print(color.get_saturation())
    print(color.get_luminance())
    print(color.triadic_colors())
```
