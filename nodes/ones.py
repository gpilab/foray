import numpy as np
from gpy import PortType


def config():
    inputs = {}
    outputs = {"out": [PortType.Real]}
    return (inputs, outputs)


def compute(input):
    return np.ones(10)
