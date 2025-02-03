import time


def config():
    class out:
        inputs = {"in": "Real"}
        outputs = {"out": "Real"}

    return out


def compute(input):
    output = input["in"]

    time.sleep(2)
    print(output)
    return output
