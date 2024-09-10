from gpipy import PortType
import numpy as np


def config():
    inputs = {"a": PortType.Vec}
    outputs = {"out": PortType.Vec}
    return (inputs, outputs)


def compute(input):
    a = np.asarray(input["a"])
    return np.absolute(np.fft.fft(a))
