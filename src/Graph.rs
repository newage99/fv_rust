pub struct Graph {
    pub adjacency_matrix: Vec<Vec<i128>>
}

impl Graph {
    pub fn number_of_nodes(&self) -> usize {
        self.adjacency_matrix.len()
    }
    pub fn adjacency_value(&self, x: usize, y: usize) -> i128 {
        if x > y {
            return self.adjacency_matrix[x][y];
        }
        return self.adjacency_matrix[y][x];
    }
    pub fn print(&self) {
        for row in self.adjacency_matrix.iter() {
            for column in row.iter() {
                print!("{} ", column);
            }
            println!("");
        }
    }
}