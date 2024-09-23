import numpy as np
from gpy import PortType


def config():
    inputs = {}
    outputs = {"out": [PortType.Real]}
    return (inputs, outputs)


def compute(input):
    return np.linspace(-10, 10, 40)
