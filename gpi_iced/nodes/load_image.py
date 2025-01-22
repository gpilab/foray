import numpy as np
from PIL import Image


def config():
    class out:
        inputs = {}
        outputs = {"out": "Real2d"}

    return out


def compute(input):
    img = Image.open("nodes/data/slogan.png")

    # just take blue channel for simplicity
    np_img = np.array(img)[:, :, 0] / 255.0

    return np_img
