from gpy import PortType


def init():
    return 1, 2, 3


def config():
    inputs = {"a": [PortType.Integer]}
    outputs = {"out": [PortType.Integer]}
    return (inputs, outputs)


def compute(inputs):
    return inputs["a"]
