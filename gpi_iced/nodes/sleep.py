import random
import time


def config():
    class out:
        inputs = {}
        outputs = {"out": "Real"}

    return out


def compute(input):
    print("start sleep")
    time.sleep(5)
    x = random.random()
    print("end sleep")
    print(x)

    return [x]
