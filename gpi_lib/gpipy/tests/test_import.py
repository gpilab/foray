import gpipy


def test_import():
    import gpipy  # noqa: F401

    # g = gpipy.GpiNode(0, 0, 0, {})
    print(
        gpipy.get_node_inputs(
            "/home/john/projects/gpi_v2/gpi_lib/python_plugin/add_int.py"
            # "add_int"
        )
    )
