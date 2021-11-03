use num_integer::Integer;

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
            return self.adjacency_matrix[y][(x-y)-1];
        }
        return self.adjacency_matrix[x][(y-x)-1];
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
        while visitied_neighbours_counter < neighbours_to_visit.len() && neighbours_to_visit.len() < self.number_of_nodes() {
            let main_node_to_find_neighbours_from: i128 = neighbours_to_visit[visitied_neighbours_counter];
            //println!("Main node: {}", main_node_to_find_neighbours_from);
            for i in 0..self.number_of_nodes() {
                let i_i128 = i as i128;
                let is_not_main_node_itself: bool = i_i128 != main_node_to_find_neighbours_from;
                if is_not_main_node_itself {
                    let is_this_node_connected_to_main_node: bool = self.adjacency_value(i, main_node_to_find_neighbours_from as usize) > 0;
                    let is_this_node_not_already_a_neighbour: bool = !neighbours_to_visit.contains(&i_i128);
                    //println!("is_this_node_connected_to_main_node({}, {}): {}", i, main_node_to_find_neighbours_from, is_this_node_connected_to_main_node);
                    if is_this_node_connected_to_main_node && is_this_node_not_already_a_neighbour {
                        //println!("Appending neighbour: {}", i_i128);
                        neighbours_to_visit.push(i_i128);
                    }
                }
            }
            visitied_neighbours_counter += 1;
        }
        return neighbours_to_visit.len() == self.number_of_nodes();
    }

    pub fn degree(&self) -> i128 {

        let mut max_degree: i128 = 0;
        let mut current_degree: i128;

        for x in 0..self.number_of_nodes() {
            current_degree = 0;
            for y in 0..self.number_of_nodes() {
                if x != y && self.adjacency_value(x, y) > 0 {
                    current_degree += 1;
                }
            }
            if current_degree > max_degree {
                max_degree = current_degree;
            }
        }
        max_degree
    }

    pub fn degree_score(&self) -> i128 {

        let mut degree: i128 = 0;
        let mut current_degree: i128;
        let number_of_nodes: usize = self.number_of_nodes();
        let mut degrees: Vec<i128> = Vec::new();

        for x in 0..number_of_nodes {
            current_degree = 0;
            for y in 0..number_of_nodes {
                if x != y && self.adjacency_value(x, y) > 0 {
                    current_degree += 1;
                }
            }
            degrees.push(current_degree);
        }

        for x in 0..(self.number_of_nodes() - 1) {
            let x_i128 = x as i128;
            for y in (x + 1)..self.number_of_nodes() {
                let y_i128 = y as i128;
                degree += (x_i128 + y_i128).div_ceil(&2);
            }
        }

        degree
    }

    pub fn diameter(&self) -> i128 {

        let diameters: Vec<i128> = self.diameters_list();
        let mut max_diameter: i128 = 0;

        for d in &diameters {
            if *d > max_diameter {
                max_diameter = *d;
            }
        }

        max_diameter
    }

    pub fn diameter_score(&self) -> i128 {

        let diameters: Vec<i128> = self.diameters_list();
        //println!("Number of diameters: {}", diameters.len());
        let mut score: i128 = 0;

        for d in &diameters {
            score += *d;
        }

        score
    }

    fn dijkstra(&self, x: usize, y: usize) -> i128 {

        if !self.is_connected() {
            return 0;
        }

        let mut to_visit_nodes: Vec<i128> = vec![x as i128];
        let mut visitied_neighbours_counter: usize = 0;
        let mut dijkstra_counter: i128 = 0;
        let number_of_nodes: usize = self.number_of_nodes();

        while to_visit_nodes.len() < number_of_nodes {

            dijkstra_counter += 1;
            let neighbours_that_we_will_visit: usize = to_visit_nodes.len() - visitied_neighbours_counter;

            for i in visitied_neighbours_counter..to_visit_nodes.len() {

                let main_node_to_find_neighbours_from: i128 = to_visit_nodes[i];
                let main_node_usize: usize = main_node_to_find_neighbours_from as usize;
    
                for j in 0..self.number_of_nodes() {
    
                    let j_i128: i128 = j as i128;
    
                    if j_i128 != main_node_to_find_neighbours_from && self.adjacency_value(j, main_node_usize) > 0 && !to_visit_nodes.contains(&j_i128) {
                        if j == y {
                            return dijkstra_counter;                            
                        }
                        to_visit_nodes.push(j_i128);
                    }
                }
            }

            visitied_neighbours_counter += neighbours_that_we_will_visit;
        }

        0 // This means 'y' has not been found, ergo, the graph is not connected
    }

    fn diameters_list(&self) -> Vec<i128> {

        let mut diameters: Vec<i128> = Vec::new();

        for x in 0..(self.number_of_nodes() - 1) {
            for y in (x + 1)..self.number_of_nodes() {
                let dijkstra: i128 = self.dijkstra(x, y);
                if dijkstra <= 0 {
                    return Vec::new();
                }
                diameters.push(dijkstra);
            }
        }
        diameters
    }

    pub fn print(&self) {

        let number_of_nodes: usize = self.number_of_nodes();
        /*for x in 0..number_of_nodes {
            for y in 0..number_of_nodes {
                print!("{} ", self.adjacency_value(x, y));
            }
            println!("");
        }*/

        print!("{}", number_of_nodes);
        for i in 0..(3 - number_of_nodes.to_string().len()) {
            print!(" ");
        }
        println!(" -> Degree: {} | Degree score: {} | Diameter: {} | Diameter score: {}", self.degree(), self.degree_score(), self.diameter(), self.diameter_score());

        /*println!("Is connected: {}", self.is_connected());
        println!("Degree: {}", self.degree());
        println!("Degree score: {}", self.degree_score());
        println!("Diameter: {}", self.diameter());
        println!("Diameter score: {}", self.diameter_score());*/
    }
}