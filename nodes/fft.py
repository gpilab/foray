from gpy import PortType
import numpy as np


def config():
    inputs = {"a": [PortType.Integer]}
    outputs = {"out": [PortType.Integer]}
    return (inputs, outputs)


def compute(input):
    a = np.asarray(input["a"])
    print("hello world again!")
    return np.absolute(np.fft.fft(a)) * 4
