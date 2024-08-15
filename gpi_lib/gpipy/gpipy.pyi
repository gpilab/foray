"""Interface to GPI"""

import numpy as np
from typing import List, Dict

### This file contains all the type/documentation information for the gpipy module
### It is manually created, because `pyo3` can't automatically generate it yet

type GPI_Int = int
type GPI_String = str
Data_Type = int | str | np.ndarray

class GpiNode:
    """doc string for GpiNode!"""

    # def __init__(self, a: int, b: int, out: int, config: Dict) -> None: ...
    inputs: List[Data_Type]
    out: Data_Type
    Config: Dict

class Gadget:
    """doc string for gadget!"""

    prop: int
    def __init__(self) -> None: ...
    def push(self, v: int) -> None: ...
