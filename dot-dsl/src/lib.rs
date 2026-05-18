pub mod graph {
    use std::collections::HashMap;
    use crate::graph::graph_items::{node::Node, edge::Edge};

    pub struct Graph {
        pub nodes: Vec<Node>,
        pub edges: Vec<Edge>,
        pub attrs: HashMap<String, String>
    }

    impl Graph {
        pub fn new() -> Self {
            Graph { nodes: Vec::new(), edges: Vec::new(), attrs: HashMap::new() }
        }

        pub fn with_nodes(mut self, nodes: &[Node]) -> Self {
            self.nodes.extend(nodes.to_vec());
            self
        }

        pub fn with_edges(mut self, edges: &[Edge]) -> Self {
            self.edges.extend(edges.to_vec());
            self
        }

        pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
            self.attrs
                .extend(attrs.iter().map(|&(key, value)| (key.into(), value.into())));
            self
        }
        
        pub fn node(&self, key: &str) -> Option<&Node> {
            self.nodes.iter().find(|&node| node.name == key)
        }
    }

    pub mod graph_items;
}
