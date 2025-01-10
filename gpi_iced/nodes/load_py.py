import numpy as np


def config():
    inputs = {"a": [PortType.Real], "b": [PortType.Real]}
    outputs = {"out": [PortType.Real]}
    return (inputs, outputs)


def compute(input):
    # a = np.asarray(input["a"])
    # b = np.asarray(input["b"])
    # print("add int:")
    # print(a)
    # print(b)
    # out = np.zeros((64, 64))
    r = 32
    A = np.arange(-r, r + 1) ** 2
    dists = np.sqrt(A[:, None] + A)
    circle = np.abs(dists - r)
    circle = circle / circle.max()
    return circle

    # print(out)
    # return out
