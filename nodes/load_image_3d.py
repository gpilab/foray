import numpy as np
from gpy import PortType
import time


def config():
    inputs = {}
    outputs = {"out": PortType.Image}
    return (inputs, outputs)


def compute(input):
    start = time.time()
    row = np.asarray(range(0, 256), dtype=np.uint8)
    img = np.tile(row, (256, 64))
    print(img)
    print("large image gen (ms):", (time.time() - start) * 1000)
    return img  # np.load("../nodes/b0.npy")
