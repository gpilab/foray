import pytest


@pytest.fixture
def gadget():
    import gpipy as pa

    g = pa.GpiNode(0, 0, 0, {})
    print(
        pa.get_node_inputs(
            "/home/john/projects/gpi_v2/gpi_lib/python_plugin/add_int.py"
        )
    )
    return g
