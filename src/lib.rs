//! Dependency resolver.
//! We use a DFS algoritm for tree traversal and dependency cycle detection.
//! In order to keep it simple, there is no complicated linked list with 
//! `smart` pointers involved, and we will detect cycles based on the id of
//! each node in our graph.
use std::collections::HashSet;

#[allow(dead_code)]
// This type help us to keep track of the vertices
// we visit during cycle detection.
type NodeIdTracker = HashSet<String>;

#[derive(Debug, Clone)]
pub struct Node {
    pub id: String,
    pub deps: Vec<Node>,
}

impl Node {
    // Return the list of all the dependencies to build.
    pub fn get_dependancy_list(&self) -> Vec<String> {
        if let Some(dependancy_list) = self.walk() {
            dependancy_list
        } else {
            Vec::new()
        }
    }

    // Walk through the graph using DFS algorithm.
    fn walk(&self) -> Option<Vec<String>> {
        if self.deps.is_empty() {
            None
        } else {
            let mut stack: Vec<String> = vec![];
            for dep in &self.deps {
                if let Some(dep) = dep.walk() {
                    stack.append(&mut dep.clone());
                }
                stack.push(dep.id.clone());
            }
            Some(stack)
        }
    }

    // Detect if a graph is a DAG or not.
    #[allow(dead_code)]
    fn has_cycle(&self) -> bool {
        let mut visited = NodeIdTracker::new();

        if let Some(v) = self.detect_cycles(&mut visited) {
            v
        } else {
            false
        }
    }

    // Walk through the graph and stopping at the first cycle it encounters.
    #[allow(dead_code)]
    fn detect_cycles(&self, visited: &mut NodeIdTracker) -> Option<bool> {
        // We already visited this node, meaning we encountered a cycle
        // in the graph.
        if visited.contains(&self.id) {
            return Some(true);
        }

        // Keeping track of the nodes we are visiting.
        visited.insert(self.id.clone());

        if self.deps.is_empty() {
            return None;
        } else {
            for dep in &self.deps {
                if let Some(true) = dep.detect_cycles(visited) {
                    return Some(true);
                }
            }
        }
        Some(false)
    }
}

#[cfg(test)]
mod test_super {
    use crate::*;

    /// Create a dependency graph without cycle.
    ///              (Mylib)
    ///        /          \        \
    ///    (a)            (b)      (c)
    ///  /    \        /   \   \     \
    ///(aa)  (ab)    /      \   \    (ca)
    ///           (ba)   (bb)  (bc)
    ///            /        \
    ///         (baa)      (bba)
    fn mock_dag() -> Node {
        Node {
            id: "MyLib".to_string(),
            deps: vec![
                Node {
                    id: "a".into(),
                    deps: vec![
                        Node {
                            id: "aa".into(),
                            deps: vec![],
                        },
                        Node {
                            id: "ab".into(),
                            deps: vec![],
                        },
                    ],
                },
                Node {
                    id: "b".into(),
                    deps: vec![
                        Node {
                            id: "ba".into(),
                            deps: vec![Node {
                                id: "baa".into(),
                                deps: vec![],
                            }],
                        },
                        Node {
                            id: "bb".into(),
                            deps: vec![Node {
                                id: "bba".into(),
                                deps: vec![],
                            }],
                        },
                        Node {
                            id: "bc".into(),
                            deps: vec![],
                        },
                    ],
                },
                Node {
                    id: "c".into(),
                    deps: vec![Node {
                        id: "ca".into(),
                        deps: vec![],
                    }],
                },
            ],
        }
    }

    /// Create a dependency graph WITH cycle that
    /// cannot be resolved.
    ///              (Mylib)
    ///        /          \        \
    ///    (a)            (b)      (c)
    ///  /    \        /   \   \     \
    ///(aa)  (ab)    /      \   \    (ca)
    ///           (ba)   (bb)  (bc)
    ///            /        \
    ///         (baa)      (bba)
    ///                       \
    ///                       (b) {Cycling here.}
    fn mock_cycle() -> Node {
        Node {
            id: "MyLib".to_string(),
            deps: vec![
                Node {
                    id: "a".into(),
                    deps: vec![
                        Node {
                            id: "aa".into(),
                            deps: vec![],
                        },
                        Node {
                            id: "ab".into(),
                            deps: vec![],
                        },
                    ],
                },
                Node {
                    id: "b".into(),
                    deps: vec![
                        Node {
                            id: "ba".into(),
                            deps: vec![Node {
                                id: "baa".into(),
                                deps: vec![],
                            }],
                        },
                        Node {
                            id: "bb".into(),
                            deps: vec![Node {
                                id: "bba".into(),
                                deps: vec![Node {
                                    // This is where we introduce a cycle!!
                                    id: "b".into(),
                                    deps: vec![],
                                }],
                            }],
                        },
                        Node {
                            id: "bc".into(),
                            deps: vec![],
                        },
                    ],
                },
                Node {
                    id: "c".into(),
                    deps: vec![Node {
                        id: "ca".into(),
                        deps: vec![],
                    }],
                },
            ],
        }
    }

    #[test]
    fn test_dag() {
        let graph = mock_dag();
        let dependancy_list = graph.get_dependancy_list();

        println!("{:?}", dependancy_list);

        assert_eq!(
            dependancy_list,
            vec!["aa", "ab", "a", "baa", "ba", "bba", "bb", "bc", "b", "ca", "c"]
        );
    }

    #[test]
    fn test_detect_cycle() {
        let graph = mock_dag();
        assert_eq!(graph.has_cycle(), false);

        let graph = mock_cycle();
        assert_eq!(graph.has_cycle(), true);
    }
}
