use std::collections::HashMap;
use std::time::Instant;

fn main() {
    let raw_input = include_str!("./input");

    let (graph, edges) = graph(raw_input);

    let start = Instant::now();
    println!("Solution for PART 1: {}", part1(graph.clone()));
    println!("Finished after {:?}", start.elapsed());

    let start = Instant::now();
    println!("Solution for PART 2: {}", part2(graph.clone(), edges));
    println!("Finished after {:?}", start.elapsed());
}

fn part1(graph: HashMap<String, Vec<String>>) -> usize {
    return graph
        .iter()
        .filter(|(v, _)| {
            *v != "shiny gold"
                && path_contains(
                &graph,
                v.to_string(),
                &"shiny gold".to_string(),
            )
        })
        .count();
}

fn part2(
    graph: HashMap<String, Vec<String>>,
    edges: HashMap<(String, String), usize>,
) -> usize {
    let part2 = count(
        &graph,
        &edges,
        "shiny gold".to_string(),
    ) - 1;
    return part2;
}

fn graph(input: &str) -> (HashMap<String, Vec<String>>, HashMap<(String, String), usize>) {
    let mut graph = HashMap::new();
    let mut edges = HashMap::new();

    for line in input.lines().collect::<Vec<&str>>() {
        let pair: Vec<&str> = line.split(" bags contain ").collect();
        let vertex = pair[0].to_string();
        graph.insert(vertex.clone(), vec![]);

        // Remove " bags", " bag" and line endings.
        let neighbors = pair[1].replace(" bags", " bag");
        let neighbors = neighbors.replace(" bag", "");
        let neighbors = neighbors.replace(".", "");

        if neighbors == "no other" {
            continue;
        }

        let adjacent = graph.get_mut(&vertex).unwrap();
        for neighbor in neighbors.split(", ") {
            let parts = neighbor
                .splitn(2, " ")
                .collect::<Vec<&str>>();
            let count = parts[0]
                .parse()
                .unwrap();
            let name = parts[1]
                .to_string();

            adjacent.push(name.clone());
            edges.insert((vertex.clone(), name.clone()), count);
        }
    }

    return (graph, edges);
}

fn path_contains(graph: &HashMap<String, Vec<String>>, current: String, lookup: &str) -> bool {
    if current == *lookup {
        return true;
    }

    let adjacent = graph
        .get(&current)
        .unwrap();

    if adjacent.is_empty() {
        return false;
    }

    return adjacent
        .iter()
        .any(|vertex| path_contains(graph, vertex.clone(), lookup));
}

fn count(
    graph: &HashMap<String, Vec<String>>,
    edges: &HashMap<(String, String), usize>,
    current: String,
) -> usize {
    let adjacent = graph
        .get(&current)
        .unwrap();

    if adjacent.is_empty() {
        return 1;
    }

    return adjacent
        .iter()
        .map(|v| {
            let cost = edges
                .get(&(current.clone(), v.clone()))
                .unwrap();
            return cost * count(graph, edges, v.clone());
        })
        .sum::<usize>()
        + 1;
}
