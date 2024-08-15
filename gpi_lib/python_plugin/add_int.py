from typing import NamedTuple


class In(NamedTuple):
    a: int
    b: int


def compute(inputs: In):
    a, b = inputs
    return a + b
