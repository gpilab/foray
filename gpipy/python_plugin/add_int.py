from typing import NamedTuple
import gpipy


def init():
    return 1, 2, 3


class In(NamedTuple):
    a: int
    b: int


def config():
    inputs = ["Integer", "Integer"]
    outputs = ["Integer"]
    return (inputs, outputs)


def compute(inputs: In):
    a, b = inputs
    return a + b
