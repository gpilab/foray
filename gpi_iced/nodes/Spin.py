import numpy as np
from gpi import port, ui

N = 12


def config():
    class out:
        inputs = {}
        outputs = {"out": port.ArrayReal}
        parameters = {
            "x": ui.Slider,
            "y": ui.Slider,
            "z": ui.Slider,
        }

    return out


def compute(_, parameters):
    b = np.tile(
        np.array([parameters["x"], parameters["y"], parameters["z"]], dtype=np.float64),
        (N, N, 1, 1),
    )
    return {"out": b}
