from gpy import PortType
import numpy as np


def config():
    inputs = {"a": [PortType.Integer], "b": [PortType.Integer]}
    outputs = {"out": [PortType.Integer]}
    return (inputs, outputs)


def compute(input):
    a = np.asarray(input["a"])
    b = np.asarray(input["b"])
    print("add int:")
    print(a)
    print(b)
    out = a + b
    print(out)
    return out
