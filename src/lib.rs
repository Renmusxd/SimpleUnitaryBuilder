use numpy::{c64, PyArray1, PyReadonlyArray2};
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use qip::prelude::*;

/// Unlike the Lattice class this maintains a set of graphs with internal state.
#[pyclass]
pub struct Circuit {
    qubits: Vec<Option<<LocalBuilder<f64> as CircuitBuilder>::Register>>,
    builder: LocalBuilder<f64>,
}

#[pymethods]
impl Circuit {
    /// Construct a new instance.
    #[new]
    fn new(qubits: Option<usize>) -> Self {
        let mut builder = LocalBuilder::default();
        let nqubits = qubits.unwrap_or(0);
        let qubits = (0..nqubits).map(|_| Some(builder.qubit())).collect();
        Self {
            qubits,
            builder,
        }

    }

    fn apply_unitary(
        &mut self,
        indices: Vec<usize>,
        matrix: PyReadonlyArray2<c64>,
    ) -> PyResult<()> {
        let max_index = indices.iter().cloned().max().unwrap_or(0);

        let shape = matrix.shape();
        if shape[0] != matrix.shape()[1] {
            return Err(PyErr::new::<PyValueError, _>(format!(
                "Only square matrices allowed: found {}x{}",
                shape[0], shape[1]
            )));
        }
        if shape[0] as u32 != 2_u32.pow(indices.len() as u32) {
            return Err(PyErr::new::<PyValueError, _>(format!(
                "Matrix must be 2^{}: found {}x{}",
                indices.len(),
                shape[0],
                shape[1]
            )));
        }

        if !indices.is_empty() {
            let qubits_needed = max_index + 1;
            let qubits_made = self.qubits.len();
            for _ in qubits_made..qubits_needed {
                let q = self.builder.qubit();
                self.qubits.push(Some(q));
            }
            let qs = indices.iter().flat_map(|i| self.qubits[*i].take());
            let r = self
                .builder
                .merge_registers(qs)
                .expect("Unexpected missing qubits");
            let data = matrix.as_slice()?.to_vec();
            let r = self
                .builder
                .apply_vec_matrix(r, data)
                .map_err(|e| PyErr::new::<PyValueError, _>(e.msg))?;
            let rs = self.builder.split_all_register(r);
            indices
                .into_iter()
                .zip(rs)
                .for_each(|(i, r)| self.qubits[i] = Some(r));
        }
        Ok(())
    }

    fn run_circuit(
        &mut self,
        py: Python,
        initial_state: Option<Vec<(usize, bool)>>,
    ) -> PyResult<Py<PyArray1<c64>>> {
        let (state, _) = match initial_state {
            None => self.builder.calculate_state(),
            Some(state) => self
                .builder
                .calculate_state_with_init(state.into_iter().map(|(index, state)| {
                    let r = self.qubits[index].as_ref().unwrap();
                    (r, if state { 1 } else { 0 })
                })),
        };
        Ok(PyArray1::from_iter(py, state).to_owned())
    }
}

#[pymodule]
fn py_qip(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_class::<Circuit>()?;
    Ok(())
}
