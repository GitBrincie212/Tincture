import pytest
import tincture

@pytest.mark.parametrize("color,expected", [
    (tincture.Color(1, 2, 3), "(1 : 2 : 3 : 255)"),
    (tincture.Color(12, 13, 30, 201), "(12 : 13 : 30 : 201)"),
    (tincture.Color(223, 18, 50, 0), "(223 : 18 : 50 : 0)"),
    (tincture.Color(192, 82, 152, 92), "(192 : 82 : 152 : 92)"),
])
def test_color_str(color, expected):
    assert str(color) == expected

@pytest.mark.parametrize("color,expected", [
    (tincture.Color(1, 2, 3), "Color(1, 2, 3, 255)"),
    (tincture.Color(12, 13, 30, 201), "Color(12, 13, 30, 201)"),
    (tincture.Color(223, 18, 50, 0), "Color(223, 18, 50, 0)"),
    (tincture.Color(192, 82, 152, 92), "Color(192, 82, 152, 92)"),
])
def test_color_repr(color, expected):
    assert repr(color) == expected

@pytest.mark.parametrize("color,expected", [
    (tincture.Color(50, 168, 82), 0.292),
    (tincture.Color(103, 184, 255), 0.443),
    (tincture.Color(103, 184, 255, 0), 0.443),
    (tincture.Color(58, 84, 19), 0.072),
    (tincture.Color(133, 111, 122), 0.177),
    (tincture.Color(255, 255, 255), 1),
    (tincture.Color(0, 0, 0), 0),
    (tincture.Color(255, 255, 255, 255), 1),
    (tincture.Color(0, 0, 0, 255), 0),
])
def test_color_luminance(color, expected):
    assert expected - 0.05 <= color.get_luminance() <= expected + 0.05

@pytest.mark.parametrize("color,expected", [
    (tincture.Color(106, 240, 117), 0.55),
    (tincture.Color(150, 136, 78), 0.48),
    (tincture.Color(45, 30, 250), 0.88),
    (tincture.Color(245, 121, 28), 0.88),
    (tincture.Color.RED, 1),
    (tincture.Color.BLUE, 1),
    (tincture.Color.GREEN, 1),
    (tincture.Color.WHITE, 0),
    (tincture.Color.BLACK, 0),
    (tincture.Color.CYAN, 1),
    (tincture.Color.YELLOW, 1),
])
def test_color_saturation(color, expected):
    assert expected - 0.05 <= color.get_saturation() <= expected + 0.05

@pytest.mark.parametrize("color,color2,diff,expected", [
    (tincture.Color(106, 240, 117), tincture.Color(106, 240, 117), 1, True),
    (tincture.Color.WHITE, tincture.Color.BLACK, 1, False),
    (tincture.Color(254, 228, 112), tincture.Color(254, 228, 112), 3, True),
    (tincture.Color.CYAN, tincture.Color.RED, 255, True),
    (tincture.Color(3, 2, 1, 255), tincture.Color(4, 5, 6, 0), 10, False),
    (tincture.Color(0, 0, 0, 255), tincture.Color(0, 0, 0, 0), 255, True),
])
def test_color_approx_equal(color, color2, diff, expected):
    result = color2.r - diff <= color.r <= color2.r + diff and \
        color2.g - diff <= color.g <= color2.g + diff and \
        color2.b - diff <= color.b <= color2.b + diff and \
        color2.a - diff <= color.a <= color2.a + diff
    assert color.approx_equal(color2, diff, True) == result == expected