advent_of_code::solution!(12);
use std::collections::HashSet;
fn in_bounds(x: i32, y: i32, rows: usize, cols: usize) -> bool {
    x >= 0 && x < rows as i32 && y >= 0 && y < cols as i32
}

fn explore_region(mut region: Region, map: &Vec<Vec<char>>, rows: usize, cols: usize) -> Region {
    let mut to_visit = Vec::new();
    to_visit.extend(region.positions.iter().cloned());

    while let Some((x, y)) = to_visit.pop() {
        for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
            let nx = x as i32 + dx;
            let ny = y as i32 + dy;

            if !in_bounds(nx, ny, rows, cols) {
                region.perimeter += 1;
            } else {
                let ux = nx as usize;
                let uy = ny as usize;

                // different plant
                if map[ux][uy] != region.plant {
                    region.perimeter += 1;

                    // add to region
                } else if !region.positions.contains(&(ux, uy)) {
                    region.positions.push((ux, uy));
                    region.area += 1;
                    to_visit.push((ux, uy));
                }
            }
        }
    }

    region
}

struct Region {
    plant: char,
    positions: Vec<(usize, usize)>,
    area: usize,
    perimeter: usize,
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut regions: Vec<Region> = Vec::new();
    let map: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let rows = map.len();
    let cols = map.get(0)?.len();

    for x in 0..rows {
        for y in 0..cols {
            let current_plant = map[x][y];
            if regions
                .iter()
                .filter(|region| region.plant == current_plant)
                .any(|region| region.positions.contains(&(x, y)))
            {
                continue;
            } else {
                let new_region = Region {
                    plant: current_plant,
                    positions: vec![(x, y)],
                    area: 1,
                    perimeter: 0,
                };

                let explored_region = explore_region(new_region, &map, rows, cols);
                regions.push(explored_region);
            }
        }
    }
    let fence_price: usize = regions
        .iter()
        .map(|region| region.perimeter * region.area)
        .sum();
    Some(fence_price)
}

fn count_sides(region: &Region) -> Option<usize> {
    let mut corner_count: usize = 0;
    let corner_directions = vec![(-1, -1), (-1, 1), (1, -1), (1, 1)];

    let pos_i32: HashSet<(i32, i32)> = region
        .positions
        .iter()
        .map(|&(x, y)| (x as i32, y as i32))
        .collect();

    for &(x, y) in region.positions.iter() {
        let x = x as i32;
        let y = y as i32;
        corner_directions.iter().for_each(|d| {

            let adj_x = x + d.0;
            let adj_y = y + d.1;
            // outer corner
            if !pos_i32.contains(&(adj_x, y)) && !pos_i32.contains(&(x, adj_y)) {
                corner_count += 1;
            }
            // inner corner
            else if pos_i32.contains(&(adj_x, y)) 
                    && pos_i32.contains(&(x, adj_y)) 
                    && !pos_i32.contains(&(adj_x, adj_y)) 
            {
                corner_count += 1;
            }
        });
    }
    Some(corner_count)
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut regions: Vec<Region> = Vec::new();
    let map: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let rows = map.len();
    let cols = map.get(0)?.len();

    for x in 0..rows {
        for y in 0..cols {
            let current_plant = map[x][y];
            if regions
                .iter()
                .filter(|region| region.plant == current_plant)
                .any(|region| region.positions.contains(&(x, y)))
            {
                continue;
            } else {
                let new_region = Region {
                    plant: current_plant,
                    positions: vec![(x, y)],
                    area: 1,
                    perimeter: 0,
                };

                let explored_region = explore_region(new_region, &map, rows, cols);
                regions.push(explored_region);
            }
        }
    }
    let fence_price: usize = regions
        .iter()
        .map(|region| count_sides(region).unwrap_or(0) * region.area)
        .sum();
    Some(fence_price)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1930));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1206));
    }
}
