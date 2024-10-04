from gpy import PortType
import numpy as np


def config():
    inputs = {"a": PortType.Real, "b": [PortType.Real]}
    outputs = {"out": [PortType.Real]}
    return (inputs, outputs)


def compute(input):
    a = input["a"]
    b = np.asarray(input["b"])
    out = a * b
    return out
