from gpi import port


def config():
    class out:
        inputs = {}
        outputs = {"a": {"a1": port.Integer, "a2": port.Real}}
        parameters = {}

    return out


def compute(input, parameters):
    out = {"a1": 1, "a2": 1.1}
    return {"a": out}
