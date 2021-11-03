use super::Graph::Graph;
use super::FVID::FVID;

pub struct Topology {
    pub fvid: FVID,
    pub graphs: Vec<Graph>
}

impl Topology {

    pub fn total_degree(&self) -> i128 {

        let mut degree: i128 = 0;

        for graph in &self.graphs {
            degree += graph.degree();
        }

        degree
    }

    pub fn is_connected(&self) -> bool {
        let mut connected: bool = true;
        for graph in &self.graphs {
            if !graph.is_connected() {
                connected = false;
            }
        }
        connected
    }

    pub fn print_topologies(topologies: Vec<&Topology>) {
        println!("");
        //for graph_list in &ordered_topologies_list {
        for topology in topologies {
            let fvid = &topology.fvid;
            let graph_list = &topology.graphs;
            let fvid_str: String = fvid.to_string();
            print!("{}", fvid_str);
            let mut connected: bool = true;
            for graph in graph_list {
                if !graph.is_connected() {
                    connected = false;
                }
            }
            if connected {
                println!("");
                print!("connected = ( ");
                let mut c: usize = 0;
                for graph in graph_list {
                    let mut connected: i128 = 0;
                    if graph.is_connected() {
                        connected = 1;
                    }
                    print!("{}: {}", graph.number_of_nodes(), connected);
                    if c < graph_list.len() - 1 {
                        print!(",  ");
                    }
                    c += 1;
                }
                println!(" )");
                print!("   degree = ( ");
                c = 0;
                for graph in graph_list {
                    if graph.is_connected() {
                        print!("{}: {}", graph.number_of_nodes(), graph.degree());
                    } else {
                        print!("    ");
                    }
                    if c < graph_list.len() - 1 {
                        print!(",  ");
                    }
                    c += 1;
                }
                println!(" )");
                println!("");
            } else {
                println!(" - Not connected");
            }
        }
    }
}