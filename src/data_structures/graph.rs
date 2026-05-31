/// A weigthed graph using the matrix representation
#[derive(Default)]
pub struct Graph {
    /// (i,j) i is the row and j is the column and the value represents the weight (or None if no edge)
    matrix: Vec<Vec<Option<u32>>>,
    /// The name of each node
    names: Vec<String>,
}

#[derive(Debug, PartialEq)]
pub struct Edge {
    pub from: String,
    pub to: String,
    pub weight: u32,
}

impl Graph {
    pub fn new(matrix: Vec<Vec<Option<u32>>>, names: Vec<String>) -> Self {
        Graph { matrix, names }
    }

    fn index_of(&self, name: &str) -> usize {
        self.names
            .iter()
            .position(|n| *n == name)
            .expect("node to exist")
    }

    pub fn add_node(&mut self, name: String) {
        self.names.push(name);

        // Increase the rows by 1 with no edges
        let mut row = Vec::new();
        for _ in 0..self.matrix.len() {
            row.push(None);
        }

        self.matrix.push(row);

        // Increase the columns by 1 with no edges
        self.matrix.iter_mut().for_each(|r| r.push(None));
    }

    pub fn set_edge(&mut self, from: &str, to: &str, weight: u32) {
        let from_index = self.index_of(from);
        let to_index = self.index_of(to);

        self.matrix[from_index][to_index] = Some(weight);
    }

    pub fn remove_node(&mut self, node: &str) {
        let index = self.index_of(node);

        self.names.remove(index);

        // Remove row
        self.matrix.remove(index);

        // Remove column
        self.matrix.iter_mut().for_each(|row| {
            row.remove(index);
        });
    }

    pub fn remove_edge(&mut self, from: &str, to: &str) {
        let from_index = self.names
            .iter()
            .position(|n| *n == from)
            .expect("node to exist");
        let to_index = self.names
            .iter()
            .position(|n| *n == to)
            .expect("node to exist");

        self.matrix[from_index][to_index] = None;
    }

    pub fn get_outgoing_edges(&self, node: &str) -> Vec<Edge> {
        let mut edges = Vec::new();

        let index = self.index_of(node);

        self.matrix[index]
            .iter()
            .enumerate()
            .for_each(|(i, n)| {
                if let Some(weight) = n {
                    edges.push(Edge {
                        from: node.to_string(),
                        to: self.names[i].clone(),
                        weight: *weight,
                    });
                }
            });

        edges
    }
}

#[cfg(test)]
mod tests {
    use crate::data_structures::graph::{ Edge, Graph };

    #[test]
    fn test_graph() {
        let mut graph = Graph::default();

        graph.add_node("A".to_string());
        graph.add_node("B".to_string());
        graph.add_node("C".to_string());

        graph.set_edge("A", "B", 1);
        graph.set_edge("B", "C", 2);
        graph.set_edge("A", "C", 3);

        assert_eq!(graph.matrix[0][1], Some(1));
        assert_eq!(graph.matrix[1][2], Some(2));
        assert_eq!(graph.matrix[0][2], Some(3));

        assert_eq!(
            graph.get_outgoing_edges("A"),
            vec![
                Edge { from: "A".into(), to: "B".into(), weight: 1 },
                Edge { from: "A".into(), to: "C".into(), weight: 3 }
            ]
        );

        graph.remove_edge("A", "C");
        assert_eq!(graph.matrix[0][2], None);
    }
}
