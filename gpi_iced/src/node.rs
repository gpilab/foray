use ordermap::OrderMap;
use smol_str::SmolStr;
use std::cell::RefCell;

use crate::math_nodes::PortData;

pub const NODE_WIDTH: f32 = 100.;
pub const NODE_HEIGHT: f32 = 60.;
pub const PORT_RADIUS: f32 = 7.5;
pub const NODE_RADIUS: f32 = 5.0;

pub fn format_node_output(
    data: &OrderMap<SmolStr, Option<&RefCell<PortData>>>,
) -> Vec<(String, String)> {
    data.into_iter()
        .map(|(port_name, d)| {
            (
                port_name.to_string(),
                d.map(|d| format!("{}", d.borrow()))
                    .unwrap_or("n/a".to_string()),
            )
        })
        .collect()
}
