/*
	graph
	This problem requires you to implement a basic graph function
*/

use std::collections::{HashMap, HashSet};
use std::fmt;

#[derive(Debug, Clone)]
pub struct NodeNotInGraph;

impl fmt::Display for NodeNotInGraph {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "accessing a node that is not in the graph")
    }
}

pub trait Graph {
    fn new() -> Self;
    fn adjacency_table_mutable(&mut self) -> &mut HashMap<String, Vec<(String, i32)>>;
    fn adjacency_table(&self) -> &HashMap<String, Vec<(String, i32)>>;

    fn add_node(&mut self, node: &str) -> bool {
        let table = self.adjacency_table_mutable();
        if table.contains_key(node) {
            false // already exists
        } else {
            table.insert(node.to_string(), Vec::new());
            true
        }
    }

    fn add_edge(&mut self, edge: (&str, &str, i32)) {
        let (from, to, weight) = edge;
        // Ensure both nodes exist
        self.add_node(from);
        self.add_node(to);

        let table = self.adjacency_table_mutable();
        // Add edge from -> to
        table.get_mut(from).unwrap().push((to.to_string(), weight));
        // Add edge to -> from (because undirected)
        table.get_mut(to).unwrap().push((from.to_string(), weight));
    }

    fn contains(&self, node: &str) -> bool {
        self.adjacency_table().contains_key(node)
    }

    fn nodes(&self) -> HashSet<&String> {
        self.adjacency_table().keys().collect()
    }

    fn edges(&self) -> Vec<(&String, &String, i32)> {
        let mut edges = Vec::new();
        for (from_node, from_node_neighbours) in self.adjacency_table() {
            for (to_node, weight) in from_node_neighbours {
                edges.push((from_node, to_node, *weight));
            }
        }
        edges
    }
}

pub struct UndirectedGraph {
    adjacency_table: HashMap<String, Vec<(String, i32)>>,
}

impl Graph for UndirectedGraph {
    fn new() -> UndirectedGraph {
        UndirectedGraph {
            adjacency_table: HashMap::new(),
        }
    }

    fn adjacency_table_mutable(&mut self) -> &mut HashMap<String, Vec<(String, i32)>> {
        &mut self.adjacency_table
    }

    fn adjacency_table(&self) -> &HashMap<String, Vec<(String, i32)>> {
        &self.adjacency_table
    }
}

#[cfg(test)]
mod test_undirected_graph {
    use super::{Graph, UndirectedGraph};
    use std::collections::HashSet; // 👈👈 关键：在测试模块内导入 HashSet

    #[test]
    fn test_add_edge() {
        let mut graph = UndirectedGraph::new();
        graph.add_edge(("a", "b", 5));
        graph.add_edge(("b", "c", 10));
        graph.add_edge(("c", "a", 7));

        let edges = graph.edges();
        let expected = vec![
            ("a", "b", 5),
            ("a", "c", 7),
            ("b", "a", 5),
            ("b", "c", 10),
            ("c", "b", 10),
            ("c", "a", 7),
        ];

        for &(from, to, weight) in &expected {
            assert!(
                edges.contains(&(&from.to_string(), &to.to_string(), weight)),
                "Missing edge: {} -> {} ({})",
                from,
                to,
                weight
            );
        }
        assert_eq!(edges.len(), expected.len());
    }

    #[test]
    fn test_add_node() {
        let mut graph = UndirectedGraph::new();
        assert!(graph.add_node("x"));
        assert!(!graph.add_node("x")); // already exists
        assert!(graph.contains("x"));
        assert!(!graph.contains("y"));
    }

    #[test]
    fn test_nodes() {
        let mut graph = UndirectedGraph::new();
        graph.add_edge(("p", "q", 1));
        let nodes: HashSet<&str> = graph.nodes().into_iter().map(|s| s.as_str()).collect();
        assert_eq!(nodes, ["p", "q"].iter().copied().collect::<HashSet<_>>());
    }
}