from typing import NamedTuple
import numpy as np


class In(NamedTuple):
    a: np.ndarray
    b: np.ndarray


def compute(input: In):
    a = np.asarray(input[0])
    b = np.asarray(input[1])
    return a + b
