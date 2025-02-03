import numpy as np


def config():
    class out:
        inputs = {"radius": "Real"}
        outputs = {"out": "Real2d"}

    return out


def compute(input):
    x = np.linspace(0, 10, 2048)
    y = np.linspace(0, 10, 2048)

    radius = input["radius"]

    dist = (x[:, None] - 5) ** 2 + (y - 5) ** 2
    out = np.zeros_like(dist)
    out[dist < radius] = 1.0

    return out
