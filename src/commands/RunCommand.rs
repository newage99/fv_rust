use std::io::{self, Write};
use chrono::Utc;

use super::super::commands::Command::Command;
use super::super::FVID::FVID;
use super::super::globals::GlobalVariables;
use super::super::Graph::Graph;
use super::super::Topology::Topology;
use super::super::symbols::Symbol::Symbol;

use super::super::symbols::Function::Subtraction::Subtraction;
use super::super::symbols::Function::Addition::Addition;
use super::super::symbols::Variable::NumberOfNodes::NumberOfNodes;
use super::super::symbols::Variable::One::One;
use super::super::symbols::Variable::X::X;

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

        //let fvids: Vec<FVID> = FVID::create_all_for_number_of_symbols(9, &global_variables);
        let mut topologies: Vec<Topology> = Vec::new();
        let mut last_simple_score: i128 = -1;
        let mut last_score: i128 = -1;
        let mut current_first_symbols: Vec<&str> = Vec::new();
        let current_number_of_first_symbols = 2;
        for i in 0..current_number_of_first_symbols {
            current_first_symbols.push("");
        }
        let mut not_finished = true;
        let mut c = 0;
        //let mut fvid_symbols_list: Vec<&dyn Symbol> = vec![&Subtraction, &NumberOfNodes, &Addition, &Addition, &Addition, &Addition, &Addition, &Addition, &Addition, &Addition];
        let mut fvid_symbols_list: Vec<&dyn Symbol> = Vec::new();
        for i in 0..9 {
            fvid_symbols_list.push(global_variables.symbols_list[0]);
        }
        let mut n_x_and_y_not_exists = 0;
        let mut n_not_connected = 0;
        let mut n_connected = 0;
        //let mut check_symbols_list_sum = 0;
        let mut create_matrix_sum = 0;
        let mut compute_function_sum = 0;
        let mut compute_variable_sum = 0;
        let mut compute_sum = 0;
        let mut number_of_fvids_with_best_score = 0;

        while not_finished {

            //let check_symbols_list_start_time = Utc::now().time();
            let check_response: (String, i128) = FVID::check_symbols_list(&fvid_symbols_list, &global_variables);
            //let check_symbols_list_end_time = Utc::now().time();
            //let check_symbols_list_diff = check_symbols_list_end_time - check_symbols_list_start_time;
            //check_symbols_list_sum += check_symbols_list_diff.num_milliseconds();
            
            c += 1;
            if c == 40000 {
                c = 0;
                print!(".");
                io::stdout().flush().unwrap();
            }

            if check_response.0 == "" {  
                let mut x_or_y_exists: bool = false;
                for fvid_symbol in &fvid_symbols_list {
                    let symbol_str: &str = fvid_symbol.symbol();
                    if symbol_str == "x" || symbol_str == "y" {
                        x_or_y_exists = true;
                    }
                }
                if x_or_y_exists {
                    let fvid: FVID = FVID {
                        id: fvid_symbols_list.to_vec()
                    };
                    for i in 0..current_number_of_first_symbols {
                        if (&fvid.id[i]).symbol() != current_first_symbols[i]  {
                            println!("");
                            print!("Current first symbols: ");
                            for j in 0..current_number_of_first_symbols {
                                current_first_symbols[j] = (&fvid.id[j]).symbol();
                                print!("{} ", current_first_symbols[j]);
                            }
                            println!("");
                            break;
                        }
                    }
                    let fvid_copy: FVID = FVID {
                        id: fvid.id.to_vec()
                    };
                    
                    //let compute_start_time = Utc::now().time();
                    
                    //let new_graph: Graph = fvid_copy.compute(33, &global_variables);
                    let result = fvid_copy.compute(33, &global_variables);
                    let new_graph = result.0;
                    create_matrix_sum += result.1;
                    compute_sum += result.2;
                    compute_function_sum += result.3;
                    compute_variable_sum += result.4;

                    if new_graph.connected {
                        n_connected += 1;
                        c = 0;
                        if last_simple_score < 0 || new_graph.simple_score < last_simple_score || (new_graph.simple_score == last_simple_score && new_graph.score < last_score) {
                            let mut new_topology: Topology = Topology {
                                fvid: fvid_copy,
                                graphs: Vec::new()
                            };
                            last_simple_score = new_graph.simple_score;
                            last_score = new_graph.score;
                            new_topology.graphs.push(new_graph);
                            println!("");
                            new_topology.print();
                            topologies.insert(c, new_topology);
                            number_of_fvids_with_best_score = 1;
                        } else if new_graph.simple_score == last_simple_score && new_graph.score == last_score {
                            print!("=");
                            number_of_fvids_with_best_score += 1;
                        }
                    } else {
                        n_not_connected += 1;
                    }
                } else {
                    n_x_and_y_not_exists += 1;
                }
            }
            for i in (0..fvid_symbols_list.len()).rev() {
                let symbol: &dyn Symbol = fvid_symbols_list[i];
                if symbol.symbol() != global_variables.symbols_list[global_variables.symbols_list.len() - 1].symbol() {
                    fvid_symbols_list[i] = FVID::get_next_symbol(symbol, &global_variables);
                    break;
                } else if i > 0 {
                    fvid_symbols_list[i] = global_variables.symbols_list[0];
                } else {
                    not_finished = false;
                }
            }
        }

        println!("");
        println!("number_of_fvids_with_best_score: {}", number_of_fvids_with_best_score);
        /*println!("");
        println!("n_x_and_y_not_exists: {}", n_x_and_y_not_exists);
        println!("n_not_connected: {}", n_not_connected);
        println!("n_connected: {}", n_connected);
        println!("check_symbols_list_sum: {}", check_symbols_list_sum);
        println!("create_matrix_sum: {}", create_matrix_sum / 1000);
        println!("compute_function_sum: {}", compute_function_sum / 1000);
        println!("compute_variable_sum: {}", compute_variable_sum / 1000);
        println!("compute_sum: {}", compute_sum / 1000);*/

        /*for t in &topologies {
            t.print();
        }*/

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