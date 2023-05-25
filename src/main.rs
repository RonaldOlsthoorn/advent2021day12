use std::{collections::{HashMap, VecDeque}, io::{BufReader, BufRead}, fs::File};


#[derive(Clone, Debug)]
struct WalkState {
    path: Vec<String>,
    hit: bool
}

struct Graph {

    adjacency_list: HashMap<String, Vec<String>>
}

impl Graph {

    fn from_input(input: &Vec<String>) -> Self {

        let mut adjacency_list: HashMap<String, Vec<String>> = HashMap::new();

        for line in input.iter() {

            let nodes: Vec<String> = line.split('-').map(|p| p.to_string()).collect();

            if let Some(list) = adjacency_list.get_mut(&nodes[0]) {
                list.push(nodes[1].clone());
            } else {
                adjacency_list.insert(nodes[0].clone(), vec![nodes[1].clone()]);
            }

            if let Some(list) = adjacency_list.get_mut(&nodes[1]) {
                list.push(nodes[0].clone());
            } else {
                adjacency_list.insert(nodes[1].clone(), vec![nodes[0].clone()]);
            }
        }

        Self{adjacency_list}
    }

    fn count_paths(&self) -> usize {

        let mut res = 0;

        let mut stack: VecDeque<WalkState> = VecDeque::new();

        stack.push_back(WalkState{path: vec!["start".to_string()], hit: false});

        while !stack.is_empty() {

            let state = stack.pop_front().unwrap();

            if state.path[state.path.len() - 1] == "end" {
                println!("{:?}", state);
                res += 1;
                continue;
            } else {
                let candidates = &self.adjacency_list[&state.path[state.path.len() - 1]];

                for candidate in candidates {
                    if candidate.chars().nth(0).unwrap().is_uppercase() {
                        let mut new_state = state.clone();
                        new_state.path.push(candidate.clone());
                        stack.push_back(new_state);
                    } else if !state.path.contains(candidate) {
                        let mut new_state = state.clone();
                        new_state.path.push(candidate.clone());
                        stack.push_back(new_state);
                    } else if !state.hit && (candidate != "start" && candidate != "end"){
                        let mut new_state = state.clone();
                        new_state.hit = true;
                        new_state.path.push(candidate.clone());
                        stack.push_back(new_state);                        
                    }
                }
            }
        }
        res
    }
}

fn main() {

    let input: Vec<String> = BufReader::new(File::open("input.txt").unwrap()).lines().map(|l| l.unwrap()).collect();

    let graph = Graph::from_input(&input);

    println!("number of paths: {}", &graph.count_paths());
}
