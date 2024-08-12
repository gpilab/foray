import gpipy
import rng


def start():
    """initialize node"""
    return gpipy.GpiNode(1, 1, 3, {})


def compute(node):
    node.out = node.a + node.b
    node.config["abc"] = str(rng.get_random_number())

    return node
