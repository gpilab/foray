from gpy import PortType


def config():
    inputs = {"a": PortType.Integer, "b": PortType.Integer}
    outputs = {"out": PortType.Integer}
    return (inputs, outputs)


def compute(inputs):
    return inputs["a"] + inputs["b"]
