use super::PortData;
use crate::OrderMap;
use ndarray::Array1;
use std::ops::Deref;

#[allow(clippy::type_complexity)]
pub fn binary_operation(
    //node: NodeData,
    inputs: OrderMap<String, &std::cell::RefCell<PortData>>,
    f: Box<dyn Fn(&Array1<f64>, &Array1<f64>) -> Array1<f64>>,
) -> OrderMap<String, PortData> {
    let out = match (inputs["a"].borrow().deref(), inputs["b"].borrow().deref()) {
        (PortData::Real(a), PortData::Real(b)) => f(a, b),
        _ => panic!("bad inputs!"),
    };

    [("out".into(), PortData::Real(out))].into()
}
