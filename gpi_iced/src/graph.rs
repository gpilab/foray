use derive_more::Debug;
use smol_str::SmolStr;
use std::{cell::RefCell, collections::HashMap};

use ordermap::OrderMap;

/// Graph node is a template for for types of nodes stored in the graph
#[derive(Debug)]
pub struct GraphNode<NodeData, PortType, WireData> {
    pub data: NodeData,
    pub inputs: OrderMap<PortName, PortType>,
    pub outputs: OrderMap<PortName, PortType>,
    #[debug(skip)]
    pub compute: Compute<WireData>,
}

impl<NodeData, PortType, PortData> GraphNode<NodeData, PortType, PortData>
where
    PortType: Clone,
{
    pub fn new(
        data: NodeData,
        inputs: Vec<(&str, &PortType)>,
        outputs: Vec<(&str, &PortType)>,
        compute: Compute<PortData>,
    ) -> Self {
        Self {
            data,
            inputs: inputs
                .into_iter()
                .map(|(port_name, port_type)| (port_name.into(), port_type.clone()))
                .collect(),
            outputs: outputs
                .into_iter()
                .map(|(port_name, port_type)| (port_name.into(), port_type.clone()))
                .collect(),
            compute,
        }
    }
}

//#[derive(Clone, Debug, PartialEq)]
//pub enum PortId {
//    In(NodeIndex, PortName),
//    Out(NodeIndex, PortName),
//}
//impl From<PortId> for (NodeIndex, PortName) {
//    fn from(value: PortId) -> Self {
//        match value {
//            PortId::In(nx, name) => (nx, name),
//            PortId::Out(nx, name) => (nx, name),
//        }
//    }
//}

type PortName = SmolStr;

type NodeIndex = u32;

type Compute<WireData> =
    Box<dyn Fn(HashMap<PortName, &RefCell<WireData>>) -> HashMap<PortName, WireData>>;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum IO {
    In,
    Out,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PortRef {
    pub node: u32,
    pub name: PortName,
    pub io: IO,
}
type Edge = (PortRef, PortRef);

#[derive(Default, Debug)]
pub struct Graph<NodeData, PortType, WireData>
where
    PortType: Clone,
{
    nodes: ordermap::OrderMap<NodeIndex, GraphNode<NodeData, PortType, WireData>>,
    edges: Vec<Edge>,
    wire_data: HashMap<(NodeIndex, PortName), RefCell<WireData>>,
    next_id: NodeIndex,
}

impl<NodeData, PortType, WireData> Graph<NodeData, PortType, WireData>
where
    PortType: Clone + Debug,
    WireData: Debug,
    NodeData: Debug,
{
    pub fn new() -> Self {
        Self {
            nodes: OrderMap::new(),
            edges: vec![],
            wire_data: HashMap::new(),
            next_id: 0,
        }
    }

    /// Add a new node to the graph, returns the node's index
    pub fn add_node(&mut self, node: GraphNode<NodeData, PortType, WireData>) -> NodeIndex {
        let id = self.next_id;
        self.nodes.insert(id, node);
        self.next_id += 1;
        id
    }

    ///Get the node value at a given index
    ///panics if index is not valid!
    ///Use the index returned from `add_node` to ensure it exists
    pub fn get_node(&self, nx: NodeIndex) -> &GraphNode<NodeData, PortType, WireData> {
        &self.nodes[&nx]
    }

    ///Get a mutable reference  to a node value at a given index
    ///panics if index is not valid
    ///Use the index returned from `add_node` to ensure it exists
    pub fn get_mut_node(&mut self, nx: NodeIndex) -> &mut GraphNode<NodeData, PortType, WireData> {
        self.nodes.get_mut(&nx).unwrap()
    }

    /// get a list of node indices
    pub fn nodes_ref(&self) -> Vec<NodeIndex> {
        self.nodes.keys().copied().collect()
    }

    ///Set the node value of an existing node
    pub fn set_node_data(&mut self, nx: NodeIndex, value: NodeData) {
        self.nodes.get_mut(&nx).unwrap().data = value;
    }

    pub fn update_wire_data(&mut self, nx: NodeIndex, outputs: HashMap<PortName, WireData>) {
        for (port_name, wire_data) in outputs.into_iter() {
            self.wire_data.insert((nx, port_name), wire_data.into());
        }
    }

    pub fn get_wire_data(&self, nx: &NodeIndex, port_name: &str) -> Option<&RefCell<WireData>> {
        self.wire_data.get(&(*nx, port_name.into()))
    }

    /// create a connection between two nodes, and an associated
    /// label for each node
    pub fn add_edge(
        &mut self,
        from: (NodeIndex, impl Into<PortName>),
        to: (NodeIndex, impl Into<PortName>),
    ) {
        let from = PortRef {
            node: from.0,
            name: from.1.into(),
            io: IO::Out,
        };
        let to = PortRef {
            node: to.0,
            name: to.1.into(),
            io: IO::In,
        };

        //TODO: check for compatiablity, or if the edge alread exists
        // warn if already exists, panic/return result if incompatabible
        self.edges.push((from, to));
    }

    ///Find a nodes direct parents and the associated labels
    pub fn incoming_edges(&self, nx: &NodeIndex) -> Vec<(PortRef, PortRef)> {
        self.edges
            .iter()
            .filter_map(|(from, to)| {
                if to.node == *nx {
                    Some((from.clone(), to.clone()))
                } else {
                    None
                }
            })
            .collect()
    }

    /// find the edges that that originate at `nx`
    pub fn outgoing_edges(&self, nx: &NodeIndex) -> Vec<PortRef> {
        self.edges
            .iter()
            .filter_map(|(from, to)| {
                if from.node == *nx {
                    Some(to.clone())
                } else {
                    None
                }
            })
            .collect()
        // loop over all nodes, and ports and find the Nodes and ports that
        // connect to `nx`
        //self.nodes
        //    .iter()
        //    .filter_map(|(possible_descendent_id, possible_descendent)| {
        //        possible_descendent
        //            .inputs
        //            .iter()
        //            .filter_map(|p| p.connection.clone())
        //            .find(|(from_nx, _pt)| *nx == *from_nx)
        //            .map(|(_, p_id)| (*possible_descendent_id, p_id))
        //    })
        //    .collect()
    }

    /// topological sort using Kahn's algorithm
    /// returns a list of NodeIndices
    pub fn topological_sort(&self) -> Vec<NodeIndex> {
        let mut sorted = vec![];
        let mut working_edges = self.edges.clone();
        //let mut working_edges: Vec<((NodeIndex, PortName), (NodeIndex, PortName))> = self
        //    .nodes
        //    .iter()
        //    .flat_map(|(nx, node)| {
        //        node.inputs.iter().flat_map(|p| {
        //            p.connection
        //                .clone()
        //                .map(|(parent_node_id, parent_port_id)| {
        //                    ((parent_node_id, parent_port_id), (*nx, p.name.clone()))
        //                })
        //        })
        //    })
        //    .collect();
        let mut no_incoming: Vec<_> = self
            .nodes
            .keys()
            .filter(|nx| !Self::has_incoming(nx, &working_edges))
            .copied()
            .collect();

        while let Some(nx) = no_incoming.pop() {
            sorted.push(nx);
            while let Some(ex) = Self::next_connected_edge(&nx, &working_edges) {
                let edge = working_edges.swap_remove(ex);
                let mx = edge.1.node;
                if !Self::has_incoming(&mx, &working_edges) {
                    no_incoming.push(mx);
                }
            }
        }
        if working_edges.is_empty() {
            sorted
        } else {
            panic!("graph has cycles!")
        }
    }

    pub fn exectute_sub_network(&mut self, root: NodeIndex) {
        let nodes: Vec<_> = self
            .topological_sort()
            .into_iter()
            .filter(|&nx| self.is_self_or_dependent(root, nx))
            .collect();
        nodes.iter().for_each(|nx| self.compute_node(nx))
    }

    /// Execute network using topological sort
    pub fn execute_network(&mut self) {
        let mut ordered = self.topological_sort();
        ordered.iter_mut().for_each(|nx| self.compute_node(nx))
    }

    pub fn get_parent(&self, nx: &NodeIndex, in_port: PortName) -> Option<PortRef> {
        self.edges
            .iter()
            .find(|(_from, to)| to.node == *nx && to.name == in_port)
            .map(|(from, _to)| from.clone())
    }
    fn compute_node(&mut self, nx: &NodeIndex) {
        //let incoming_edges = &self.incoming_edges(nx);
        dbg!(nx);
        let node = self.get_node(*nx);
        //TODO: Handle errors nicely
        let inputs: Option<HashMap<PortName, &RefCell<WireData>>> = node
            .inputs
            .keys()
            .filter_map(|port_name| {
                self.get_parent(nx, port_name.clone()).map(|out_port| {
                    self.wire_data
                        .get(&(out_port.node, out_port.name))
                        .map(|data| (port_name.clone(), data))
                })
            })
            .collect();

        if let Some(inputs) = inputs {
            if dbg!(inputs.len()) == dbg!(node.inputs.len()) {
                let outputs = (*node.compute)(inputs);
                self.update_wire_data(*dbg!(nx), dbg!(outputs));
                dbg!(self);
            }
        }
    }

    fn is_self_or_dependent(&self, root: NodeIndex, to_check: NodeIndex) -> bool {
        if root == to_check {
            true
        } else {
            self.incoming_edges(&to_check)
                .into_iter()
                .any(|(from, _to)| self.is_self_or_dependent(root, from.node))
        }
    }

    /// determine if a node has any incoming connections
    fn has_incoming(nx: &NodeIndex, edges: &[Edge]) -> bool {
        edges.iter().any(|(_from, to)| to.node == *nx)
    }

    /// find the index of `edges` corresponding to the first
    /// connection starting from `nx` (if it exists)
    fn next_connected_edge(nx: &NodeIndex, edges: &[Edge]) -> Option<usize> {
        edges.iter().position(|(from, _to)| from.node == *nx)
    }
}

pub fn identity_node<NodeData, PortType: Clone, WireData: Clone>(
    data: NodeData,
    port_type: &PortType,
) -> GraphNode<NodeData, PortType, WireData> {
    GraphNode::new(
        data,
        vec![("in", port_type)],
        vec![("out", port_type)],
        Box::new(|a| [("out".into(), a["in"].borrow().clone())].into()),
    )
}

pub fn constant_node<NodeData: 'static, PortType: Clone + 'static, WireData: Clone + 'static>(
    data: NodeData,
    out_data: WireData,
    out_type: &PortType,
) -> GraphNode<NodeData, PortType, WireData> {
    GraphNode::new(
        data,
        vec![],
        vec![("out", out_type)],
        Box::new(move |_| [("out".into(), out_data.clone())].into()),
    )
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn sort() {
        let mut g: Graph<u32, (), ()> = Graph::new();

        let n8 = g.add_node(identity_node(8, &()));
        let n7 = g.add_node(identity_node(7, &()));
        let n6 = g.add_node(identity_node(6, &()));
        let n5 = g.add_node(identity_node(5, &()));
        let n4 = g.add_node(identity_node(4, &()));
        let n3 = g.add_node(identity_node(3, &()));
        let n2 = g.add_node(identity_node(2, &()));
        let n1 = g.add_node(identity_node(1, &()));

        g.add_edge((n1, "out"), (n3, "in"));
        g.add_edge((n1, "out"), (n2, "in"));
        g.add_edge((n3, "out"), (n4, "in"));
        g.add_edge((n4, "out"), (n5, "in"));
        g.add_edge((n5, "out"), (n6, "in"));
        g.add_edge((n6, "out"), (n7, "in"));
        g.add_edge((n7, "out"), (n8, "in"));
        assert_eq!(g.topological_sort(), vec![7, 6, 5, 4, 3, 2, 1, 0]);
    }

    #[test]
    fn process() {
        let mut g: Graph<(), (), u32> = Graph::new();

        let n1 = g.add_node(constant_node((), 7, &()));
        let n2 = g.add_node(identity_node((), &()));
        let n3 = g.add_node(identity_node((), &()));
        let n4 = g.add_node(identity_node((), &()));
        // leave a node unconnected to check that it doesn't get a value propogated
        let n_unconnected = g.add_node(identity_node((), &()));

        g.add_edge((n1, "out"), (n3, "in"));
        g.add_edge((n1, "out"), (n2, "in"));
        g.add_edge((n3, "out"), (n4, "in"));

        //Propogate values
        g.execute_network();

        assert_eq!(g.get_wire_data(&n1, "out"), Some(RefCell::new(7)).as_ref());
        assert_eq!(g.get_wire_data(&n2, "out"), Some(RefCell::new(7)).as_ref());
        assert_eq!(g.get_wire_data(&n3, "out"), Some(RefCell::new(7)).as_ref());
        assert_eq!(g.get_wire_data(&n_unconnected, "out"), None);
    }
    //#[test]
    //fn vertex() {
    //    type Node<'a> = HashMap<&'a str, Vec<u32>>;
    //
    //    let mut g: Graph<Vec<u32>, &str, ()> = Graph::new();
    //    let n1 = g.add_node([("out", vec![1, 2, 3])].into());
    //    let n2 = g.add_node([("out", vec![4, 5, 6])].into());
    //    let n3 = g.add_node([("out", vec![])].into());
    //    let n4 = g.add_node([("out", vec![])].into());
    //
    //    g.add_edge((n1, "o1"), (n2, "i2"));
    //    g.add_edge((n1, "o1"), (n3, "i3"));
    //    g.add_edge((n2, "o2"), (n3, "i3"));
    //    g.add_edge((n3, "o3"), (n4, "i4"));
    //
    //    g.topological_sort().iter_mut().for_each(|nx| {
    //        let new_vec: Vec<_> = g
    //            .incoming_edges(nx)
    //            .into_iter()
    //            .flat_map(|(from, _to)| g.get_node(from.0)["out"].clone())
    //            .collect();
    //
    //        let node = g.get_mut_node(*nx);
    //        node.get_mut("out").unwrap().extend(new_vec);
    //    });
    //    assert_eq!(*g.get_node(n1)["out"], vec![1, 2, 3]);
    //    assert_eq!(*g.get_node(n2)["out"], vec![4, 5, 6, 1, 2, 3]);
    //    assert_eq!(*g.get_node(n3)["out"], vec![1, 2, 3, 4, 5, 6, 1, 2, 3]);
    //    assert_eq!(*g.get_node(n4)["out"], vec![1, 2, 3, 4, 5, 6, 1, 2, 3]);
    //}
    //
    //use numpy::PyArrayMethods;
    //use pyo3::{prepare_freethreaded_python, Python};
    //
    //#[test]
    //fn simple_ndarray() {
    //    prepare_freethreaded_python();
    //
    //    Python::with_gil(|py| {
    //        let res = get_array(py).readonly();
    //        let slice = res.as_slice().unwrap();
    //
    //        assert_eq!(&[1, 2, 3], slice);
    //    })
    //}
}
