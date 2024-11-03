import pytest
import tincture

@pytest.mark.parametrize("color1,color2,expected", [
    (tincture.Color(1, 2, 3), tincture.Color(3, 2, 1, 0), tincture.Color(4, 4, 4)),
    (tincture.Color(1, 2, 3), tincture.Color(0, 0, 0, 0), tincture.Color(1, 2, 3)),
    (tincture.Color(1, 2, 3, 20), tincture.Color(0, 0, 0, 25), tincture.Color(1, 2, 3, 45)),
    (tincture.Color(0, 0, 0), tincture.Color(0, 0, 0), tincture.Color(0, 0, 0)),
    (tincture.Color(255, 255, 255), tincture.Color(255, 255, 255), tincture.Color(255, 255, 255)),
])
def test_color_add(color1, color2, expected):
    assert color1 + color2 == expected

@pytest.mark.parametrize("color1,color2,expected", [
    (tincture.Color(10, 20, 30), tincture.Color(3, 2, 1, 0), tincture.Color(7, 18, 29)),
    (tincture.Color(255, 255, 255), tincture.Color(0, 0, 0, 0), tincture.Color(255, 255, 255)),
    (tincture.Color(255, 0, 255), tincture.Color(0, 2, 0, 0), tincture.Color(255, 0, 255)),
    (tincture.Color(255, 0, 0, 0), tincture.Color(0, 2, 0, 255), tincture.Color(255, 0, 0, 0)),
    (tincture.Color(255, 255, 255, 255), tincture.Color(255, 255, 255, 255), tincture.Color(0, 0, 0, 0)),
])
def test_color_sub(color1, color2, expected):
    assert color1 - color2 == expected

@pytest.mark.parametrize("color1,factor,expected", [
    (tincture.Color(10, 21, 32), 2, tincture.Color(20, 42, 64)),
    (tincture.Color(255, 255, 255), 3, tincture.Color(255, 255, 255)),
    (tincture.Color(255, 0, 255), 0, tincture.Color(0, 0, 0, 0)),
    (tincture.Color(255, 255, 255, 255), 255, tincture.Color(255, 255, 255, 255)),
    (tincture.Color(255, 255, 255, 255), 9e20, tincture.Color(255, 255, 255, 255)),
    (tincture.Color(255, 255, 255, 255), 1e-10, tincture.Color(0, 0, 0, 0)),
])
def test_color_mul(color1, factor, expected):
    assert color1 * factor == expected

@pytest.mark.parametrize("color1,factor,expected", [
    (tincture.Color(10, 21, 32), 2, tincture.Color(5, 10, 16, 127)),
    (tincture.Color(255, 255, 255), 1, tincture.Color(255, 255, 255)),
    (tincture.Color(12, 90, 50, 30), 0.5, tincture.Color(24, 180, 100, 60)),
    (tincture.Color(255, 255, 255, 255), 255, tincture.Color(1, 1, 1, 1)),
    (tincture.Color(255, 255, 255, 255), 9e20, tincture.Color(0, 0, 0, 0)),
    (tincture.Color(255, 255, 255, 255), 1e-10, tincture.Color(255, 255, 255, 255)),
    (tincture.Color(255, 20, 40, 1), 0, ZeroDivisionError),
])
def test_color_div(color1, factor, expected):
    if not isinstance(expected, tincture.Color):
        try:
            color1 / factor
        except expected:
            pass
        except:
            assert f"An error was caught but wasn't {expected}"
        return
    assert color1 / factor == expected
    if isinstance(factor, int):
        assert color1 // factor == expected

@pytest.mark.parametrize("color1,color2,expected", [
    (tincture.Color(10, 21, 32), tincture.Color(2, 5, 3, 1), tincture.Color(20, 105, 96)),
    (tincture.Color(53, 0, 255), tincture.Color(4, 255, 0, 0), tincture.Color(212, 0, 0, 0)),
    (tincture.Color(0, 0, 0), tincture.Color(0, 0, 0), tincture.Color(0, 0, 0)),
    (tincture.Color(1, 1, 1), tincture.Color(2, 5, 3, 1), tincture.Color(2, 5, 3)),
    (tincture.Color(255, 255, 255), tincture.Color(255, 255, 255), tincture.Color(255, 255, 255)),
    (tincture.Color(0, 0, 0), tincture.Color(0, 0, 0), tincture.Color(0, 0, 0)),
])
def test_color_tensor(color1, color2, expected):
    assert color1.tensor(color2, True) == expected

@pytest.mark.parametrize("color1,factor,expected", [
    (tincture.Color(16, 64, 9), 2, tincture.Color(4, 8, 3, 15)),
    (tincture.Color(255, 255, 255), 1, ValueError),
    (tincture.Color(12, 52, 92), 0, ValueError),
    (tincture.Color(255, 255, 255), 255, tincture.Color(1, 1, 1, 1)),
    (tincture.Color(255, 255, 255), 255, tincture.Color(1, 1, 1, 1)),
])
def test_color_sqrt(color1, factor, expected):
    if not isinstance(expected, tincture.Color):
        try:
            color1.base_sqrt(factor, True)
        except expected:
            pass
        except:
            assert f"An error was caught but wasn't {expected}"
        return
    assert color1.base_sqrt(factor, True) == expected


@pytest.mark.parametrize("color1,color2,expected", [
    (tincture.Color(16, 64, 9), tincture.Color(16, 64, 9), tincture.Color(16, 64, 9)),
    (tincture.Color(2, 64, 9), tincture.Color(16, 64, 3), tincture.Color(16, 64, 9)),
    (tincture.Color(221, 210, 220, 90), tincture.Color(5, 1, 2), tincture.Color(221, 210, 220)),
    (tincture.Color(3, 3, 3, 95), tincture.Color(3, 3, 3, 90), tincture.Color(3, 3, 3, 95)),
    (tincture.Color(0, 0, 0, 0), tincture.Color(255, 255, 255), tincture.Color(255, 255, 255)),
    (tincture.Color(255, 255, 255), tincture.Color(0, 0, 0, 0), tincture.Color(255, 255, 255)),
])
def test_color_max(color1, color2, expected):
    assert color1.max(color2, True) == expected

@pytest.mark.parametrize("color1,color2,expected", [
    (tincture.Color(16, 64, 9), tincture.Color(16, 64, 9), tincture.Color(16, 64, 9)),
    (tincture.Color(2, 64, 9), tincture.Color(16, 64, 3), tincture.Color(2, 64, 3)),
    (tincture.Color(221, 210, 220, 90), tincture.Color(5, 1, 2), tincture.Color(5, 1, 2, 90)),
    (tincture.Color(3, 3, 3, 95), tincture.Color(3, 3, 3, 90), tincture.Color(3, 3, 3, 90)),
    (tincture.Color(0, 0, 0, 0), tincture.Color(255, 255, 255), tincture.Color(0, 0, 0, 0)),
    (tincture.Color(255, 255, 255), tincture.Color(0, 0, 0, 0), tincture.Color(0, 0, 0, 0)),
])
def test_color_min(color1, color2, expected):
    assert color1.min(color2, True) == expected

@pytest.mark.parametrize("color1,expected", [
    (tincture.Color(16, 64, 9), tincture.Color(239, 191, 246, 0)),
    (tincture.Color(0, 0, 0, 0), tincture.Color(255, 255, 255, 255)),
    (tincture.Color(255, 255, 255, 255), tincture.Color(0, 0, 0, 0)),
    (tincture.Color(0, 100, 255, 50), tincture.Color(255, 155, 0, 205)),
    (tincture.Color(255, 155, 0, 205), tincture.Color(0, 100, 255, 50)),
])
def test_color_inverse(color1, expected):
    assert -color1 == color1.inverse(True) == expected

@pytest.mark.parametrize("color1,expected", [
    (tincture.Color(140, 92, 210), tincture.Color(120, 120, 120)),
    (tincture.Color(80, 129, 59, 65), tincture.Color(106, 106, 106, 65)),
    (tincture.Color(94, 67, 8, 40), tincture.Color(68, 68, 68, 40)),
    (tincture.Color(0, 0, 0, 0), tincture.Color(0, 0, 0, 0)),
    (tincture.Color(255, 255, 255), tincture.Color(255, 255, 255)),
])
def test_color_grayscale(color1, expected):
    assert color1.grayscale() == expected

@pytest.mark.parametrize("color1,factor,expected", [
    (tincture.Color(140, 92, 210), 1, tincture.Color(255, 184, 255)),
    (tincture.Color(140, 92, 210, 20), 0, tincture.Color(140, 92, 210, 20)),
    (tincture.Color(10, 5, 20, 12), 2, tincture.Color(30, 15, 60, 12)),
    (tincture.Color(10, 5, 20, 12), -1, tincture.Color(5, 2, 10, 12)),
    (tincture.Color(0, 0, 0), 1e20, tincture.Color(0, 0, 0)),
    (tincture.Color(255, 255, 255), 1e20, tincture.Color(255, 255, 255)),
    (tincture.Color(0, 0, 0), -1e20, tincture.Color(0, 0, 0)),
    (tincture.Color(255, 255, 255), -1e20, tincture.Color(0, 0, 0)),
])
def test_color_brightness(color1, factor, expected):
    assert color1.brightness(factor) == expected

@pytest.mark.parametrize("color,factor,expected", [
    (tincture.Color(66, 135, 245), 40, tincture.Color(116, 66, 245)),
    (tincture.Color(50, 168, 82), -20, tincture.Color(57, 168, 49)),
    (tincture.Color(235, 64, 52, 10), 360, tincture.Color(235, 64, 52, 10)),
    (tincture.Color(121, 92, 3, 10), -360, tincture.Color(121, 92, 3, 10)),
    (tincture.Color(121, 92, 3, 10), -360000000, tincture.Color(121, 92, 3, 10)),
    (tincture.Color(121, 92, 3, 10), 360000000, tincture.Color(121, 92, 3, 10)),
    (tincture.Color(31, 81, 65, 0), 0, tincture.Color(31, 81, 65, 0)),
    (tincture.Color(138, 22, 109), 710, tincture.Color(136, 22, 127)),
])
def test_color_tint(color, factor, expected):
    assert color.tint(factor).approx_equal(expected, 2, True)

@pytest.mark.parametrize("color1,start,end,expected", [
    (tincture.Color(66, 135, 245), [None, None, None, None], [None, None, None, None], tincture.Color(66, 135, 245)),
    (tincture.Color(66, 135, 245), [None, None, None, 254], [None, None, None, 255], tincture.Color(66, 135, 245)),
    (tincture.Color(66, 135, 245), [254, 254, 254, 254], [255, 255, None, 255], ValueError),
    (tincture.Color(66, 135, 245), [None, None, None, None], [0, None, None, None], ValueError),
    (tincture.Color(66, 135, 245), [None, None, None, 255], [0, None, None, None], IndexError),
])
def test_color_rand(color1, start, end, expected):
    if not isinstance(expected, tincture.Color):
        try:
            color1.randomise(start, end)
        except expected:
            pass
        except:
            assert f"An error was caught but wasn't {expected}"
        return
    assert color1.randomise(start, end).approx_equal(expected, 1)

@pytest.mark.parametrize("color,expected", [
    (tincture.Color(2, 2, 2), True),
    (tincture.Color(0, 0, 0), True),
    (tincture.Color(0, 0, 0, 0), False),
    (tincture.Color(255, 255, 255), True),
    (tincture.Color(0, 48, 92), True),
])
def test_color_bool(color, expected):
    assert bool(color) == color.__nonzero__() == expected

@pytest.mark.parametrize("color1,color2,expected", [
    (tincture.Color(2, 2, 2), tincture.Color(4, 3, 2), False),
    (tincture.Color(2, 2, 2, 0), tincture.Color(2, 2, 2), False),
    (tincture.Color(1, 2, 3), tincture.Color(1, 2, 3), True),
    (tincture.Color(255, 255, 255), tincture.Color(255, 255, 255), True),
    (tincture.Color(0, 0, 0), tincture.Color(0, 0, 0), True),
])
def test_color_equality(color1, color2, expected):
    assert (color1 == color2) == expected
    assert (color1 != color2) != expected

@pytest.mark.parametrize("color1,color2,t,expected", [
    (tincture.Color(2, 2, 2), tincture.Color(4, 3, 2), 1.0, tincture.Color(4, 3, 2)),
    (tincture.Color(20, 52, 86), tincture.Color(20, 52, 86), 0.0, tincture.Color(20, 52, 86)),
    (tincture.Color(20, 52, 86), tincture.Color(20, 52, 86), 0.5, tincture.Color(20, 52, 86)),
    (tincture.Color(20, 52, 86), tincture.Color(20, 52, 86), 1.0, tincture.Color(20, 52, 86)),
    (tincture.Color(121, 211, 32), tincture.Color(64, 92, 41), 0.5, tincture.Color(92, 151, 36)),
    (tincture.RED, tincture.BLUE, 0.4, tincture.Color(153, 0, 102)),
    (tincture.GREEN, tincture.PURPLE, 0.6, tincture.Color(75, 102, 153)),
])
def test_color_mlerp(color1, color2, t, expected):
    result = tincture.Color.mlerp(color1, color2, t)
    color_cloned = color1.copy()
    color_cloned.mlerp_inplace(color2, t)
    color_cloned2 = color2.copy()
    color_cloned2.mlerp_inplace(color1, 1.0 - t)
    assert result.approx_equal(color_cloned, 1)
    assert color_cloned2.approx_equal(tincture.Color.mlerp(color2, color1, 1.0 - t), 1)
    assert expected.approx_equal(result, 1)

@pytest.mark.parametrize("color1,color2,t,expected", [
    (tincture.Color(2, 2, 2), tincture.Color(4, 3, 2), 1.0, tincture.Color(4, 3, 2)),
    (tincture.Color(20, 52, 86), tincture.Color(20, 52, 86), 0.0, tincture.Color(20, 52, 86)),
    (tincture.Color(20, 52, 86), tincture.Color(20, 52, 86), 0.5, tincture.Color(20, 52, 86)),
    (tincture.Color(20, 52, 86), tincture.Color(20, 52, 86), 1.0, tincture.Color(20, 52, 86)),
    (tincture.Color(121, 211, 32), tincture.Color(64, 92, 41), 0.5, tincture.Color(93, 147, 42)),
    (tincture.RED, tincture.BLUE, 0.4, tincture.Color(0, 125, 35)),
    (tincture.GREEN, tincture.PURPLE, 0.6, tincture.Color(75, 102, 153)),
])
def test_color_clerp(color1, color2, t, expected):
    result = tincture.Color.clerp(color1, color2, t)
    color_cloned = color1.copy()
    color_cloned.clerp_inplace(color2, t)
    color_cloned2 = color2.copy()
    color_cloned2.clerp_inplace(color1, 1.0 - t)
    assert result.approx_equal(color_cloned, 1)
    assert color_cloned2.approx_equal(tincture.Color.clerp(color2, color1, 1.0 - t), 1)
    assert result.approx_equal(expected, 1)