use super::PortData;
use crate::{app::PortDataContainer, nodes::NodeError, OrderMap};
use log::debug;
use ndarray::Array1;
use std::ops::Deref;

#[allow(clippy::type_complexity)]
pub fn binary_operation(
    //node: NodeData,
    inputs: OrderMap<String, PortDataContainer>,
    f: Box<dyn Fn(&Array1<f64>, &Array1<f64>) -> Array1<f64>>,
) -> Result<OrderMap<String, PortData>, NodeError> {
    debug!("about to lock port a");
    let a = inputs
        .get("a")
        .ok_or(NodeError::input_error("a"))?
        .lock()
        .unwrap();
    debug!("about to lock port b");
    let b = inputs
        .get("b")
        .ok_or(NodeError::input_error("b"))?
        .lock()
        .unwrap();

    let out = match (&(*a), &(*b)) {
        (PortData::Real(a), PortData::Real(b)) => f(a, b),
        _ => panic!("bad inputs!"),
    };
    debug!("finished  locking binary ports");

    Ok([("out".into(), PortData::Real(out))].into())
}
#[allow(clippy::type_complexity)]
pub fn unary_operation(
    //node: NodeData,
    inputs: OrderMap<String, PortDataContainer>,
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
