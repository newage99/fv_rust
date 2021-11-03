pub struct Graph {
    pub adjacency_matrix: Vec<Vec<i128>>
}

impl Graph {

    pub fn number_of_nodes(&self) -> usize {
        self.adjacency_matrix.len() + 1
    }

    pub fn adjacency_value(&self, x: usize, y: usize) -> i128 {
        //println!("adjacency_value: {} {}", x, y);
        if x == y {
            return 0;
        }
        if x > y {
            return self.adjacency_matrix[y][x-y];
        }
        return self.adjacency_matrix[x][y-x];
    }

    pub fn is_connected(&self) -> bool {
        let mut starting_node: i128 = -1;
        let mut row_counter: i128 = 0;
        'outer: for row in self.adjacency_matrix.iter() {
            for adjacency_value in row.iter() {
                if adjacency_value > &0 {
                    starting_node = row_counter;
                    break 'outer;
                }
            }
            row_counter += 1;
        }
        //println!("Starting node: {}", starting_node);
        if starting_node < 0 {
            return false;
        }
        let mut neighbours_to_visit: Vec<i128> = vec![starting_node];
        let mut visitied_neighbours_counter: usize = 0;
        while visitied_neighbours_counter < neighbours_to_visit.len() && neighbours_to_visit.len() < (self.number_of_nodes() - 1) {
            let main_node_to_find_neighbours_from: i128 = neighbours_to_visit[visitied_neighbours_counter];
            //println!("Main node: {}", main_node_to_find_neighbours_from);
            for i in 0..(self.number_of_nodes() - 1) {
                let i_i128 = i as i128;
                let is_not_main_node_itself: bool = i_i128 != main_node_to_find_neighbours_from;
                if is_not_main_node_itself {
                    let is_this_node_connected_to_main_node: bool = self.adjacency_value(i, main_node_to_find_neighbours_from as usize) > 0;
                    let is_this_node_not_already_a_neighbour: bool = !neighbours_to_visit.contains(&i_i128);
                    if is_this_node_connected_to_main_node && is_this_node_not_already_a_neighbour {
                        //println!("Appending neighbour: {}", i_i128);
                        neighbours_to_visit.push(i_i128);
                    }
                }
            }
            visitied_neighbours_counter += 1;
        }
        return neighbours_to_visit.len() == (self.number_of_nodes() - 1);
    }

    pub fn degree(&self) -> i128 {
        let mut max_degree: i128 = 0;
        let mut current_degree: i128;
        for row in self.adjacency_matrix.iter() {
            current_degree = 0;
            for adjacency_value in row.iter() {
                if *adjacency_value > 0 {
                    current_degree += 1;
                }
            }
            if current_degree > max_degree {
                max_degree = current_degree;
            }
        }
        max_degree
    }

    pub fn print(&self) {
        for row in self.adjacency_matrix.iter() {
            for adjacency_value in row.iter() {
                print!("{} ", adjacency_value);
            }
            println!("");
        }
        println!("Is connected: {}", self.is_connected());
    }
}