use std::collections::{HashMap, HashSet};

advent_of_code::solution!(23);

fn triple_network(input: &str) -> (HashSet<(&str, &str, &str)>, HashMap<&str, Vec<&str>>) {
    let mut connection_map: HashMap<&str, Vec<&str>> = HashMap::new();
    let mut triple_network: HashSet<(&str, &str, &str)> = HashSet::new();

    input.lines().for_each(|l| {
        let mut connection = l.split("-");
        let c1 = connection.next().unwrap();
        let c2 = connection.next().unwrap();
        connection_map.entry(c1).or_default().push(c2);
        connection_map.entry(c2).or_default().push(c1);
    });
    let keys: Vec<&str> = connection_map.keys().cloned().collect();

    for &key in &keys {
        if let Some(values) = connection_map.get(key) {
            let connected_nodes: Vec<&str> = values.clone();
            for &val in &connected_nodes {
                if let Some(networks) = connection_map.get_mut(val) {
                    networks.retain(|n| *n != key);
                    if networks.len() > 0 {
                        for net in networks.clone() {
                            if let Some(connect) = connection_map.get(net) {
                                if connect.contains(&key) {
                                    triple_network.insert((key, val, net));
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    (triple_network, connection_map)
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut t_triples: u32 = 0;
    let triple_network = triple_network(input).0;

    for (c1, c2, c3) in &triple_network {
        if [c1, c2, c3].iter().any(|c| c.starts_with('t')) {
            t_triples += 1;
        }
    }

    Some(t_triples)
}

fn expand_triple(
    triple: (&str, &str, &str),
    connection_map: &HashMap<&str, Vec<&str>>,
) -> Vec<String> {
    let mut set = vec![
        triple.0.to_string(),
        triple.1.to_string(),
        triple.2.to_string(),
    ];

    let set_a: HashSet<&str> = connection_map[triple.0].iter().copied().collect();
    let set_b: HashSet<&str> = connection_map[triple.1].iter().copied().collect();
    let set_c: HashSet<&str> = connection_map[triple.2].iter().copied().collect();
    let mut common_neighbours = &(&set_a & &set_b) & &set_c;

    while !common_neighbours.is_empty() {
        let candidate = common_neighbours.iter().next().unwrap();
        set.push(candidate.to_string());

        let candidate_neighbors: HashSet<&str> =
            connection_map[candidate].iter().copied().collect();
        common_neighbours = &common_neighbours & &candidate_neighbors;
    }

    set
}

pub fn part_two(input: &str) -> Option<String> {
    let mut largest_set = Vec::new();
    let (triple_network, connection_map) = triple_network(input);

    for triple in triple_network {
        let net_set = expand_triple(triple, &connection_map);
        if net_set.len() > largest_set.len() {
            largest_set = net_set;
        }
    }
    largest_set.sort();

    Some(largest_set.join(","))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
