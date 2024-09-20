import numpy as np
from gpy import PortType


def config():
    inputs = {"a": PortType.String}
    outputs = {"out": [PortType.Integer]}
    return (inputs, outputs)


# TODO: correct passing as an object, rather than a list
def compute(input):
    return np.load(input["a"])
