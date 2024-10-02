use gpi_framework::{network::Network, node_type::NodeType};
use petgraph::algo::toposort;

fn main() {
    let mut g = Network::new();

    let n1 = g.add_node(NodeType::Const(9));
    let n2 = g.add_node(NodeType::Offset(1));
    let n3 = g.add_node(NodeType::Offset(3));
    let n4 = g.add_node(NodeType::Add);
    let n5 = g.add_node(NodeType::Offset(3));

    g.add_edge((n1, "out".into()), (n2, "a".into()));
    g.add_edge((n1, "out".into()), (n3, "a".into()));
    g.add_edge((n2, "out".into()), (n4, "a".into()));
    g.add_edge((n3, "out".into()), (n4, "b".into()));
    g.add_edge((n4, "out".into()), (n5, "a".into()));

    println!("{:?}", &g);
    println!("{:?}", toposort(&g.g, None));
    g.process();
    println!("{:?}", &g);
    println!("{:?}", toposort(&g.g, None));
}
