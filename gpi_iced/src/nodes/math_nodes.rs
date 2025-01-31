use crate::{gui_node::PortDataReference, nodes::NodeError, OrderMap};
use ndarray::Array1;

use super::port::PortData;

#[allow(clippy::type_complexity)]
pub fn binary_operation(
    inputs: OrderMap<String, PortDataReference>,
    f: Box<dyn Fn(&Array1<f64>, &Array1<f64>) -> Array1<f64>>,
) -> Result<OrderMap<String, PortData>, NodeError> {
    let a = inputs.get("a").ok_or(NodeError::input_error("a"))?;
    let b = inputs.get("b").ok_or(NodeError::input_error("b"))?;

    let out = match (&**a, &**b) {
        (PortData::Real(a), PortData::Real(b)) => f(a, b),
        _ => panic!("bad inputs!"),
    };

    Ok([("out".into(), PortData::Real(out))].into())
}
#[allow(clippy::type_complexity)]
pub fn unary_operation(
    inputs: OrderMap<String, PortDataReference>,
    f: Box<dyn Fn(&Array1<f64>) -> Array1<f64>>,
) -> Result<OrderMap<String, PortData>, NodeError> {
    let out = match &**inputs.get("a").ok_or(NodeError::input_error("a"))? {
        PortData::Real(a) => f(a),
        _ => panic!("bad inputs!"),
    };

    Ok([("out".into(), PortData::Real(out))].into())
}
