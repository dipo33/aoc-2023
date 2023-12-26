use std::collections::HashMap;

#[derive(Debug, Copy, Clone)]
pub enum Direction {
    Left,
    Right,
}

pub type NodeId = u32;

pub struct Graph {
    pub edges: HashMap<NodeId, (NodeId, NodeId)>,
}

pub struct LabeledMap {
    pub graph: Graph,
    pub directions: Vec<Direction>,
}

impl Graph {
    pub fn new() -> Self {
        Graph {
            edges: HashMap::new(),
        }
    }

    pub fn add_edge(&mut self, source: NodeId, dest_a: NodeId, dest_b: NodeId) {
        self.edges.insert(source, (dest_a, dest_b));
    }

    pub fn label_to_node_id(label: &str) -> u32 {
        assert_eq!(label.len(), 3, "Label must be exactly 3 characters long");

        label.chars().enumerate().fold(0, |acc, (i, c)| {
            acc + (c as u32 * 256u32.pow(i as u32))
        })
    }
}
