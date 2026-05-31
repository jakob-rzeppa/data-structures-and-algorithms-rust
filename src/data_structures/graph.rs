/// A weigthed graph using the matrix representation
#[derive(Default)]
pub struct Graph {
    /// (i,j) i is the row and j is the column and the value represents the weight (or None if no edge)
    matrix: Vec<Vec<Option<u32>>>,
    /// The name of each node
    names: Vec<String>,
}

impl Graph {
    pub fn new(matrix: Vec<Vec<Option<u32>>>, names: Vec<String>) -> Self {
        Graph { matrix, names }
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
        let from_index = self.names
            .iter()
            .position(|n| *n == from)
            .expect("node to exist");
        let to_index = self.names
            .iter()
            .position(|n| *n == to)
            .expect("node to exist");

        self.matrix[from_index][to_index] = Some(weight);
    }

    pub fn remove_node(&mut self, name: &str) {
        let index = self.names
            .iter()
            .position(|n| *n == name)
            .expect("node to exist");

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
}

#[cfg(test)]
mod tests {
    use crate::data_structures::graph::Graph;

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

        graph.remove_edge("A", "C");
        assert_eq!(graph.matrix[0][2], None);
    }
}
