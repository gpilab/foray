# import the contents of the Rust library into the Python extension
from .gpirs import *
from .gpirs import __all__

# optional: include the documentation from the Rust module
from .gpirs import __doc__  # noqa: F401
