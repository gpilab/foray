# from gpy import PortType
import numpy as np


# def config():
#     inputs = {"a": [PortType.Real], "b": [PortType.Real]}
#     outputs = {"out": [PortType.Real]}
#     return (inputs, outputs)


def compute(input):
    a = np.asarray(input["a"])
    b = np.asarray(input["b"])
    # print("add int:")
    # print(a)
    # print(b)
    out = a + b
    # print(out)
    return out
