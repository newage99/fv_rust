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
        let graph: Graph = fvid.compute(4, &global_variables);
        graph.print();*/
        let response: Vec<FVID> = FVID::create_all_for_number_of_symbols(3, &global_variables);
        let mut topologies_list: Vec<Topology> = Vec::new();
        let mut ordered_topologies_list: Vec<Topology> = Vec::new();
        let ns: Vec<i128> = vec![3, 4, 5, 6, 7, 8, 9, 10];
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
            topologies_list.push(new_topology);
        }
        /*for graph_list in &topologies_list {
            let not_connected_graph_lists: Vec<Vec<Graph>> = Vec::new();
            let ordered_topologies_tuple_list: Vec<(i128, Vec<Graph>)> = Vec::new();

        }*/
        println!("");
        //for graph_list in &ordered_topologies_list {
        for topology in &topologies_list {
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
                    if c < ns.len() - 1 {
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
                    if c < ns.len() - 1 {
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
    fn help() -> String {
        return String::from("Executes algorithm given parameters.");
    }
}