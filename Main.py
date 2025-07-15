import math
from concurrent.futures import ProcessPoolExecutor, as_completed

import matplotlib.pyplot as plt
from tqdm import tqdm

import tincture
import timeit
import random

class MyColor:
    def __init__(self, r, g, b, a):
        self.r: int = r
        self.g: int = g
        self.b: int = b
        self.a: int = a
    def randomise(self, l1 = [0, 0, 0, 0], l2 = [255, 255, 255, 255]):
        return MyColor(
            random.randint(l1[0], l2[0]),
            random.randint(l1[1], l2[1]),
            random.randint(l1[2], l2[2]),
            random.randint(l1[3], l2[3])
        )

    def add(self, color: "MyColor") -> "MyColor":
        return MyColor(self.r + color.r, self.g + color.g, self.b + color.b, self.a + color.a)

    def sub(self, color: "MyColor") -> "MyColor":
        return MyColor(self.r - color.r, self.g - color.g, self.b - color.b, self.a - color.a)

    def mul(self, other: float) -> "MyColor":
        return MyColor(self.r * other, self.g * other, self.b * other, self.a * other)

    def div(self, other: float) -> "MyColor":
        return MyColor(self.r / other, self.g / other, self.b / other, self.a / other)

    def tensor(self, color: "MyColor") -> "MyColor":
        return MyColor(self.r * color.r, self.g * color.g, self.b * color.b, self.a * color.a)

    def base_sqrt(self, other: float) -> "MyColor":
        return MyColor(math.pow(self.r, 1 / other), math.pow(self.g, 1 / other), math.pow(self.b, 1 / other), math.pow(self.a, 1 / other))


    def __add__(self, color: "MyColor") -> "MyColor":
        return MyColor(self.r + color.r, self.g + color.g, self.b + color.b, self.a + color.a)

    def __pow__(self, other: float) -> "MyColor":
        return MyColor(self.r * other, self.g * other, self.b * other, self.a * other)

def tincture_test():
    tincture.WHITE.mul(2).add(tincture.WHITE).add(tincture.WHITE).add(tincture.WHITE).add(tincture.WHITE).tensor(tincture.RED).base_sqrt(10)
    result = tincture.WHITE.sub(tincture.WHITE.randomise([0, 0, 0, 0], [255, 255, 255, 255]))
    result.mul(1, True).mul(2, True).mul(0.5, True)
    result.div(1, True).div(2, True).div(0.5, True)
    result.tensor(tincture.RED).tensor(tincture.GREEN, True).tensor(result.tensor(tincture.BLUE))
    result.base_sqrt(20, True).base_sqrt(4, True)
    result.add(result, True).add(result, True).add(tincture.Color(20, 20, 20, 20), False)
    result.base_sqrt(20).base_sqrt(40).add(tincture.WHITE).sub(tincture.BLACK).div(20)

def python_test():
    WHITE = MyColor(255, 255, 255, 255)
    WHITE.mul(2).add(WHITE).add(WHITE).add(WHITE).add(WHITE).tensor(MyColor(255, 0, 0, 255)).base_sqrt(10)
    result = WHITE.sub(WHITE.randomise([0, 0, 0, 0], [255, 255, 255, 255]))
    result.mul(1).mul(2).mul(0.5)
    result.div(1).div(2).div(0.5)
    result.tensor(MyColor(255, 0, 0, 255)).tensor(MyColor(0, 255, 0, 255)).tensor(result.tensor(MyColor(0, 0, 255, 255)))
    result.base_sqrt(20).base_sqrt(4)
    result + result + result + MyColor(20, 20, 20, 0)
    result.base_sqrt(20).base_sqrt(40).add(WHITE).sub(MyColor(0, 0, 0, 255)).div(20)

def bench_tincture(i):
    timing = 0
    for _ in range(5):
        timing += timeit.timeit("tincture_test()", globals=globals(), number=i)
    return timing / 5

def bench_python(i):
    timing = 0
    for _ in range(5):
        timing += timeit.timeit("python_test()", globals=globals(), number=i)
    return timing / 5


def main():
    iters = list(range(1, 3000))
    tincture_timings = [None] * len(iters)
    python_timings   = [None] * len(iters)

    # Map futures to (which, iteration)
    future_meta = {}
    with ProcessPoolExecutor() as exe:
        for idx, i in enumerate(iters):
            f1 = exe.submit(bench_tincture, i)
            future_meta[f1] = ("tincture", idx)
            f2 = exe.submit(bench_python, i)
            future_meta[f2] = ("python",   idx)

        for fut in tqdm(as_completed(future_meta), total=len(future_meta)):
            kind, idx = future_meta[fut]
            try:
                res = fut.result()
            except Exception as e:
                # handle or re-raise; for now we just crash
                raise

            if kind == "tincture":
                tincture_timings[idx] = res
            else:
                python_timings[idx] = res

    # Plot
    plt.plot(iters, tincture_timings, label="tincture")
    plt.plot(iters, python_timings,   label="pure Python")
    plt.xlabel("Number of iterations")
    plt.ylabel("Elapsed time (s)")
    plt.legend()
    plt.show()

if __name__ == "__main__":
    main()
