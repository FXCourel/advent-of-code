#![allow(dead_code)]

use super::lib::binary_heap_node::BinaryHeapNode;
use std::{collections::HashMap, fmt, hash::Hash};

#[derive(Debug, Copy, Clone)]
struct Edge<N> {
    start: N,
    destination: N,
    weight: i32,
}

impl<N> Edge<N> {
    fn new(start: N, destination: N, weight: i32) -> Self {
        Self {
            start,
            destination,
            weight,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Graph<N> {
    adjecency_list: HashMap<N, Vec<Edge<N>>>,
}

impl<N> Graph<N>
where
    N: Eq + Hash + Copy,
{
    pub fn new() -> Self {
        Self {
            adjecency_list: HashMap::new(),
        }
    }

    pub fn add_node(&mut self, node: N) {
        self.adjecency_list.entry(node).or_insert(Vec::new());
    }

    pub fn add_edge_directed(&mut self, node1: N, node2: N) {
        self.adjecency_list
            .entry(node1)
            .or_insert(Vec::new())
            .push(Edge::new(node1, node2, 1));
        self.add_node(node2);
    }

    pub fn add_edge_directed_weighted(&mut self, node1: N, node2: N, weight: i32) {
        self.adjecency_list
            .entry(node1)
            .or_insert(Vec::new())
            .push(Edge::new(node1, node2, weight));
        self.add_node(node2);
    }

    pub fn add_edge_undirected(&mut self, node1: N, node2: N) {
        self.adjecency_list
            .entry(node1)
            .or_insert(Vec::new())
            .push(Edge::new(node1, node2, 1));
        self.adjecency_list
            .entry(node2)
            .or_insert(Vec::new())
            .push(Edge::new(node2, node1, 1));
    }

    pub fn add_edge_undirected_weighted(&mut self, node1: N, node2: N, weight: i32) {
        self.adjecency_list
            .entry(node1)
            .or_insert(Vec::new())
            .push(Edge::new(node1, node2, weight));
        self.adjecency_list
            .entry(node2)
            .or_insert(Vec::new())
            .push(Edge::new(node2, node1, weight));
    }

    pub fn get_neighbors(&self, node: N) -> Option<Vec<N>> {
        self.adjecency_list.get(&node).map(|edges| {
            edges
                .iter()
                .map(|edge| edge.destination)
                .collect::<Vec<N>>()
        })
    }

    pub fn iter_neighbors<F>(&self, node: N, mut f: F)
    where
        F: FnMut(N),
    {
        match self.get_neighbors(node) {
            Some(neighbors) => neighbors.iter().for_each(|&neighbor| f(neighbor)),
            None => (),
        }
    }

    fn iter_edges<F>(&self, node: N, mut f: F)
    where
        F: FnMut(Edge<N>),
    {
        match self.adjecency_list.get(&node) {
            Some(edges) => edges.iter().for_each(|&edge| f(edge)),
            None => (),
        }
    }

    pub fn dijsktra<F>(&self, start: N, stop_condition: F) -> Option<(Vec<N>, isize)>
    where
        F: Fn(N) -> bool,
    {
        let mut heap = std::collections::BinaryHeap::new();
        let mut parents = HashMap::new();
        heap.push(BinaryHeapNode::new(Edge::new(start, start, 0), 0));
        while let Some(BinaryHeapNode {
            value: edge,
            priority,
        }) = heap.pop()
        {
            if parents.contains_key(&edge.destination) {
                continue;
            }
            parents.insert(edge.destination, edge.start);
            if stop_condition(edge.destination) {
                let mut path = Vec::from([edge.destination]);
                let mut current = edge.destination;
                while current != start {
                    current = parents[&current];
                    path.push(current);
                }
                return Some((path, -priority as isize));
            }
            self.iter_edges(edge.destination, |edge| {
                if !parents.contains_key(&edge.destination) {
                    heap.push(BinaryHeapNode::new(edge, priority - edge.weight));
                }
            });
        }
        None
    }
}

impl<N> fmt::Display for Graph<N>
where
    N: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let string = self
            .adjecency_list
            .iter()
            .map(|(key, value)| {
                format!(
                    "{} -> {}",
                    key,
                    value
                        .iter()
                        .map(|x| x.destination.to_string())
                        .collect::<Vec<String>>()
                        .join(", ")
                )
            })
            .collect::<Vec<String>>()
            .join("\n");
        write!(f, "{}", string)
    }
}

#[cfg(feature = "test_aoc_lib")]
mod tests {

    #[test]
    fn test_graph() {
        let mut graph = super::Graph::new();
        graph.add_edge_directed(1, 2);
        graph.add_edge_directed(2, 3);
        graph.add_edge_directed(3, 1);
        assert_eq!(graph.get_neighbors(1), Some(vec![2]));
        assert_eq!(graph.get_neighbors(2), Some(vec![3]));
        assert_eq!(graph.get_neighbors(3), Some(vec![1]));
    }

    #[test]
    fn test_dijkstra_no_path() {
        let mut graph = super::Graph::new();
        graph.add_edge_directed_weighted(1, 3, 3);
        graph.add_edge_directed_weighted(1, 2, 1);
        graph.add_edge_directed_weighted(2, 3, 1);
        let result = graph.dijsktra(3, |node| node == 1);
        assert_eq!(result, None);
    }

    #[test]
    fn test_big_dijkstra() {
        let mut graph = super::Graph::new();
        let edges = [(1, 2, 2), (1, 3, 2), (2, 4, 2), (3, 4, 1), (3, 6, 7), (4, 5, 3), (5, 6, 2), (5, 7, 7), (6, 7, 4)];
        edges.iter().for_each(|&(start, destination, weight)| {
            graph.add_edge_directed_weighted(start, destination, weight);
        });
        let result = graph.dijsktra(1, |node| node == 7);
        assert_eq!(result, Some((vec![7, 6, 5, 4, 3, 1], 12)));
    }
}
