import numpy as np
from typing import NamedTuple


class In(NamedTuple):
    path: str


# TODO: correct passing as an object, rather than a list
def compute(input: In):
    return np.load(input[0])
