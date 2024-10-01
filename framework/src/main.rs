use std::{collections::HashMap, fmt::Debug, rc::Rc};

use petgraph::{graph::DiGraph, visit::Topo, Direction::Incoming};
#[derive(Debug)]
struct Port {
    _source_label: String,
    val: Option<Rc<i32>>,
    target_label: String,
}
impl Port {
    fn new(source_label: &str, target_label: &str) -> Self {
        Self {
            _source_label: source_label.into(),
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
    inputs: HashMap<String, Option<Rc<i32>>>,
    outputs: HashMap<String, Option<Rc<i32>>>,
    n_type: NodeType,
}

impl Node {
    fn new(name: String, n_type: NodeType, inputs: Vec<String>, outputs: Vec<String>) -> Self {
        Node {
            name,
            n_type,
            inputs: inputs.iter().map(|s| (s.clone(), None)).collect(),
            outputs: outputs.iter().map(|s| (s.clone(), None)).collect(),
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

    let mut g = Graph {
        g: DiGraph::<Node, Port>::new(),
    };
    let n1 = g.g.add_node(node1);
    let n2 = g.g.add_node(node2);
    let n3 = g.g.add_node(node3);
    let n4 = g.g.add_node(node4);
    g.g.add_edge(n1, n2, Port::new("out", "a"));
    g.g.add_edge(n1, n3, Port::new("out", "a"));
    g.g.add_edge(n2, n4, Port::new("out", "a"));
    g.g.add_edge(n3, n4, Port::new("out", "b"));

    let mut topo = Topo::new(&g.g);
    while let Some(nx) = topo.next(&g.g) {
        //let in_edges = g.g.neighbors_directed(nx, petgraph::Direction::Incoming);
        //edges.fold(HashMap::new(), |acc, e| );
        //let mut edge_walker = in_edges.detach();
        // use a walker -- a detached neighbors iterator
        //
        let mut edges = g.g.neighbors_directed(nx, Incoming).detach();
        while let Some(edge) = edges.next_edge(&g.g) {
            let (nw, ew) = g.g.index_twice_mut(nx, edge);
            //PERF: could this clone be avoided using something like
            // rfcell or COW or someting?
            dbg!(&nw);
            nw.inputs.insert(ew.target_label.clone(), ew.val.clone());
            dbg!(&nw);
        }
        //
        //

        ////let node = g.g.node_weight_mut(nx).unwrap();
        //while let Some(edge) = edge_walker.next_edge(&g.g) {
        //    //// get parent value
        //    let parent_index = g.g.edge_endpoints(edge).unwrap().0;
        //    let parent = g.g.node_weight(parent_index).unwrap();
        //    //let (parent, ewp) = g.g.index_twice_mut(parent_index, edge);
        //    let (node, ew) = g.g.index_twice_mut(nx, edge);
        //
        //    let val = parent.outputs.get(&ew.0).unwrap();
        //
        //    //// copy value into input
        //    node.inputs.insert(ew.1.clone(), *val);
        //}

        let node = g.g.node_weight_mut(nx).unwrap();
        println!("start {:?}", node);

        match node.n_type {
            NodeType::Const(val) => {
                node.outputs.insert("out".into(), Some(val.into()));
            }
            NodeType::Add => {
                let out = node.inputs.get("a").unwrap().as_ref().unwrap().as_ref()
                    + node.inputs.get("b").unwrap().as_ref().unwrap().as_ref();
                node.outputs.insert("out".into(), Some(out.into()));
            }
            NodeType::_Sub => {}
            NodeType::Offset(val) => {
                let out = node
                    .inputs
                    .get("a")
                    .expect("input a should exist for Offset Nodes")
                    .as_ref()
                    .expect("Input should be populated")
                    .as_ref()
                    + val;
                node.outputs.insert("out".into(), Some(out.into()));
            }
        };
        println!("end {:?}", node);

        let mut out_edges =
            g.g.neighbors_directed(nx, petgraph::Direction::Outgoing)
                .detach();
        while let Some(edge) = out_edges.next_edge(&g.g) {
            let (nw, ew) = g.g.index_twice_mut(nx, edge);
            //PERF: could this clone be avoided using something like
            // rfcell or COW or someting?
            dbg!(&ew);
            ew.val = nw.outputs.get(&ew._source_label).unwrap().clone();
            dbg!(&ew);
        }
    }
    //println!("{:?}", &g);
    //println!("{:?}", toposort(&g.g, None));

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
