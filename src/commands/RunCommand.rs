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

        let response: Vec<FVID> = FVID::create_all_for_number_of_symbols(3, &global_variables);
        let mut topologies: Vec<Topology> = Vec::new();
        let ns: Vec<i128> = vec![33];
        for fvid in &response {
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
            /*for not_connected_topology in not_connected_topologies {
                ordered_topologies.push(not_connected_topology);
            }
            for ordered_topology in new_ordered_topologies {
                ordered_topologies.push(ordered_topology);
            }*/
            //ordered_topologies.append(&mut not_connected_topologies);
            //ordered_topologies.append(&mut new_ordered_topologies);
        }

        Topology::print_topologies(not_connected_topologies);
        Topology::print_topologies(new_ordered_topologies);
    }
    fn help() -> String {
        return String::from("Executes algorithm given parameters.");
    }
}