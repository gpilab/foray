use derive_more::Debug;
use std::hash::Hash;

use ordermap::{Equivalent, OrderMap};

type NodeIndex = u32;
type Edge<PortId> = ((NodeIndex, PortId), (NodeIndex, PortId));

type Compute<PortId, PortType, WireData> = Box<
    dyn Fn(
        Vec<&OutputPort<PortId, PortType, WireData>>,
    ) -> Vec<OutputPort<PortId, PortType, WireData>>,
>;

pub fn identity_node<NodeData, PortType: Clone, WireData: Clone>(
    data: NodeData,
    port_type: PortType,
) -> GraphNode<NodeData, String, PortType, WireData> {
    GraphNode::new(
        data,
        &[("in".into(), port_type.clone())],
        &[("out".into(), port_type.clone())],
        Box::new(|a| {
            vec![OutputPort::new(
                "out".into(),
                a[0].variant.clone(),
                a[0].data.clone(),
            )]
        }),
    )
}

pub fn constant_node<NodeData: 'static, PortType: Clone + 'static, WireData: Clone + 'static>(
    data: NodeData,
    out_data: WireData,
    out_type: PortType,
) -> GraphNode<NodeData, String, PortType, WireData> {
    GraphNode::new(
        data,
        &[],
        &[("out".into(), out_type.clone())],
        Box::new(move |_| {
            vec![OutputPort::new(
                "out".into(),
                out_type.clone(),
                Some(out_data.clone()),
            )]
        }),
    )
}
#[derive(Debug)]
pub struct GraphNode<NodeData, PortId, PortType, WireData> {
    pub data: NodeData,
    pub inputs: Vec<InputPort<PortId, PortType>>,
    pub outputs: Vec<OutputPort<PortId, PortType, WireData>>,
    #[debug(skip)]
    pub compute: Compute<PortId, PortType, WireData>,
}

impl<NodeData, PortId, PortType, PortData> GraphNode<NodeData, PortId, PortType, PortData>
where
    PortId: Clone,
    PortType: Clone,
{
    pub fn new(
        data: NodeData,
        inputs: &[(PortId, PortType)],
        outputs: &[(PortId, PortType)],
        compute: Compute<PortId, PortType, PortData>,
    ) -> Self {
        Self {
            data,
            inputs: inputs
                .iter()
                .map(|(p_id, p_type)| InputPort::new(p_id.clone(), p_type.clone()))
                .collect(),
            outputs: outputs
                .iter()
                .map(|(p_id, p_type)| OutputPort::new(p_id.clone(), p_type.clone(), None))
                .collect(),
            compute,
        }
    }
}

#[derive(Clone, Debug)]
pub struct InputPort<PortId, PortType> {
    id: PortId,
    //TODO: restrict connections to only matching variants
    variant: PortType,
    connection: Option<(NodeIndex, PortId)>,
}

impl<PortId, PortType> InputPort<PortId, PortType> {
    pub fn new(id: PortId, variant: PortType) -> Self {
        Self {
            id,
            variant,
            connection: None,
        }
    }
}

#[derive(Debug)]
pub struct OutputPort<PortId, PortType, PortData> {
    pub id: PortId,
    pub variant: PortType,
    pub data: Option<PortData>,
}
impl<PortId, PortType, PortData> OutputPort<PortId, PortType, PortData> {
    pub fn new(id: PortId, variant: PortType, data: Option<PortData>) -> Self {
        Self { id, variant, data }
    }
}

// Input Ports always have a type.
// If connected, they identify their connection with a `NodeIndex` and `PortName`
//#[derive(Debug, Serialize)]
//pub(crate) enum InputPort {
//    Empty(PortType),
//    Connected(PortType, NodeIndex, PortName),
//}

/// Output ports always have a type.
/// They optionally have data
//#[derive(Debug)]
//pub(crate) enum OutputPort<'a, T> {
//    Empty(T),
//    Filled(PortData<T>),
//}
//

#[derive(Default, Debug)]
pub struct Graph<NodeData, PortId, PortType, WireData>
where
    PortId: Clone,
    PortType: Clone,
{
    nodes: ordermap::OrderMap<NodeIndex, GraphNode<NodeData, PortId, PortType, WireData>>,
    next_id: NodeIndex,
}

impl<NodeData, PortId, PortType, WireData> Graph<NodeData, PortId, PortType, WireData>
where
    PortId: Clone + Hash + Equivalent<PortId> + Eq + Debug,
    PortType: Clone + Debug,
    WireData: Debug,
    NodeData: Debug,
{
    pub fn new() -> Self {
        Self {
            nodes: OrderMap::new(),
            next_id: 0,
        }
    }

    /// Add a new node to the graph
    pub fn add_node(&mut self, node: GraphNode<NodeData, PortId, PortType, WireData>) -> NodeIndex {
        let id = self.next_id;
        self.nodes.insert(id, node);
        self.next_id += 1;
        id
    }
    ///Get the node value at a given index
    ///panics if index is not valid
    pub fn get_node(&self, nx: NodeIndex) -> &GraphNode<NodeData, PortId, PortType, WireData> {
        &self.nodes[&nx]
    }
    pub fn nodes_ref(
        &self,
    ) -> &OrderMap<NodeIndex, GraphNode<NodeData, PortId, PortType, WireData>> {
        &self.nodes
    }

    ///Get a mutable reference  to a node value at a given index
    ///panics if index is not valid
    pub fn get_mut_node(
        &mut self,
        nx: NodeIndex,
    ) -> &mut GraphNode<NodeData, PortId, PortType, WireData> {
        self.nodes.get_mut(&nx).unwrap()
    }

    ///Set the node value of an existing node
    pub fn update_node_data(&mut self, nx: NodeIndex, value: NodeData) {
        self.nodes.get_mut(&nx).unwrap().data = value;
    }

    pub fn update_wire_output(
        &mut self,
        nx: NodeIndex,
        port_id: PortId,
        wire_data: Option<WireData>,
    ) {
        self.nodes
            .get_mut(&nx)
            .unwrap()
            .outputs
            .iter_mut()
            .find(|port| port.id == port_id)
            .unwrap()
            .data = wire_data;
    }

    /// create a connection between two nodes, and an associated
    /// label for each node
    pub fn add_edge(
        &mut self,
        from: (NodeIndex, impl Into<PortId>),
        to: (NodeIndex, impl Into<PortId>),
    ) {
        let from = (from.0, from.1.into());
        let to = (to.0, to.1.into());

        let from_port = &self
            .get_node(from.0)
            .outputs
            .iter()
            .find(|port| port.id == from.1);

        let to_port = self
            .get_node(to.0)
            .inputs
            .iter()
            .find(|port| port.id == to.1);

        if let (Some(_from_input), Some(_to_output)) = (from_port, to_port) {
            self.get_mut_node(to.0)
                .inputs
                .iter_mut()
                .find(|port| port.id == to.1)
                .unwrap()
                .connection = Some(from.clone());
        } else {
            panic!("Invalid edge connection: {:?},{:?}", from, to);
        }
    }

    ///Find a nodes direct parents and the associated labels
    pub fn incoming_edges(&self, nx: &NodeIndex) -> Vec<(NodeIndex, PortId)> {
        self.get_node(*nx)
            .inputs
            .iter()
            .filter_map(|port| port.connection.clone())
            .collect()
    }

    /// find the edges that that originate at `nx`
    pub fn outgoing_edges(&self, nx: &NodeIndex) -> Vec<(NodeIndex, PortId)> {
        // loop over all nodes, and ports and find the Nodes and ports that
        // connect to `nx`
        self.nodes
            .iter()
            .filter_map(|(possible_descendent_id, possible_descendent)| {
                possible_descendent
                    .inputs
                    .iter()
                    .filter_map(|p| p.connection.clone())
                    .find(|(from_nx, _pt)| *nx == *from_nx)
                    .map(|(_, p_id)| (*possible_descendent_id, p_id))
            })
            .collect()
    }

    /// topological sort using Kahn's algorithm
    /// returns a list of NodeIndices
    pub fn topological_sort(&self) -> Vec<NodeIndex> {
        let mut sorted = vec![];
        let mut working_edges: Vec<((NodeIndex, PortId), (NodeIndex, PortId))> = self
            .nodes
            .iter()
            .flat_map(|(nx, node)| {
                node.inputs.iter().flat_map(|p| {
                    p.connection
                        .clone()
                        .map(|(parent_node_id, parent_port_id)| {
                            ((parent_node_id, parent_port_id), (*nx, p.id.clone()))
                        })
                })
            })
            .collect();
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
                let mx = edge.1 .0;
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

    fn compute_node(&mut self, nx: &NodeIndex) {
        let incoming_edges = &self.incoming_edges(nx);

        let inputs: Vec<_> = incoming_edges
            .iter()
            .flat_map(|(nx, port_id)| {
                self.get_node(*nx)
                    .outputs
                    .iter()
                    .filter(|o| o.id == *port_id && o.data.is_some())
                    .collect::<Vec<_>>()
            })
            .collect();

        if inputs.len() != self.get_node(*nx).inputs.len() {
            println!(
                "Node:{} - Not enough inputs, expected: {}, received: {}",
                nx,
                self.get_node(*nx).inputs.len(),
                inputs.len()
            );
            return;
        }

        let node = self.get_node(*nx);
        let outputs = (*node.compute)(inputs);
        self.get_mut_node(*nx).outputs = outputs;
    }

    fn is_self_or_dependent(&self, root: NodeIndex, to_check: NodeIndex) -> bool {
        if root == to_check {
            true
        } else {
            self.incoming_edges(&to_check)
                .into_iter()
                .any(|(nx, _port_id)| self.is_self_or_dependent(root, nx))
        }
    }

    /// determine if a node has any incoming connections
    fn has_incoming(nx: &NodeIndex, edges: &[Edge<PortId>]) -> bool {
        edges.iter().any(|(_from, to)| to.0 == *nx)
    }

    /// find the index of `edges` corresponding to the first
    /// connection starting from `nx` (if it exists)
    fn next_connected_edge(nx: &NodeIndex, edges: &[Edge<PortId>]) -> Option<usize> {
        edges.iter().position(|(from, _to)| from.0 == *nx)
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn sort() {
        let mut g: Graph<u32, String, (), ()> = Graph::new();

        let n8 = g.add_node(identity_node(8, ()));
        let n7 = g.add_node(identity_node(7, ()));
        let n6 = g.add_node(identity_node(6, ()));
        let n5 = g.add_node(identity_node(5, ()));
        let n4 = g.add_node(identity_node(4, ()));
        let n3 = g.add_node(identity_node(3, ()));
        let n2 = g.add_node(identity_node(2, ()));
        let n1 = g.add_node(identity_node(1, ()));

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
        let mut g: Graph<(), String, (), u32> = Graph::new();

        let n1 = g.add_node(constant_node((), 7, ()));
        let n2 = g.add_node(identity_node((), ()));
        let n3 = g.add_node(identity_node((), ()));
        let n4 = g.add_node(identity_node((), ()));
        // leave a node unconnected to check that it doesn't get a value propogated
        let n_unconnected = g.add_node(identity_node((), ()));

        g.add_edge((n1, "out"), (n3, "in"));
        g.add_edge((n1, "out"), (n2, "in"));
        g.add_edge((n3, "out"), (n4, "in"));

        //Propogate values
        g.execute_network();

        assert_eq!(g.get_node(n1).outputs[0].data, Some(7));
        assert_eq!(g.get_node(n2).outputs[0].data, Some(7));
        assert_eq!(g.get_node(n3).outputs[0].data, Some(7));
        assert_eq!(g.get_node(n4).outputs[0].data, Some(7));
        assert_eq!(dbg!(g.get_node(n_unconnected)).outputs[0].data, None);
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
