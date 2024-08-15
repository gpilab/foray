import pytest


@pytest.fixture
def gadget():
    import gpipy as pa

    g = pa.GpiNode(0, 0, 0, {})
    return g
