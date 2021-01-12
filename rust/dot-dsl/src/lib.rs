// Adaptation of nilsso's solution: https://exercism.io/tracks/rust/exercises/dot-dsl/solutions/caa9a67e77294429bfdd430eb8d7d052
// and runiq's solution: https://exercism.io/tracks/rust/exercises/dot-dsl/solutions/7538eb532e23413ca9dea828d60ddc6d
// Following the builder pattern: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html

// Attrs trait will be implemented via derive_proc_macro called Attrs (used via derive attribute on structs):
use dot_dsl_derive::Attrs;
pub trait Attrs {
    fn with_attrs(self, attrs: &[(&str, &str)]) -> Self;
    fn get_attr(&self, key: &str) -> Option<&str>;
}

pub mod graph {
    use graph_items::edge::Edge;
    use graph_items::node::Node;
    use std::collections::HashMap;

    use super::Attrs; // seems to import the Attrs Trait as well as the Attrs macro, since the use statement of the derive macro is at the same level.

    #[derive(Attrs)]
    pub struct Graph {
        pub nodes: Vec<Node>,
        pub edges: Vec<Edge>,
        pub attrs: HashMap<String, String>,
    }

    impl Graph {
        pub fn new() -> Self {
            Self {
                nodes: Vec::new(),
                edges: Vec::new(),
                attrs: HashMap::new(),
            }
        }

        pub fn with_nodes(self, nodes: &[Node]) -> Self {
            Self {
                nodes: nodes.into(),
                ..self
            }
        }

        pub fn with_edges(self, edges: &[Edge]) -> Self {
            Self {
                edges: edges.into(),
                ..self
            }
        }

        pub fn get_node(&self, name: &str) -> Option<&Node> {
            self.nodes.iter().find(|node| node.name == name)
        }
    }

    pub mod graph_items {
        pub mod node {
            use std::collections::HashMap;

            use super::super::super::Attrs;

            #[derive(Attrs, Clone, PartialEq, Debug)]
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
            }
        }

        pub mod edge {
            use std::collections::HashMap;

            use super::super::super::Attrs;

            #[derive(Attrs, Clone, PartialEq, Debug)]
            pub struct Edge {
                pub from: String,
                pub to: String,
                pub attrs: HashMap<String, String>,
            }

            impl Edge {
                pub fn new(from: &str, to: &str) -> Self {
                    Self {
                        from: from.to_string(),
                        to: to.to_string(),
                        attrs: HashMap::new(),
                    }
                }
            }
        }
    }
}
