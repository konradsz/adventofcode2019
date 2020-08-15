use std::collections::HashMap;
use std::fs;

fn count_from_node(
    node: &[String],
    paths: &HashMap<String, Vec<String>>,
    current_distance: u32,
) -> u32 {
    let mut total_node_distance = 0;
    for connection in node {
        total_node_distance += current_distance;

        let inner_node = paths.get(connection);
        if let Some(inner_node) = inner_node {
            total_node_distance += count_from_node(inner_node, paths, current_distance + 1);
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

fn part_1(paths: &HashMap<String, Vec<String>>) {
    let starting_node = paths.get("COM").unwrap();
    let counter = count_from_node(starting_node, &paths, 1);
    assert_eq!(counter, 314_702);
}

fn part_2(paths: &HashMap<String, Vec<String>>) {
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

    assert_eq!(smallest_distance, 439);
}

fn main() {
    let content = fs::read_to_string("input").expect("file not found");
    let content = content.trim();

    let mut paths: HashMap<String, Vec<String>> = HashMap::new();
    for entry in content.lines() {
        let mut iter = entry.split(')');
        let from = iter.next().unwrap();
        let to = iter.next().unwrap();

        let connection = paths.entry(from.to_string()).or_insert(vec![]);
        connection.push(to.to_string());
    }

    part_1(&paths);
    part_2(&paths);
}
