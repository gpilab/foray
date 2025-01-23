use super::PortData;
use crate::{nodes::NodeError, OrderMap};
use ndarray::Array1;
use std::{ops::Deref, sync::Mutex};

#[allow(clippy::type_complexity)]
pub fn binary_operation(
    //node: NodeData,
    inputs: OrderMap<String, &Mutex<PortData>>,
    f: Box<dyn Fn(&Array1<f64>, &Array1<f64>) -> Array1<f64>>,
) -> Result<OrderMap<String, PortData>, NodeError> {
    let out = match (
        inputs
            .get("a")
            .ok_or(NodeError::input_error("a"))?
            .lock()
            .unwrap()
            .deref(),
        inputs
            .get("b")
            .ok_or(NodeError::input_error("b"))?
            .lock()
            .unwrap()
            .deref(),
    ) {
        (PortData::Real(a), PortData::Real(b)) => f(a, b),
        _ => panic!("bad inputs!"),
    };

    Ok([("out".into(), PortData::Real(out))].into())
}
#[allow(clippy::type_complexity)]
pub fn unary_operation(
    //node: NodeData,
    inputs: OrderMap<String, &Mutex<PortData>>,
    f: Box<dyn Fn(&Array1<f64>) -> Array1<f64>>,
) -> Result<OrderMap<String, PortData>, NodeError> {
    let out = match inputs
        .get("a")
        .ok_or(NodeError::input_error("a"))?
        .lock()
        .unwrap()
        .deref()
    {
        PortData::Real(a) => f(a),
        _ => panic!("bad inputs!"),
    };

    Ok([("out".into(), PortData::Real(out))].into())
}
