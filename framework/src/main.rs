use gpi_framework::port::PortType;
use gpi_framework::{network::Network, node::Node, node_type::NodeType};
use petgraph::algo::toposort;

fn main() {
    let mut g = Network::new();

    let n1 = Node::new(
        NodeType::Const(9.),
        vec![],
        vec![("out".into(), PortType::Vec)],
    );
    let n2 = Node::new(
        NodeType::Offset(1.),
        vec![("a".into(), PortType::Vec)],
        vec![("out".into(), PortType::Vec)],
    );
    let n3 = Node::new(
        NodeType::Offset(3.),
        vec![("a".into(), PortType::Vec)],
        vec![("out".into(), PortType::Vec)],
    );
    let n4 = Node::new(
        NodeType::Add,
        vec![("a".into(), PortType::Vec), ("b".into(), PortType::Vec)],
        vec![("out".into(), PortType::Vec)],
    );
    let n5 = Node::new(
        NodeType::Offset(3.),
        vec![("a".into(), PortType::Vec), ("b".into(), PortType::Vec)],
        vec![("out".into(), PortType::Vec)],
    );
    let nx1 = g.add_node(n1);
    let nx2 = g.add_node(n2);
    let nx3 = g.add_node(n3);
    let nx4 = g.add_node(n4);
    let nx5 = g.add_node(n5);

    g.add_edge(nx1, "out".into(), nx2, "a".into());
    g.add_edge(nx1, "out".into(), nx3, "a".into());
    g.add_edge(nx2, "out".into(), nx4, "a".into());
    g.add_edge(nx3, "out".into(), nx4, "b".into());
    g.add_edge(nx4, "out".into(), nx5, "a".into());

    println!("{:?}", &g);
    println!("{:?}", toposort(&g.g, None));
    g.process();
    println!("{:?}", &g);
    println!("{:?}", toposort(&g.g, None));
}
