# from gpy import PortType
# import numpy as np


def config():
    class out:
        inputs = {"a": "Real", "b": "Real"}
        outputs = {"out": "Real"}

    return out


def compute(input):
    # a = np.asarray(input["a"])
    # b = np.asarray(input["b"])
    a = input["a"]
    b = input["b"]
    # print("add int:")
    # print(a)
    # print(b)
    out = a + b
    # print(out)
    return out
