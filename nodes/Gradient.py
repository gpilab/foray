# from gpy import PortType
# import numpy as np
import numpy as np
from gpi import port, ui
from Spin import N


def config():
    class out:
        inputs = {}
        outputs = {"out": port.ArrayReal}
        parameters = {
            "X": ui.Slider,
            "Y": ui.Slider,
            "Z": ui.Slider,
        }

    return out


def compute(_, parameters):
    gx = float(parameters["X"])
    gy = float(parameters["Y"])
    gz = float(parameters["Z"])
    x = np.linspace(0, gx, N)
    y = np.linspace(0, gy, N)
    X, Y, Z = np.meshgrid(x, y, np.array([gz]))

    G = np.zeros_like(X)

    G += X
    G += Y
    G += Z

    return {"out": G}
