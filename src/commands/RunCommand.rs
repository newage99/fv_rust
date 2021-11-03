use super::super::commands::Command::Command;
use super::super::FVID::FVID;
use super::super::globals::GlobalVariables;
use super::super::Graph::Graph;
use super::super::Topology::Topology;

pub struct RunCommand;

impl Command for RunCommand {
    fn text() -> String {
        return String::from("Run");
    }
    fn run(parameters: Vec<&str>, global_variables: GlobalVariables) {
        
        /*let mut id: &str = "";
        for param in &parameters {
            if param.contains("=") {
                let split = param.split("=").collect::<Vec<&str>>();
                if split.len() > 1 {
                    let identifier = split[0];
                    let value = split[1];
                    if identifier == "id" {
                        id = value;
                    }
                }
            }
        }
        if id == "" {
            println!("'id' parameter not provided.");
            return;
        }
        let response: (String, i128) = FVID::check_str(id, &global_variables);
        if response.0 != "" {
            println!("'{}' check: {}", id, response.0);
            return;
        }
        let fvid: FVID = FVID::create(id, &global_variables);
        let sizes: Vec<i128> = vec![4,6,8,12,16,22,30,31,32,33,50,100];
        for n in sizes {
            let graph: Graph = fvid.compute(n, &global_variables);
            graph.print();
        }
        return;*/

        let fvids: Vec<FVID> = FVID::create_all_for_number_of_symbols(4, &global_variables);
        let mut topologies: Vec<Topology> = Vec::new();
        for fvid in &fvids {
            let fvid_copy: FVID = FVID {
                id: fvid.id.to_vec()
            };
            let new_graph: Graph = fvid_copy.compute(33, &global_variables);
            if new_graph.connected {
                //println!("{}", fvid.to_string());
                let mut c: usize = 0;
                for t in &topologies {
                    let current_graph: &Graph = &t.graphs[0];
                    let current_graph_simple_score: i128 = current_graph.simple_score;
                    let new_graph_simple_score: i128 = new_graph.simple_score;
                    if current_graph_simple_score < new_graph_simple_score ||
                            (current_graph_simple_score == new_graph_simple_score && current_graph.score < new_graph.score) {
                        break;
                    }
                    c += 1;
                }
                let mut new_topology: Topology = Topology {
                    fvid: fvid_copy,
                    graphs: Vec::new()
                };
                new_topology.graphs.push(new_graph);
                topologies.insert(c, new_topology);
            }
        }
        for t in &topologies {
            t.print();
        }
        /*let mut topologies: Vec<Topology> = Vec::new();
        let ns: Vec<i128> = vec![33];
        for fvid in &fvids {
            let fvid_copy: FVID = FVID {
                id: fvid.id.to_vec()
            };
            let mut new_topology: Topology = Topology {
                fvid: fvid_copy,
                graphs: Vec::new()
            };
            //let mut graph_list: Vec<Graph> = Vec::new();
            for i in &ns {
                let graph: Graph = fvid.compute(*i, &global_variables);
                //graph_list.push(graph);
                new_topology.graphs.push(graph);
            }
            topologies.push(new_topology);
        }
        let mut not_connected_topologies: Vec<&Topology> = Vec::new();
        let mut new_ordered_topologies: Vec<&Topology> = Vec::new();
        for topology in &topologies {
            if topology.is_connected() {
                let mut c: usize = 0;
                let topology_total_degree: i128 = topology.total_degree();
                for ordered_topology in &new_ordered_topologies {
                    if topology_total_degree > ordered_topology.total_degree() {
                        break;
                    }
                    c += 1;
                }
                new_ordered_topologies.insert(c, topology);
            } else {
                not_connected_topologies.push(topology);
            }
        }
        Topology::print_topologies(not_connected_topologies);
        Topology::print_topologies(new_ordered_topologies);*/
    }
    fn help() -> String {
        return String::from("Executes algorithm given parameters.");
    }
}