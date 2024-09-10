from gpipy import PortType


def init():
    return 1, 2, 3


def config():
    inputs = {"a": PortType.Vec}
    outputs = {"out": PortType.Vec}
    return (inputs, outputs)


def compute(inputs):
    return inputs["a"]
