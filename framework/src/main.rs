use std::{collections::HashMap, fmt::Debug};

use petgraph::{
    algo::toposort,
    graph::{DiGraph, EdgeIndex, NodeIndex},
    visit::Topo,
};

type PortData = i32;

#[derive(Debug)]
struct Port {
    _source_label: String,
    val: Option<Box<PortData>>,
    target_label: String,
}

//impl Port {
//    fn new(source_label: &str, target_label: &str) -> Self {
//        Self {
//            source_label: source_label.into(),
//            val: None,
//            target_label: target_label.into(),
//        }
//    }
//}
type PortId = (NodeIndex, String);

#[derive(Debug)]
struct Graph {
    g: DiGraph<Node, Port>,
}
impl Graph {
    fn new() -> Self {
        Graph {
            g: DiGraph::<Node, Port>::new(),
        }
    }
    fn add_node(&mut self, n_type: NodeType) -> NodeIndex {
        let id = self.g.node_count() + 1;
        self.g.add_node(Node::new(id.to_string(), n_type))
    }
    fn add_edge(&mut self, from: PortId, to: PortId) -> EdgeIndex {
        //// From node records it's output port with no reference to child
        self.g
            .node_weight_mut(from.0)
            .unwrap()
            .output
            .insert(from.1.clone(), None);

        //// To Node references its parent, From Node
        self.g
            .node_weight_mut(to.0)
            .unwrap()
            .input
            .insert(to.1.clone(), (from.0.clone(), from.1.clone()));

        //// Store the edge in the graph
        self.g.add_edge(
            from.0,
            to.0,
            Port {
                _source_label: from.1,
                target_label: to.1,
                val: None,
            },
        )
    }

    fn process(&mut self) {
        let mut g = self;
        let mut topo = Topo::new(&g.g);
        while let Some(nx) = topo.next(&g.g) {
            let node = g.g.node_weight(nx).unwrap();
            node.n_type.clone().compute(nx, &mut g);
            //println!("start {:?}", node);
            //println!("{:?}", g);
        }
    }
}

#[derive(Debug, Clone)]
enum NodeType {
    Const(i32),
    Add,
    _Sub,
    Offset(i32),
}

impl NodeType {
    fn compute(&self, nx: NodeIndex, g: &mut Graph) {
        match self {
            NodeType::Const(val) => {
                //let mut in_edges =
                //    g.g.neighbors_directed(nx, petgraph::Direction::Outgoing)
                //        .detach();
                //while let Some(edge) = in_edges.next_edge(&g.g) {
                //    let (_nw, ew) = g.g.index_twice_mut(nx, edge);
                //    ew.val = Some((*val).into());
                //}
                let node = g.g.node_weight_mut(nx).unwrap();
                node.output.insert("out".into(), Some(*val));
            }
            NodeType::Add => {
                let node = g.g.node_weight(nx).unwrap();

                let inputs = &node.input;
                let (a_index, _a_port) = inputs.get("a".into()).unwrap();
                let a =
                    g.g.node_weight(*a_index)
                        .unwrap()
                        .output
                        .get("out")
                        .unwrap();

                let (b_index, _b_port) = inputs.get("b".into()).unwrap();
                let b =
                    g.g.node_weight(*b_index)
                        .unwrap()
                        .output
                        .get("out")
                        .unwrap();

                //let in_edge = g.g.edges_directed(nx, petgraph::Direction::Incoming);
                //
                //let a = &in_edge
                //    .clone()
                //    .find(|e| e.weight().target_label == "a")
                //    .unwrap()
                //    .weight()
                //    .val;
                //let b = &in_edge
                //    .clone()
                //    .find(|e| e.weight().target_label == "b")
                //    .unwrap()
                //    .weight()
                //    .val;

                fn my_add(a: &i32, b: &i32) -> i32 {
                    a + b
                }

                let out = my_add(&a.as_ref().unwrap(), &b.as_ref().unwrap());

                let node = g.g.node_weight_mut(nx).unwrap();
                node.output.insert("out".into(), Some(out));

                //let mut in_edges =
                //    g.g.neighbors_directed(nx, petgraph::Direction::Outgoing)
                //        .detach();
                //while let Some(edge) = in_edges.next_edge(&g.g) {
                //    let (_nw, ew) = g.g.index_twice_mut(nx, edge);
                //    ew.val = Some(out.into());
                //}
            }
            NodeType::_Sub => {}
            NodeType::Offset(val) => {
                let node = g.g.node_weight(nx).unwrap();

                let inputs = &node.input;
                let (a_index, _a_port) = inputs.get("a".into()).unwrap();
                let a =
                    g.g.node_weight(*a_index)
                        .unwrap()
                        .output
                        .get("out")
                        .unwrap();

                //let in_edge = g.g.edges_directed(nx, petgraph::Direction::Incoming);
                //dbg!(&nx);
                //dbg!(&in_edge);

                //let a = &in_edge
                //    .clone()
                //    .find(|e| dbg!(&e.weight().target_label) == "a")
                //    .unwrap()
                //    .weight()
                //    .val;

                fn my_add(a: &i32, b: &i32) -> i32 {
                    a + b
                }

                let out = my_add(&a.as_ref().unwrap(), &val);

                let node = g.g.node_weight_mut(nx).unwrap();
                node.output.insert("out".into(), Some(out));
            }
        }
    }
}

struct Node {
    name: String,
    input: HashMap<String, PortId>,
    output: HashMap<String, Option<PortData>>, // outputs can be connected to any number of inputs
    n_type: NodeType,
}

impl Node {
    fn new(name: String, n_type: NodeType) -> Self {
        Node {
            name,
            n_type,
            input: [].into(),
            output: [].into(),
        }
    }
}

fn main() {
    let mut g = Graph::new();

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

    g.add_edge((n4, "out".into()), (n5, "a".into()));

    g.process();
    println!("{:?}", &g);
    println!("{:?}", toposort(&g.g, None));

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
            self.name, self.n_type, self.input, self.output
        )
    }
}
