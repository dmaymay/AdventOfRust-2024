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

fn bron_kerbosch(
    adj: &HashMap<String, HashSet<String>>,
    max_clique: &mut Vec<String>,
    p: &mut Vec<String>,
    r: &mut Vec<String>,
    x: &mut Vec<String>,
) -> () {
    /*
    BronKerbosch(R, P, X):
        if P is empty and X is empty:
            // R is a maximal clique
            record R as a maximal clique

        choose a pivot u from P ∪ X
        for each vertex v in P \ N(u):
            BronKerbosch(R ∪ {v}, P ∩ N(v), X ∩ N(v))
            P = P \ {v}
            X = X ∪ {v}
    */
    if p.is_empty() && x.is_empty() {
        if max_clique.len() < r.len() {
            max_clique.clear();
            max_clique.extend(r.iter().cloned());
        }
        return;
    }

    // choose a pivot

    let pivot = if !p.is_empty() {
        Some(p[0].clone())
    } else if !x.is_empty() {
        Some(x[0].clone())
    } else {
        None
    };

    if let Some(u) = pivot {
        let u_adjacent = adj.get(&u).unwrap();
        let candidates: Vec<String> = p
            .iter()
            .filter(|v| !u_adjacent.contains(*v))
            .cloned()
            .collect();

        for v in candidates {
            r.push(v.clone());
            let v_adjacent = adj.get(&v).unwrap();
            let mut updated_p = p
                .iter()
                .filter(|o| v_adjacent.contains(*o))
                .cloned()
                .collect();

            let mut updated_x = x
                .iter()
                .filter(|o| v_adjacent.contains(*o))
                .cloned()
                .collect();

            bron_kerbosch(adj, max_clique, &mut updated_p, r, &mut updated_x);

            r.pop();
            p.retain(|node| *node != v);
            x.push(v);
        }
    }
}

pub fn part_two(input: &str) -> Option<String> {
    // Bron–Kerbosch approach
    let mut adjacency_map: HashMap<String, HashSet<String>> = HashMap::new();

    input.lines().for_each(|l| {
        let mut connection = l.split("-");
        let c1 = connection.next().unwrap().to_string();
        let c2 = connection.next().unwrap().to_string();
        adjacency_map
            .entry(c1.clone())
            .or_default()
            .insert(c2.clone());
        adjacency_map.entry(c2).or_default().insert(c1);
    });
    // P -> Candidate set
    let mut candidates: Vec<String> = adjacency_map.keys().cloned().collect();
    // R - Current Clique
    let mut clique = Vec::new();
    // X -> Exclusion set
    let mut exclusion = Vec::new();
    // maximal cliques
    let mut max_clique = Vec::new();

    bron_kerbosch(
        &adjacency_map,
        &mut max_clique,
        &mut candidates,
        &mut clique,
        &mut exclusion,
    );
    max_clique.sort();
    
    //println!("{:?}", max_clique);
    Some(max_clique.join(","))

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
