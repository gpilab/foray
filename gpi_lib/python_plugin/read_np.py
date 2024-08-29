import numpy as np
from typing import NamedTuple


def init():
    return 7, 8, 9


class In(NamedTuple):
    path: str


class Config(NamedTuple):
    inputs = (str,)
    outputs = np.ndarray


def config():
    inputs = ["String"]
    outputs = ["Vec2"]
    return (inputs, outputs)


# TODO: correct passing as an object, rather than a list
def compute(input: In):
    return np.load(input[0])
