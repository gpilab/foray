import gpipy
import rng


def start():
    """create an instance of Gadget, configure it and return to Rust"""
    g = gpipy.Gadget()
    g.push(1)
    g.push(2)
    g.push(3)
    g.push(9)
    g.prop = rng.get_random_number()
    return g
