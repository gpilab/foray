use std::{collections::HashMap, fmt::Debug};

type NodeIndex = u32;
/// A labeled starting/ending point of an edge
type LabeledVertex<S> = (NodeIndex, S);

#[derive(Default)]
pub struct Graph<T, S>
where
    S: Clone,
{
    nodes: HashMap<NodeIndex, T>,
    edges: Vec<(LabeledVertex<S>, LabeledVertex<S>)>,
    next_id: NodeIndex,
}

impl<T, S> Graph<T, S>
where
    S: Clone + Debug,
{
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
            edges: vec![],
            next_id: 0,
        }
    }

    /// Add a new node to the graph
    pub fn add_node(&mut self, node: T) -> NodeIndex {
        let id = self.next_id;
        self.nodes.insert(id, node);
        self.next_id += 1;
        id
    }
    ///Get the node value at a given index
    ///panics if index is not valid
    pub fn get_node(&self, nx: NodeIndex) -> &T {
        &self.nodes[&nx]
    }

    ///Set the node value of an existing node
    pub fn update_node(&mut self, nx: NodeIndex, value: T) {
        self.nodes.insert(nx, value);
    }

    /// create a connection between two nodes, and an associated
    /// label for each node
    pub fn add_edge(&mut self, from: LabeledVertex<S>, to: LabeledVertex<S>) {
        self.edges.push((from, to));
    }

    ///Find a nodes direct parents and the associated labels
    pub fn incoming_edges(&self, nx: &NodeIndex) -> Vec<(LabeledVertex<S>, LabeledVertex<S>)> {
        self.edges
            .iter()
            .filter(|(_from, to)| to.0 == *nx)
            .cloned()
            .collect()
    }

    /// find the edges that that originate at `nx`
    pub fn outgoing_edges(&self, nx: &NodeIndex) -> Vec<(LabeledVertex<S>, LabeledVertex<S>)> {
        self.edges
            .iter()
            .filter(|(from, _to)| from.0 == *nx)
            .cloned()
            .collect()
    }

    /// topological sort using Kahn's algorithm
    /// returns a list of NodeIndices
    pub fn topological_sort(&self) -> Vec<NodeIndex> {
        let mut sorted = vec![];
        let mut working_edges = self.edges.clone();
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

    /// determine if a node has any incoming connections
    fn has_incoming(nx: &NodeIndex, edges: &[(LabeledVertex<S>, LabeledVertex<S>)]) -> bool {
        edges.iter().any(|(_from, to)| to.0 == *nx)
    }

    /// find the index of `edges` corresponding to the first
    /// connection starting from `nx` (if it exists)
    fn next_connected_edge(
        nx: &NodeIndex,
        edges: &[(LabeledVertex<S>, LabeledVertex<S>)],
    ) -> Option<usize> {
        edges.iter().position(|(from, _to)| from.0 == *nx)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn sort() {
        let mut g: Graph<_, &str> = Graph::new();
        let n8 = g.add_node(8);
        let n7 = g.add_node(7);
        let n6 = g.add_node(6);
        let n5 = g.add_node(5);
        let n4 = g.add_node(4);
        let n3 = g.add_node(3);
        let n2 = g.add_node(2);
        let n1 = g.add_node(1);
        g.add_edge((n1, "o1"), (n3, "i3"));
        g.add_edge((n1, "o1"), (n2, "i2"));
        g.add_edge((n3, "o3"), (n4, "i4"));
        g.add_edge((n4, "o4"), (n5, "i5"));
        g.add_edge((n5, "o5"), (n6, "i6"));
        g.add_edge((n6, "o6"), (n7, "i7"));
        g.add_edge((n7, "o7"), (n8, "i8"));
        assert_eq!(g.topological_sort(), vec![7, 6, 5, 4, 3, 2, 1, 0]);
    }

    #[test]
    fn process() {
        let mut g: Graph<_, &str> = Graph::new();
        let n1 = g.add_node(Some(1));
        let n2 = g.add_node(Some(1));
        let n3 = g.add_node(Some(1));
        let n4 = g.add_node(None);
        g.add_edge((n1, "o1"), (n2, "i2"));
        g.add_edge((n1, "o1"), (n3, "i3"));
        g.add_edge((n2, "o2"), (n3, "i3"));
        g.add_edge((n3, "o3"), (n4, "i4"));

        g.topological_sort().iter_mut().for_each(|nx| {
            let parent_sum: u32 = g
                .incoming_edges(nx)
                .into_iter()
                .map(|(from, _to)| g.get_node(from.0).expect("all parents should be filled"))
                .sum();

            g.update_node(*nx, Some(g.get_node(*nx).unwrap_or(0) + parent_sum));
        });
        assert_eq!(*g.get_node(n1), Some(1));
        assert_eq!(*g.get_node(n2), Some(2));
        assert_eq!(*g.get_node(n3), Some(4));
        assert_eq!(*g.get_node(n4), Some(4));
    }
}
