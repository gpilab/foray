# from gpy import PortType
# import numpy as np
import numpy as np


def config():
    class out:
        inputs = {"a": "Complex2d", "b": "Complex2d"}
        outputs = {"out": "Complex2d"}

    return out


def compute(input):
    a = input["a"]
    b = input["b"]
    out = np.multiply(a, b)
    return out
