use std::collections::HashMap;
use std::fs;

fn count_from_node(
    node: &Vec<String>,
    paths: &HashMap<String, Vec<String>>,
    current_distance: u32,
) -> u32 {
    let mut total_node_distance = 0;
    for connection in node {
        total_node_distance += current_distance;

        let inner_node = paths.get(connection);
        if inner_node.is_some() {
            total_node_distance +=
                count_from_node(inner_node.unwrap(), paths, current_distance + 1);
        }
    }

    total_node_distance
}

fn build_path_from_node(
    out_path: &mut HashMap<String, u32>,
    current_distance: u32,
    node: &str,
    paths: &HashMap<String, Vec<String>>,
) {
    for (key, value) in paths {
        if value.contains(&String::from(node)) {
        	out_path.insert(String::from(key), current_distance);
        	build_path_from_node(out_path, current_distance + 1, key, paths);
        }
    }
}

fn main() {
    let contents = fs::read_to_string("input").expect("file not found");
    let contents = contents.trim();

    let mut paths: HashMap<String, Vec<String>> = HashMap::new();
    for entry in contents.lines() {
        let mut iter = entry.split(')');
        let from = iter.next().unwrap();
        let to = iter.next().unwrap();

        let connection = paths.entry(from.to_string()).or_insert(vec![]);
        connection.push(to.to_string());
    }

    let starting_node = paths.get("COM").unwrap();
    let counter = count_from_node(starting_node, &paths, 1);
    println!("{}", counter);

    let mut path_from_san: HashMap<String, u32> = HashMap::new();
    build_path_from_node(&mut path_from_san, 0, "SAN", &paths);

    let mut path_from_you: HashMap<String, u32> = HashMap::new();
    build_path_from_node(&mut path_from_you, 0, "YOU", &paths);

    let mut smallest_distance = std::u32::MAX;
    for (san_k, san_v) in path_from_san.iter() {
    	if let Some(distance) = path_from_you.get(san_k) {
    		let total = san_v + distance;
    		if total < smallest_distance {
    			smallest_distance = total;
    		}
    	}
    }

    println!("{}", smallest_distance);
}
