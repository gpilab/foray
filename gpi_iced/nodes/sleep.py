import random
import time
import numpy as np


def config():
    class out:
        inputs = {"in": "Real"}
        outputs = {"out": "Real"}

    return out


def compute(input):
    print("start sleep")
    time.sleep(2)
    x = random.random()
    print("end sleep")
    print(x)

    x = np.array([x])
    print(x)
    return x
