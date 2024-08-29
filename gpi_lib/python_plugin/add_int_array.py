from typing import NamedTuple
import numpy as np


def init():
    return 4, 5, 6


class In(NamedTuple):
    a: np.ndarray
    b: np.ndarray


class Config(NamedTuple):
    inputs = (np.ndarray, np.ndarray)
    outputs = np.ndarray


def config():
    inputs = ["Integer", "Integer"]
    outputs = ["Integer"]
    return (inputs, outputs)


def compute(input: In):
    a = np.asarray(input[0])
    b = np.asarray(input[1])
    return a + b
