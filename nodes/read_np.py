import numpy as np
from typing import NamedTuple
from gpipy import PortType


def init():
    return 7, 8, 9


class In(NamedTuple):
    path: str


class Config(NamedTuple):
    inputs = (str,)
    outputs = np.ndarray


def config():
    inputs = {"a": PortType.String}
    outputs = {"out": PortType.Vec}
    return (inputs, outputs)


# TODO: correct passing as an object, rather than a list
def compute(input):
    return np.load(input["a"])
