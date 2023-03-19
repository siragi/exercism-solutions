pub mod graph {
    use graph_items::edge::Edge;
    use graph_items::node::Node;
    use std::collections::HashMap;

    pub struct Graph {
        pub nodes: Vec<Node>,
        pub edges: Vec<Edge>,
        pub attrs: HashMap<String, String>,
    }

    impl Graph {
        pub fn new() -> Self {
            // Construct a new Graph struct."
            Self {
                nodes: Vec::new(),
                edges: Vec::new(),
                attrs: HashMap::new(),
            }
        }

        pub fn with_nodes(mut self, nodes: &Vec<Node>) -> Self {
            self.nodes = nodes.clone();
            self
        }

        pub fn with_edges(mut self, edges: &Vec<Edge>) -> Self {
            self.edges = edges.clone();
            self
        }

        pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
            self.attrs = attrs
                .iter()
                .map(|(a, b)| (a.to_string(), b.to_string()))
                .collect();
            self
        }

        pub fn get_node(&self, name: &str) -> Option<&Node> {
            self.nodes.iter().find(|node| node.name == name)
        }
    }

    pub mod graph_items {
        pub mod node {
            use std::collections::HashMap;
            #[derive(Clone, PartialEq, Debug)]
            pub struct Node {
                pub name: String,
                pub attrs: HashMap<String, String>,
            }

            impl Node {
                pub fn new(name: &str) -> Self {
                    Self {
                        name: String::from(name),
                        attrs: HashMap::new(),
                    }
                }

                pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
                    self.attrs = attrs
                        .iter()
                        .map(|(a, b)| (a.to_string(), b.to_string()))
                        .collect();
                    self
                }

                pub fn get_attr(&self, attr: &str) -> Option<&str> {
                    self.attrs.get(attr).map(|value| value.as_str())
                }
            }
        }

        pub mod edge {
            use std::collections::HashMap;
            #[derive(Clone, PartialEq, Debug)]
            pub struct Edge {
                pub leftnode: String,
                pub rightnode: String,
                pub attrs: HashMap<String, String>,
            }

            impl Edge {
                pub fn new(leftnode: &str, rightnode: &str) -> Self {
                    Self {
                        leftnode: leftnode.to_string(),
                        rightnode: rightnode.to_string(),
                        attrs: HashMap::new(),
                    }
                }

                pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
                    self.attrs = attrs
                        .iter()
                        .map(|(a, b)| (a.to_string(), b.to_string()))
                        .collect();
                    self
                }
            }
        }
    }
}
