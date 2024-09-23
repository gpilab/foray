from gpy import PortType
import numpy as np


def config():
    inputs = {"a": PortType.Real, "b": [PortType.Real]}
    outputs = {"out": [PortType.Real]}
    return (inputs, outputs)


def compute(input):
    a = np.asarray(input["a"])
    b = input["b"]
    out = a * b
    return out
