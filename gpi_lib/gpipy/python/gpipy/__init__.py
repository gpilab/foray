# import the contents of the Rust library into the Python extension
from .gpipy import *
from .gpipy import __all__

# optional: include the documentation from the Rust module
from .gpipy import __doc__  # noqa: F401

from .node import get_local_nodes
