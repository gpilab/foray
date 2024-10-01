use std::{cell::RefCell, fmt::Debug, rc::Rc};

use petgraph::{algo::toposort, graph::DiGraph, visit::Topo};
#[derive(Debug)]
struct Port {
    source_label: String,
    val: Option<Rc<RefCell<i32>>>,
    target_label: String,
}
impl Port {
    fn new(source_label: &str, target_label: &str) -> Self {
        Self {
            source_label: source_label.into(),
            val: None,
            target_label: target_label.into(),
        }
    }
}

#[derive(Debug)]
struct Graph {
    g: DiGraph<Node, Port>,
}

#[derive(Debug)]
enum NodeType {
    Const(i32),
    Add,
    _Sub,
    Offset(i32),
}

struct Node {
    name: String,
    inputs: Vec<String>,
    outputs: Vec<String>,
    n_type: NodeType,
}

impl Node {
    fn new(name: String, n_type: NodeType, inputs: Vec<String>, outputs: Vec<String>) -> Self {
        Node {
            name,
            n_type,
            inputs,
            outputs,
        }
    }
}

fn main() {
    let node1 = Node::new(
        "n1".into(),
        NodeType::Const(9),
        [].into(),
        ["out"].map(String::from).to_vec(),
    );
    let node2 = Node::new(
        "n2".into(),
        NodeType::Offset(1),
        ["a"].map(String::from).to_vec(),
        ["out"].map(String::from).to_vec(),
    );
    let node3 = Node::new(
        "n3".into(),
        NodeType::Offset(3),
        ["a"].map(String::from).to_vec(),
        ["out"].map(String::from).to_vec(),
    );
    let node4 = Node::new(
        "n4".into(),
        NodeType::Add,
        ["a", "b"].map(String::from).to_vec(),
        ["out"].map(String::from).to_vec(),
    );
    let node5 = Node::new(
        "n3".into(),
        NodeType::Offset(3),
        ["a"].map(String::from).to_vec(),
        ["out"].map(String::from).to_vec(),
    );

    let mut g = Graph {
        g: DiGraph::<Node, Port>::new(),
    };
    let n1 = g.g.add_node(node1);
    let n2 = g.g.add_node(node2);
    let n3 = g.g.add_node(node3);
    let n4 = g.g.add_node(node4);
    let n5 = g.g.add_node(node5);
    g.g.add_edge(n1, n2, Port::new("out", "a"));
    g.g.add_edge(n1, n3, Port::new("out", "a"));
    g.g.add_edge(n2, n4, Port::new("out", "a"));
    g.g.add_edge(n3, n4, Port::new("out", "b"));
    g.g.add_edge(n4, n5, Port::new("out", "a"));

    let mut topo = Topo::new(&g.g);
    while let Some(nx) = topo.next(&g.g) {
        let node = g.g.node_weight(nx).unwrap();
        println!("start {:?}", node);
        println!("{:?}", g);

        match node.n_type {
            NodeType::Const(val) => {
                let mut in_edges =
                    g.g.neighbors_directed(nx, petgraph::Direction::Outgoing)
                        .detach();
                while let Some(edge) = in_edges.next_edge(&g.g) {
                    let (_nw, ew) = g.g.index_twice_mut(nx, edge);
                    ew.val = Some(Rc::new(val.into()));
                }
            }
            NodeType::Add => {
                let in_edge = g.g.edges_directed(nx, petgraph::Direction::Incoming);

                let a = &in_edge
                    .clone()
                    .find(|e| e.weight().target_label == "a")
                    .unwrap()
                    .weight()
                    .val;
                let b = &in_edge
                    .clone()
                    .find(|e| e.weight().target_label == "b")
                    .unwrap()
                    .weight()
                    .val;

                fn my_add(a: &i32, b: &i32) -> i32 {
                    a + b
                }
                let out = my_add(&a.as_ref().unwrap().borrow(), &b.as_ref().unwrap().borrow());

                let mut in_edges =
                    g.g.neighbors_directed(nx, petgraph::Direction::Outgoing)
                        .detach();
                while let Some(edge) = in_edges.next_edge(&g.g) {
                    let (_nw, ew) = g.g.index_twice_mut(nx, edge);
                    ew.val = Some(Rc::new(out.into()));
                }
            }
            NodeType::_Sub => {}
            NodeType::Offset(val) => {
                let in_edge = g.g.edges_directed(nx, petgraph::Direction::Incoming);
                dbg!(&nx);
                dbg!(&in_edge);

                let a = &in_edge
                    .clone()
                    .find(|e| dbg!(&e.weight().target_label) == "a")
                    .unwrap()
                    .weight()
                    .val;

                fn my_add(a: &i32, b: &i32) -> i32 {
                    a + b
                }

                let out = my_add(&a.as_ref().unwrap().borrow(), &val);

                let mut in_edges =
                    g.g.neighbors_directed(nx, petgraph::Direction::Outgoing)
                        .detach();
                while let Some(edge) = in_edges.next_edge(&g.g) {
                    let (_nw, ew) = g.g.index_twice_mut(nx, edge);
                    ew.val = Some(Rc::new(out.into()));
                }
            }
        };
    }
    println!("{:?}", &g);
    println!("{:?}", toposort(&g.g, None));

    // Output the tree to `graphviz` `DOT` format
    //println!("{:?}", Dot::with_config(&g.g, &[Config::EdgeNoLabel]));
}

impl Debug for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "\t{{
\t  {},
\t  type:{:?},
\t  inputs:{:?},
\t  outpus:{:?}
\t}}
",
            self.name, self.n_type, self.inputs, self.outputs
        )
    }
}
