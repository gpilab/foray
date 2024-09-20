import gpirs
from gpy import PortType


def config():
    inputs = {
        "my_int": PortType.Integer,
        "my_real": PortType.Real,
        "my_string": PortType.String,
        "my_array": [PortType.Integer],
        "my_2d_array": [[PortType.Integer]],
        "my_3d_array": [[[PortType.Integer]]],
        "my_struct": {
            "nested_int": PortType.Integer,
            "nested_real": PortType.Real,
            "nested_nested": {"my_double_nested_string": PortType.String},
        },
    }
    outputs = {"out": PortType.Integer}
    return (inputs, outputs)
