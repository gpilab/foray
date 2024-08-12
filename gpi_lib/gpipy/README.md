This project generates the python module used to interact with GPI from python.

To install:
```bash
doesn't actually work yet! isn't published, use development instructions below
pip install gpipy
```


## Testing
test can be run using [nox](https://nox.thea.codes/en/stable/)
```bash
nox
```

## Development
It is strongly recommended to create a new virtual environment for GPI development.
This can be done many ways, here's a simple method:
```bash
cd gpi_lib
python -m venv .venv
source .venv/bin/activate
```


Install all dependencies
```bash
cd gpipy
pip install .[dev]
```


After making your changes, you can install the local package to your python env using:
```bash
maturin develop
```
