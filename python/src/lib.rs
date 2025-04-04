extern crate i3s as i3s_rs;

use i3s_rs::SceneLayer;
use i3s_rs::node::Node;
use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use std::sync::Arc;

/// Wrapper for the `SceneLayer` struct to expose it to Python.
#[pyclass]
struct SceneLayerWrapper {
    scene_layer: Arc<SceneLayer>, // Use Arc to manage shared ownership
}

#[pymethods]
impl SceneLayerWrapper {
    /// Expose the `nodes` method.
    fn nodes(&self) -> PyResult<Py<NodeArrayWrapper>> {
        Python::with_gil(|py| {
            let node_array = self.scene_layer.nodes(); // Get the NodeArray
            let wrapper = NodeArrayWrapper {
                node_array: Arc::new(node_array), // Wrap NodeArray in Arc
            };
            Py::new(py, wrapper) // Return a Python-managed object
        })
    }
}

/// Wrapper for the `NodeArray` struct to expose it to Python.
#[pyclass]
struct NodeArrayWrapper {
    node_array: Arc<i3s_rs::node::NodeArray<'static>>, // Use Arc for shared ownership
}

#[pymethods]
impl NodeArrayWrapper {
    /// Expose the `len` method.
    fn len(&self) -> usize {
        self.node_array.len()
    }

    /// Expose the `traverse` method.
    fn traverse(&self, callback: PyObject) {
        let gil = Python::acquire_gil();
        let py = gil.python();

        self.node_array.traverse(|node, level| {
            let result = callback.call1(py, (node.index, *level));
            result.is_ok()
        });
    }
}

/// Python module definition.
#[pymodule]
fn i3s(m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(from_uri, m)?)?;
    m.add_class::<SceneLayerWrapper>()?;
    m.add_class::<NodeArrayWrapper>()?;
    Ok(())
}
