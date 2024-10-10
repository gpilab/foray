use gpi_framework::port::PortType;
use gpi_framework::{network::Network, node_type::NodeType};

#[test]
fn process() {
    let mut g = Network::default();

    //let n1 = g.add_node(
    //    NodeType::Fill(9., 3),
    //    vec![],
    //    vec![("out".into(), PortType::Vec)],
    //);
    //let n2 = g.add_node(
    //    NodeType::Offset(1.),
    //    vec![("a".into(), PortType::Vec)],
    //    vec![("out".into(), PortType::Vec)],
    //);
    //let n3 = g.add_node(
    //    NodeType::Offset(3.),
    //    vec![("a".into(), PortType::Vec)],
    //    vec![("out".into(), PortType::Vec)],
    //);
    //let n4 = g.add_node(
    //    NodeType::Add,
    //    vec![("a".into(), PortType::Vec), ("b".into(), PortType::Vec)],
    //    vec![("out".into(), PortType::Vec)],
    //);
    //let n5 = g.add_node(
    //    NodeType::Sum,
    //    vec![("a".into(), PortType::Vec)],
    //    vec![("out".into(), PortType::Real)],
    //);
    //
    //g.connect_nodes(n1, "out", n2, "a");
    //g.connect_nodes(n1, "out", n3, "a");
    //g.connect_nodes(n2, "out", n4, "a");
    //g.connect_nodes(n3, "out", n4, "b");
    //g.connect_nodes(n4, "out", n5, "a");
    //
    //g.process();
    //assert_eq!(
    //    *g.get_output_data((n5, "out".into())),
    //    PortValue::Real(66.0)
    //)
}
