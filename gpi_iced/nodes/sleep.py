import time


def config():
    class out:
        inputs = {"in": "Real"}
        outputs = {"out": "Real"}

    return out


def compute(input):
    time.sleep(2)
    output = input["in"]
    return output
