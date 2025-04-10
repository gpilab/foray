import numpy as np
from gpi import port, ui
from Spin import N


def config():
    class out:
        inputs = {}
        outputs = {"out": port.ArrayReal}  # //3d
        parameters = {
            "x": ui.Slider,
            "y": ui.Slider,
        }

    return out


def compute(input, parameters):
    b = np.tile(
        np.array([parameters["x"], parameters["y"], 0], dtype=np.float64),
        (N, N, 1, 1),
    )
    return {"out": b}
