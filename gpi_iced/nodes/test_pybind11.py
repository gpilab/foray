# from gpy import PortType
import numpy as np
import python_example

python_example.add(1, 2)


def config():
    class out:
        inputs = {"a": "Real", "b": "Real"}
        outputs = {"out": "Real"}

    return out


def compute(input):
    a = input["a"]
    b = input["b"]
    c = python_example.add(a, b)
    out = np.array([c, c], dtype=np.float64)

    print(out)

    return out
