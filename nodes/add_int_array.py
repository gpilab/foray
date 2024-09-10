from typing import NamedTuple
import numpy as np
from gpipy import PortType


def init():
    return 4, 5, 6


class In(NamedTuple):
    a: np.ndarray
    b: np.ndarray


def config():
    inputs = {"a": PortType.Vec, "b": PortType.Vec}
    outputs = {"out": PortType.Vec}
    return (inputs, outputs)


def compute(input):
    a = np.asarray(input["a"])
    b = np.asarray(input["b"])
    print("add int:")
    print(a)
    print(b)
    out = a + b
    print(out)
    return out
