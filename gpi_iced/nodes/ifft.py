import numpy as np


def config():
    class out:
        inputs = {"a": "Complex2d"}
        outputs = {"out": "Complex2d"}

    return out


def compute(input):
    a = input["a"]
    out = np.fft.ifft2(a)
    return out
