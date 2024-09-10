from typing import NamedTuple
from gpipy import PortType


def init():
    return 1, 2, 3


class In(NamedTuple):
    a: int
    b: int


def config():
    inputs = {"a": PortType.Integer, "b": PortType.Integer}
    outputs = {"out": PortType.Integer}
    return (inputs, outputs)


def compute(inputs):
    return inputs["a"] + inputs["b"]
