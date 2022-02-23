# SimpleUnitaryBuilder

This builds a simple python package to run quantum circuits.

##Example
```python
import numpy
import py_qip

# Construct a circuit, start with 10 qubits
b = py_qip.Circuit(qubits=10)

# Queue an X gate to qubit 0
xgate = numpy.array([[0,1],[1,0]], dtype=numpy.complex128)
b.apply_unitary([0], xgate)

# And an CX to [9,0]
cxgate = numpy.array([[1,0,0,0],[0,1,0,0],[0,0,0,1],[0,0,1,0]], dtype=numpy.complex128)
b.apply_unitary([9,0], cxgate)

# Run the circuit and get the state.
# Low index qubits are the most significant bits in the state.
state = b.run_circuit()
```

## Installation
1. Install rust on your system: https://www.rust-lang.org/learn/get-started

2. Prepare your python environment by installing `maturin`, `numpy`, `wheel`, and upgrading `pip`:
   1. `> pip install maturin numpy wheel`
   2. `> pip install --upgrade pip`
3. Clone the repository:
   1. `> git clone git@github.com:Renmusxd/SimpleUnitaryBuilder.git`
4. Run `make` in the parent directory
   1. `> make`
5. Install the resulting wheel with pip
   1. `> pip install target/wheels/*`