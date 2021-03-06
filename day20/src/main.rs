use std::collections::{HashMap, VecDeque};
use std::fs;

struct Edge {
    name: String,
    weight: u32,
}

type Graph = HashMap<String, Vec<Edge>>;

enum PortalType {
    TopOutter(String),
    TopInner(String),
    BottomOutter(String),
    BottomInner(String),
    LeftOutter(String),
    LeftInner(String),
    RightOutter(String),
    RightInner(String),
}

#[derive(Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn get_portal_name(
    content: &str,
    position: (usize, usize),
    offset_1: (i32, i32),
    offset_2: (i32, i32),
) -> String {
    vec![
        content
            .lines()
            .nth((position.1 as i32 + offset_1.1) as usize)
            .unwrap()
            .chars()
            .nth((position.0 as i32 + offset_1.0) as usize)
            .unwrap(),
        content
            .lines()
            .nth((position.1 as i32 + offset_2.1) as usize)
            .unwrap()
            .chars()
            .nth((position.0 as i32 + offset_2.0) as usize)
            .unwrap(),
    ]
    .iter()
    .collect::<String>()
}

fn get_portal_type(
    content: &str,
    position: (usize, usize),
    content_width: usize,
    content_height: usize,
    maze_width: usize,
) -> Option<PortalType> {
    if position.1 == 2 {
        let name = get_portal_name(&content, position, (0, -2), (0, -1));
        return Some(PortalType::TopOutter(name));
    } else if position.1 == maze_width + 1
        && position.0 > maze_width + 1
        && position.0 < content_width - maze_width - 2
    {
        let name = get_portal_name(&content, position, (0, 1), (0, 2));
        return Some(PortalType::TopInner(name));
    } else if position.1 == content_height - 3 {
        let name = get_portal_name(&content, position, (0, 1), (0, 2));
        return Some(PortalType::BottomOutter(name));
    } else if position.1 == content_height - maze_width - 2
        && position.0 > maze_width + 1
        && position.0 < content_width - maze_width - 2
    {
        let name = get_portal_name(&content, position, (0, -2), (0, -1));
        return Some(PortalType::BottomInner(name));
    } else if position.0 == 2 {
        let name = get_portal_name(&content, position, (-2, 0), (-1, 0));
        return Some(PortalType::LeftOutter(name));
    } else if position.0 == maze_width + 1
        && position.1 > maze_width + 1
        && position.1 < content_height - maze_width - 2
    {
        let name = get_portal_name(&content, position, (1, 0), (2, 0));
        return Some(PortalType::LeftInner(name));
    } else if position.0 == content_width - 3 {
        let name = get_portal_name(&content, position, (1, 0), (2, 0));
        return Some(PortalType::RightOutter(name));
    } else if position.0 == content_width - maze_width - 2
        && position.1 > maze_width + 1
        && position.1 < content_height - maze_width - 2
    {
        let name = get_portal_name(&content, position, (-2, 0), (-1, 0));
        return Some(PortalType::RightInner(name));
    }

    None
}

fn build_graph(content: &str, portals: &HashMap<(usize, usize), PortalType>) -> Graph {
    let get_next_position = |position: (usize, usize), direction| -> (usize, usize) {
        match direction {
            Direction::Up => (position.0, position.1 - 1),
            Direction::Down => (position.0, position.1 + 1),
            Direction::Left => (position.0 - 1, position.1),
            Direction::Right => (position.0 + 1, position.1),
        }
    };

    let is_empty = |position: (usize, usize)| -> bool {
        content
            .lines()
            .nth((position.1 as i32) as usize)
            .unwrap()
            .chars()
            .nth((position.0 as i32) as usize)
            .unwrap()
            == '.'
    };

    let get_opposite_direction = |direction| -> Direction {
        match direction {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    };

    let mut graph = Graph::new();

    for (portal_position, portal_type) in portals.iter() {
        let (portal_name, initial_direction) = match portal_type {
            PortalType::TopOutter(name) => (name, Direction::Down),
            PortalType::TopInner(name) => (name, Direction::Up),
            PortalType::BottomOutter(name) => (name, Direction::Up),
            PortalType::BottomInner(name) => (name, Direction::Down),
            PortalType::LeftOutter(name) => (name, Direction::Right),
            PortalType::LeftInner(name) => (name, Direction::Left),
            PortalType::RightOutter(name) => (name, Direction::Left),
            PortalType::RightInner(name) => (name, Direction::Right),
        };

        let node = graph
            .entry(portal_name.to_string())
            .or_insert_with(Vec::new);

        let mut queue = VecDeque::new();
        let first_step = get_next_position(*portal_position, initial_direction);
        queue.push_back(first_step);

        let mut visited = HashMap::new();
        visited.insert(*portal_position, get_opposite_direction(initial_direction));
        visited.insert(first_step, get_opposite_direction(initial_direction));

        while !queue.is_empty() {
            let current_position = queue.pop_front().unwrap();

            if let Some(portal) = portals.get(&current_position) {
                let name = match portal {
                    PortalType::TopOutter(name)
                    | PortalType::TopInner(name)
                    | PortalType::BottomOutter(name)
                    | PortalType::BottomInner(name)
                    | PortalType::LeftOutter(name)
                    | PortalType::LeftInner(name)
                    | PortalType::RightOutter(name)
                    | PortalType::RightInner(name) => name,
                };
                let mut step_back_position = current_position;
                let mut weight = 0;

                while step_back_position != *portal_position {
                    let direction = visited[&step_back_position];
                    step_back_position = get_next_position(step_back_position, direction);
                    weight += 1;
                }

                // passing through portal takes 1 step
                if name != "ZZ" {
                    weight += 1;
                }

                node.push(Edge {
                    name: name.to_string(),
                    weight,
                });

                continue;
            }

            let up = get_next_position(current_position, Direction::Up);
            if is_empty(up) && visited.get(&up).is_none() {
                visited.insert(up, get_opposite_direction(Direction::Up));
                queue.push_back(up);
            }

            let down = get_next_position(current_position, Direction::Down);
            if is_empty(down) && visited.get(&down).is_none() {
                visited.insert(down, get_opposite_direction(Direction::Down));
                queue.push_back(down);
            }

            let left = get_next_position(current_position, Direction::Left);
            if is_empty(left) && visited.get(&left).is_none() {
                visited.insert(left, get_opposite_direction(Direction::Left));
                queue.push_back(left);
            }

            let right = get_next_position(current_position, Direction::Right);
            if is_empty(right) && visited.get(&right).is_none() {
                visited.insert(right, get_opposite_direction(Direction::Right));
                queue.push_back(right);
            }
        }
    }

    graph
}

fn build_recursive_graph(
    content: &str,
    portals: &HashMap<(usize, usize), PortalType>,
    recursion_level: u32,
) -> Graph {
    let get_next_position = |position: (usize, usize), direction| -> (usize, usize) {
        match direction {
            Direction::Up => (position.0, position.1 - 1),
            Direction::Down => (position.0, position.1 + 1),
            Direction::Left => (position.0 - 1, position.1),
            Direction::Right => (position.0 + 1, position.1),
        }
    };

    let is_empty = |position: (usize, usize)| -> bool {
        content
            .lines()
            .nth((position.1 as i32) as usize)
            .unwrap()
            .chars()
            .nth((position.0 as i32) as usize)
            .unwrap()
            == '.'
    };

    let get_opposite_direction = |direction| -> Direction {
        match direction {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    };

    let mut graph = Graph::new();

    let extend_by = |name: &str, by: &str| -> String {
        let mut ss = String::from(name);
        if name != "AA" && name != "ZZ" {
            ss.push_str(by);
        }
        ss
    };

    for (portal_position, portal_type) in portals.iter() {
        let (portal_name, initial_direction) = match portal_type {
            PortalType::TopOutter(name) => (extend_by(name, "_o"), Direction::Down),
            PortalType::TopInner(name) => (extend_by(name, "_i"), Direction::Up),
            PortalType::BottomOutter(name) => (extend_by(name, "_o"), Direction::Up),
            PortalType::BottomInner(name) => (extend_by(name, "_i"), Direction::Down),
            PortalType::LeftOutter(name) => (extend_by(name, "_o"), Direction::Right),
            PortalType::LeftInner(name) => (extend_by(name, "_i"), Direction::Left),
            PortalType::RightOutter(name) => (extend_by(name, "_o"), Direction::Left),
            PortalType::RightInner(name) => (extend_by(name, "_i"), Direction::Right),
        };

        let node = graph
            .entry(portal_name.to_string())
            .or_insert_with(Vec::new);

        let mut queue = VecDeque::new();
        let first_step = get_next_position(*portal_position, initial_direction);
        queue.push_back(first_step);

        let mut visited = HashMap::new();
        visited.insert(*portal_position, get_opposite_direction(initial_direction));
        visited.insert(first_step, get_opposite_direction(initial_direction));

        while !queue.is_empty() {
            let current_position = queue.pop_front().unwrap();

            if let Some(portal) = portals.get(&current_position) {
                let name = match portal {
                    PortalType::TopInner(name)
                    | PortalType::BottomInner(name)
                    | PortalType::LeftInner(name)
                    | PortalType::RightInner(name) => extend_by(name, "_i"),
                    PortalType::TopOutter(name)
                    | PortalType::BottomOutter(name)
                    | PortalType::LeftOutter(name)
                    | PortalType::RightOutter(name) => extend_by(name, "_o"),
                };
                let mut step_back_position = current_position;
                let mut weight = 0;

                while step_back_position != *portal_position {
                    let direction = visited[&step_back_position];
                    step_back_position = get_next_position(step_back_position, direction);
                    weight += 1;
                }

                node.push(Edge {
                    name: name.to_string(),
                    weight,
                });

                continue;
            }

            let up = get_next_position(current_position, Direction::Up);
            if is_empty(up) && visited.get(&up).is_none() {
                visited.insert(up, get_opposite_direction(Direction::Up));
                queue.push_back(up);
            }

            let down = get_next_position(current_position, Direction::Down);
            if is_empty(down) && visited.get(&down).is_none() {
                visited.insert(down, get_opposite_direction(Direction::Down));
                queue.push_back(down);
            }

            let left = get_next_position(current_position, Direction::Left);
            if is_empty(left) && visited.get(&left).is_none() {
                visited.insert(left, get_opposite_direction(Direction::Left));
                queue.push_back(left);
            }

            let right = get_next_position(current_position, Direction::Right);
            if is_empty(right) && visited.get(&right).is_none() {
                visited.insert(right, get_opposite_direction(Direction::Right));
                queue.push_back(right);
            }
        }
    }

    let mut graph_recursive = Graph::new();

    for level in 0..recursion_level {
        for (node_name, edges) in graph.iter() {
            if level == 0 && node_name.contains("_o")
                || (level != 0 && (node_name.contains("AA") || node_name.contains("ZZ")))
            {
                continue;
            }
            let updated_node_name: String = if node_name.contains("AA") || node_name.contains("ZZ")
            {
                node_name.to_string()
            } else {
                extend_by(&node_name[0..4], &level.to_string())
            };
            let edge_between_levels = graph_recursive
                .entry(updated_node_name.to_string())
                .or_insert_with(Vec::new);
            if node_name.contains("_o") {
                let name = extend_by(&updated_node_name[0..3], "i");
                let name = extend_by(&name, &(level - 1).to_string());
                (*edge_between_levels).push(Edge { name, weight: 1 });
            } else if node_name.contains("_i") && level < recursion_level - 1 {
                let name = extend_by(&updated_node_name[0..3], "o");
                let name = extend_by(&name, &(level + 1).to_string());
                (*edge_between_levels).push(Edge { name, weight: 1 });
            }

            for edge in edges.iter() {
                if level == 0 && (edge.name.contains("AA") || edge.name.contains("ZZ")) {
                    graph_recursive
                        .get_mut(&updated_node_name)
                        .unwrap()
                        .push(Edge {
                            name: edge.name.to_string(),
                            weight: edge.weight,
                        });
                }
                if (level == 0 && edge.name.contains("_o"))
                    || level != 0 && (edge.name.contains("AA") || edge.name.contains("ZZ"))
                {
                    continue;
                }

                if edge.name.contains("_i") || edge.name.contains("_o") {
                    let name = extend_by(&edge.name[0..4], &level.to_string());
                    graph_recursive
                        .get_mut(&updated_node_name)
                        .unwrap()
                        .push(Edge {
                            name,
                            weight: edge.weight,
                        });
                }
            }
        }
    }

    graph_recursive
}

fn dijkstra(graph: &Graph, starting_node: &str) -> u32 {
    let mut nodes_to_check: Vec<String> = graph.keys().cloned().collect();

    let mut distances = HashMap::new();
    for node in nodes_to_check.iter() {
        distances.insert(node.clone(), std::u32::MAX);
    }
    *distances.get_mut(&String::from(starting_node)).unwrap() = 0;

    while !nodes_to_check.is_empty() {
        let mut min_distance = std::u32::MAX;
        let mut min_name = String::from("");
        for (name, cost) in distances.iter() {
            if nodes_to_check.contains(name) && *cost < min_distance {
                min_distance = *cost;
                min_name = name.to_string();
            }
        }

        let index = nodes_to_check
            .iter()
            .position(|name| *name == *min_name)
            .unwrap();
        nodes_to_check.remove(index);

        for neighbour in graph.get(&min_name).unwrap().iter() {
            if *distances.get(&neighbour.name).unwrap()
                > distances.get(&min_name).unwrap() + neighbour.weight
            {
                *distances.get_mut(&neighbour.name).unwrap() =
                    distances.get(&min_name).unwrap() + neighbour.weight;
            }
        }
    }

    *distances.get("ZZ").unwrap()
}

fn part_1(content: &str, portals: &HashMap<(usize, usize), PortalType>) {
    let graph = build_graph(&content, &portals);
    assert_eq!(dijkstra(&graph, "AA"), 602);
}

fn part_2(content: &str, portals: &HashMap<(usize, usize), PortalType>) {
    let graph = build_recursive_graph(&content, &portals, 26);
    assert_eq!(dijkstra(&graph, "AA"), 6_986);
}

fn main() {
    let content = fs::read_to_string("input").expect("file not found");

    let content_width = content.lines().next().unwrap().len();
    let content_height = content.lines().count();
    let maze_width = 33;

    let mut position = (0, 0);

    let mut portals = HashMap::new();

    for line in content.lines() {
        position.0 = 0;
        for c in line.chars() {
            if c == '.' {
                if let Some(portal_type) = get_portal_type(
                    &content,
                    position,
                    content_width,
                    content_height,
                    maze_width,
                ) {
                    portals.insert(position, portal_type);
                }
            }
            position.0 += 1;
        }
        position.1 += 1;
    }

    part_1(&content, &portals);
    part_2(&content, &portals);
}
