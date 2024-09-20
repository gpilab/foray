use std::{cell::RefCell, collections::HashMap, rc::Rc};

pub struct Node<'a> {
    id: &'a str,
    edges: Vec<&'a str>,
    value: RefCell<i32>,
}

pub struct Graph<'a> {
    /// nodeIds to node index in the `nodes` vec
    pub nodes: HashMap<&'a str, Rc<Node<'a>>>,
}
impl Graph<'_> {
    pub fn start_propogate(&self, id: &str, val: i32) {
        self.nodes.get(&id).map(|node| node.update(self, val));
    }
    fn propogate(&self, id: &str, val: i32) {
        let start_node = self.nodes.get(&id);
        let nodes_to_update = start_node.map(|node| &node.edges);
        match nodes_to_update {
            None => (),
            Some(nodes) => {
                nodes
                    .into_iter()
                    .map(|id| {
                        self.nodes.get(id).map(|node| node.update(self, val));
                    })
                    .for_each(drop);
                //start_node.map(|node| node.update(self, val + 1));
            }
        };
    }
    pub fn get_value<'a>(&self, id: &str) -> Option<i32> {
        self.nodes.get(&id).map(|node| *node.value.borrow())
    }
}

impl Node<'_> {
    fn update(&self, links: &Graph, v: i32) {
        println!("Node: {}, updatd to: {}", self.id, v + 1);
        *self.value.borrow_mut() = v + 1;

        links.propogate(self.id, *self.value.borrow())
    }
    pub fn new<'a>(id: &'a str, edges: Vec<&'a str>) -> Rc<Node<'a>> {
        Rc::new(Node {
            id,
            edges,
            value: RefCell::new(0),
        })
    }
}

#[test]
fn prop() {
    let n_a = Node::new("a", vec!["ab", "ac"]);
    let n_ab = Node::new("ab", vec!["abe"]);
    let n_ac = Node::new("ac", vec![]);
    let n_abe = Node::new("abe", vec![]);

    let mut nodes = HashMap::new();

    nodes.insert(n_a.id, n_a);
    nodes.insert(n_ab.id, n_ab);
    nodes.insert(n_ac.id, n_ac);
    nodes.insert(n_abe.id, n_abe);

    let links = Graph { nodes };

    links.start_propogate("a", 0);
    assert_eq!(links.get_value("a"), Some(1));
    assert_eq!(links.get_value("ab"), Some(2));
    assert_eq!(links.get_value("ac"), Some(2));
    assert_eq!(links.get_value("abe"), Some(3));
    links.start_propogate("ab", 7);
    assert_eq!(links.get_value("ab"), Some(8));
    assert_eq!(links.get_value("abe"), Some(9));
}
