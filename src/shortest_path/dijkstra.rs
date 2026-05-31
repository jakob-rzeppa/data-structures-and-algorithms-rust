use std::collections::HashMap;

use crate::data_structures::{ graph::Graph, priority_queue::PriorityQueue };

#[derive(Debug, Clone)]
struct QueueEntry {
    node: String,
    distance: u32,
}

impl PartialOrd for QueueEntry {
    fn partial_cmp(&self, other: &QueueEntry) -> Option<std::cmp::Ordering> {
        Some(self.distance.cmp(&other.distance))
    }
}

impl PartialEq for QueueEntry {
    fn eq(&self, other: &QueueEntry) -> bool {
        self.node == other.node
    }
}

pub fn dijkstra(graph: &Graph, start: &str) -> HashMap<String, u32> {
    let mut result: HashMap<String, u32> = HashMap::new();

    // Initialize the priority queue with the starting node
    let mut priority_queue = PriorityQueue::<QueueEntry>::new();
    priority_queue.enque(QueueEntry {
        node: start.to_string(),
        distance: 0,
    });

    while !priority_queue.is_empty() {
        let current = priority_queue.deque().expect("queue not empty after checking");
        result.insert(current.node.clone(), current.distance);

        for edge in graph.get_outgoing_edges(&current.node) {
            if result.contains_key(&edge.to) {
                continue;
            }

            let distance = current.distance + edge.weight;

            if let Some(index) = priority_queue.position(|e| e.node == edge.to) {
                let entry = priority_queue.get(index).expect("entry to exist");

                if entry.distance > distance {
                    priority_queue.remove(index);
                    priority_queue.enque(QueueEntry { node: edge.to, distance });
                }
            } else {
                priority_queue.enque(QueueEntry { node: edge.to, distance });
            }
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dijkstra() {
        let mut graph = Graph::default();

        // Add nodes
        graph.add_node("A".to_string());
        graph.add_node("B".to_string());
        graph.add_node("C".to_string());
        graph.add_node("D".to_string());
        graph.add_node("E".to_string());
        graph.add_node("F".to_string());
        graph.add_node("G".to_string());
        graph.add_node("H".to_string());
        graph.add_node("I".to_string());
        graph.add_node("J".to_string());
        graph.add_node("K".to_string());

        // Add edges based on the provided graph diagram
        graph.set_edge("A", "B", 2);
        graph.set_edge("A", "C", 9);
        graph.set_edge("A", "D", 8);
        graph.set_edge("B", "C", 7);
        graph.set_edge("B", "E", 9);
        graph.set_edge("B", "F", 1);
        graph.set_edge("D", "C", 1);
        graph.set_edge("E", "J", 2);
        graph.set_edge("F", "C", 5);
        graph.set_edge("F", "D", 4);
        graph.set_edge("F", "E", 7);
        graph.set_edge("F", "G", 2);
        graph.set_edge("F", "I", 4);
        graph.set_edge("G", "D", 1);
        graph.set_edge("G", "H", 6);
        graph.set_edge("G", "J", 1);
        graph.set_edge("H", "E", 1);
        graph.set_edge("H", "K", 1);
        graph.set_edge("I", "H", 2);
        graph.set_edge("I", "K", 4);
        graph.set_edge("J", "I", 1);
        graph.set_edge("J", "K", 6);

        // Run Dijkstra from node A
        let distances = dijkstra(&graph, "A");

        // Verify shortest distances
        assert_eq!(distances.get("A"), Some(&0));
        assert_eq!(distances.get("B"), Some(&2));
        assert_eq!(distances.get("C"), Some(&7));
        assert_eq!(distances.get("D"), Some(&6));
        assert_eq!(distances.get("E"), Some(&10));
        assert_eq!(distances.get("F"), Some(&3));
        assert_eq!(distances.get("G"), Some(&5));
        assert_eq!(distances.get("H"), Some(&9));
        assert_eq!(distances.get("I"), Some(&7));
        assert_eq!(distances.get("J"), Some(&6));
        assert_eq!(distances.get("K"), Some(&10));
    }
}
