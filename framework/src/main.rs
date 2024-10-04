use core::time;
use std::path::PathBuf;
use std::thread::{self};

use gpi_framework::port::PortType;
use gpi_framework::{network::Network, node_type::NodeType};
use gpirs::pyo;

fn main() {
    initialize_python();
    let mut g = Network::default();

    let n1 = g.add_node(
        NodeType::Fill(1., 100_000_000),
        vec![],
        vec![("out".into(), PortType::Vec)],
    );
    let n2 = g.add_node(
        NodeType::Offset(0.),
        vec![("a".into(), PortType::Vec)],
        vec![("out".into(), PortType::Vec)],
    );
    let n3 = g.add_node(
        NodeType::Offset(1.),
        vec![("a".into(), PortType::Vec)],
        vec![("out".into(), PortType::Vec)],
    );
    //let n4 = g.add_node(
    //    NodeType::Python("scale_array".into()),
    //    vec![("a".into(), PortType::Vec), ("b".into(), PortType::Real)],
    //    vec![("out".into(), PortType::Vec)],
    //);
    let n4 = g.add_node(
        NodeType::Add,
        vec![("a".into(), PortType::Vec), ("b".into(), PortType::Vec)],
        vec![("out".into(), PortType::Vec)],
    );
    let n5 = g.add_node(
        NodeType::Sum,
        vec![("a".into(), PortType::Vec)],
        vec![("out".into(), PortType::Real)],
    );

    g.connect_nodes(n1, "out", n2, "a");
    g.connect_nodes(n1, "out", n3, "a");
    g.connect_nodes(n2, "out", n4, "a");
    g.connect_nodes(n3, "out", n4, "b");
    g.connect_nodes(n4, "out", n5, "a");

    println!("{:?}", &g);
    g.process();
    println!("{:?}", &g);
    let ten_millis = time::Duration::from_secs(10);

    thread::sleep(ten_millis);
}

fn initialize_python() {
    let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    d.push("../nodes/");
    let _ = pyo::initialize_gpipy();
}
