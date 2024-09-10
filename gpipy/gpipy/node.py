import os
import sys
import importlib

GPI_STRING = "String"
GPI_VEC1 = "Vec1"


def get_local_nodes(local_node_dir: str):
    files = [
        file_path
        for file_path in os.listdir(local_node_dir)
        if (os.path.isfile(local_node_dir + file_path) and file_path[-3:] == ".py")
    ]
    nodes = []
    abs_node_path = os.path.abspath(local_node_dir)
    sys.path.append(abs_node_path)
    for file in files:
        node_name = file.split(".")[0]  # strip off '.py'
        # mod = importlib.import_module(node_name)

        # config, inputs, outputs = mod.init()

        nodes.append((node_name, file))

    return nodes


def validateNode(): ...


def get_all_nodes(): ...


def get_node_inputs(abs_node_path: str):
    dir = abs_node_path.split("/")[:-1]
    sys.path.append("/".join(dir))
    file = abs_node_path.split("/")[-1]
    mod_name = file.split(".")[0]
    inputs, _ = importlib.import_module(mod_name).config()
    return inputs


def get_node_outputs(abs_node_path: str):
    dir = abs_node_path.split("/")[:-1]
    sys.path.append("/".join(dir))
    file = abs_node_path.split("/")[-1]
    mod_name = file.split(".")[0]
    _, outputs = importlib.import_module(mod_name).config()
    return outputs


if __name__ == "__main__":
    get_local_nodes("../nodes/")
