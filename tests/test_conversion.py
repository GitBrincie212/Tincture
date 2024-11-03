import pytest
import tincture

def approx_equal_field(val1, val2, diff):
    return val2[0] - diff <= val1[0] <= val2[0] + diff and \
        val2[1] - diff <= val1[1] <= val2[1] + diff and \
        val2[2] - diff <= val1[2] <= val2[2] + diff and \
        val2[3] - diff <= val1[3] <= val2[3] + diff

@pytest.mark.parametrize("color,expected", [
    (tincture.RED, [255, 0, 0, 255]),
    (tincture.BLUE, [0, 0, 255, 255]),
    (tincture.GREEN, [0, 255, 0, 255]),
    (tincture.MAGENTA, [255, 0, 255, 255]),
    (tincture.Color(10, 20, 30, 40), [10, 20, 30, 40]),
    (tincture.BLACK, [0, 0, 0, 255]),
    (tincture.Color(92, 102, 31, 65), [92, 102, 31, 65]),
    (tincture.WHITE, [255, 255, 255, 255]),
    (tincture.Color(115, 115, 115), [115, 115, 115, 255]),
    (tincture.Color(44, 33, 22, 11), [44, 33, 22, 11]),
    (tincture.Color(92, 123, 34, 239), [92, 123, 34, 239])
])
def test_color_rgba(color, expected):
    assert color.to_rgba_tuple() == tuple(expected)
    assert color.to_rgba_list() == expected

@pytest.mark.parametrize("color,expected", [
    (tincture.RED, [1.0, 0, 0, 1.0]),
    (tincture.BLUE, [0, 0, 1.0, 1.0]),
    (tincture.GREEN, [0.0, 1.0, 0.0, 1.0]),
    (tincture.MAGENTA, [1.0, 0, 1.0, 1.0]),
    (tincture.Color(10, 20, 30, 40), [0.0392, 0.0784, 0.11764, 0.15686]),
    (tincture.BLACK, [0.0, 0.0, 0.0, 1.0]),
    (tincture.Color(92, 102, 31, 65), [0.36078, 0.4, 0.12156, 0.25490]),
    (tincture.WHITE, [1.0, 1.0, 1.0, 1.0]),
    (tincture.Color(115, 115, 115), [0.45098, 0.45098, 0.45098, 1.0]),
    (tincture.Color(44, 33, 22, 11), [0.172549, 0.1294117, 0.0862745, 0.043137]),
    (tincture.Color(92, 123, 34, 239), [0.3607, 0.482352, 0.133333, 0.937254])
])
def test_color_decimal_rgba(color, expected):
    result_tuple = color.to_decimal_rgba()
    result_list = color.to_decimal_rgba_list()
    assert approx_equal_field(result_tuple, tuple(expected), 0.05)
    assert approx_equal_field(result_list, expected, 0.05)

@pytest.mark.parametrize("color,expected", [
    (tincture.RED, (0.0, 1.0, 1.0, 0.0, 1.0)),
    (tincture.GREEN, (1.0, 0.0, 1.0, 0.0, 1.0)),
    (tincture.MAGENTA, (0, 1.0, 0.0, 0.0, 255)),
    (tincture.BLUE, (1.0, 1.0, 0.0, 0.0, 1.0)),
    (tincture.Color(10, 20, 30, 40), (0.67, 0.33, 0.0, 0.88, 0.15686)),
    (tincture.BLACK, (0.0, 0.0, 0.0, 1.0, 1.0)),
    (tincture.Color(92, 102, 31, 65), (0.09, 0.0, 0.70, 0.60, 0.25490)),
    (tincture.WHITE, (0.0, 0.0, 0.0, 0.0, 1.0)),
    (tincture.Color(115, 115, 115), (0.0, 0.0, 0.0, 0.55, 1.0)),
    (tincture.Color(44, 33, 22, 11), (0.0, 0.25, 0.50, 0.83, 0.043137)),
    (tincture.Color(92, 123, 34, 239), (0.25, 0.0, 0.72, 0.52, 0.937254))
])
def test_color_cmyk(color, expected):
    result = color.to_cmyk()
    assert approx_equal_field(result, tuple(expected), 0.05)

@pytest.mark.parametrize("color,expected", [
    (tincture.RED, (0, 1.0, 1.0, 1.0)),
    (tincture.GREEN, (120, 1.0, 1.0, 1.0)),
    (tincture.BLUE, (240, 1.0, 1.0, 1.0)),
    (tincture.MAGENTA, (300, 1.0, 1.0, 1.0)),
    (tincture.Color(10, 20, 30, 40), (210, 0.67, 0.12, 0.15686)),
    (tincture.BLACK, (0.0, 0.0, 0.0, 1.0, 1.0)),
    (tincture.Color(92, 102, 31, 65), (68, 0.7, 0.4, 0.25490)),
    (tincture.WHITE, (0, 0.0, 1.0, 1.0)),
    (tincture.Color(115, 115, 115), (0, 0.0, 0.45, 1.0)),
    (tincture.Color(44, 33, 22, 11), (30, 0.5, 0.17, 0.043137)),
    (tincture.Color(92, 123, 34, 239), (81, 0.72, 0.48, 0.937254))
])
def test_color_hsv(color, expected):
    result = color.to_hsv()
    assert approx_equal_field(result, expected, 0.05)

@pytest.mark.parametrize("color,expected", [
    (tincture.RED, (0, 1.0, 0.5, 1.0)),
    (tincture.GREEN, (120, 1.0, 0.5, 1.0)),
    (tincture.MAGENTA, (300, 1.0, 0.5, 1.0)),
    (tincture.BLUE, (240, 1.0, 0.5, 1.0)),
    (tincture.Color(10, 20, 30, 40), (210, 0.5, 0.08, 0.15686)),
    (tincture.BLACK, (0.0, 0.0, 0.0, 1.0, 1.0)),
    (tincture.Color(92, 102, 31, 65), (68, 0.53, 0.26, 0.25490)),
    (tincture.WHITE, (0, 0.0, 1.0, 1.0)),
    (tincture.Color(115, 115, 115), (0, 0.0, 0.45, 1.0)),
    (tincture.Color(44, 33, 22, 11), (30, 0.33, 0.13, 0.043137)),
    (tincture.Color(92, 123, 34, 239), (81, 0.57, 0.31, 0.937254))
])
def test_color_hsl(color, expected):
    result = color.to_hsl()
    assert approx_equal_field(result, expected, 0.05)

