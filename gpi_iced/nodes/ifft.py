import numpy as np
from gpi import port


def config():
    class out:
        inputs = {"a": port.ArrayComplex}
        outputs = {"out": port.ArrayComplex}
        parameters = {}

    return out


def compute(input, _):
    a = input["a"]
    out = np.fft.ifft2(a)
    return {"out": out}
