from gpy import PortType
import numpy as np


def config():
    inputs = {"a": [PortType.Real]}
    outputs = {"out": [PortType.Real]}
    return (inputs, outputs)


def compute(input):
    a = np.asarray(input["a"])
    out = 2 * np.sinc(a)
    return out
