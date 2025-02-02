use std::collections::{HashMap, HashSet};

advent_of_code::solution!(23);

pub fn part_one(input: &str) -> Option<u32> {
    let mut connection_map: HashMap<&str,Vec<&str>> = HashMap::new();
    let mut triple_network: HashSet<(&str,&str,&str)> = HashSet::new();
    let mut t_triples: u32 = 0;
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
                            if let Some(connect) = connection_map.get(net){
                                if connect.contains(&key){
                                    //println!("triple found {:?} {:?} {:?}" , key, val, net);
                                    triple_network.insert((key, val, net));
                                    
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    for (c1, c2, c3) in &triple_network {
        if [c1, c2, c3].iter().any(|c| c.starts_with('t')) {
            t_triples += 1;
        }
    }
    
    Some(t_triples)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut connection_map: HashMap<&str,Vec<&str>> = HashMap::new();
    //let mut triple_network: HashSet<(&str,&str,&str)> = HashSet::new();
    let mut max_connections: u32 = 0;

    input.lines().for_each(|l| {
        let mut connection = l.split("-");
        let c1 = connection.next().unwrap();
        let c2 = connection.next().unwrap();
        connection_map.entry(c1).or_default().push(c2);
        connection_map.entry(c2).or_default().push(c1);
    });

    for (key,values) in connection_map {
        let mut connections

    }

    None
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
