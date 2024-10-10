import numpy as np
from gpy import PortType


def config():
    inputs = {}
    outputs = {"out": PortType.Image}
    return (inputs, outputs)


def compute(input):
    row = np.asarray(range(0, 256), dtype=np.uint8)
    img = np.tile(row, (256, 1))
    print(img)
    #return img  # np.load("../nodes/b0.npy")
    return np.load("../nodes/b0.npy")
