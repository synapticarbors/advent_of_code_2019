use std::collections::{HashMap, HashSet};
use std::io::{self, Read};

type Result<T> = ::std::result::Result<T, Box<dyn::std::error::Error>>;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    part1(&input)?;
    part2(&input)?;

    Ok(())
}

#[derive(Debug, Hash, PartialEq, Eq)]
struct Point(i64, i64);

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

struct WireSegment {
    direction: Direction,
    distance: i64,
}

fn input2segments(input: &str) -> Vec<Vec<WireSegment>> {
    input
        .lines()
        .map(|line| {
            line.split(",")
                .map(|s| WireSegment {
                    direction: match s.chars().next().unwrap() {
                        'U' => Direction::Up,
                        'D' => Direction::Down,
                        'L' => Direction::Left,
                        'R' => Direction::Right,
                        _ => unreachable!(),
                    },
                    distance: s[1..].parse().unwrap(),
                })
                .collect()
        })
        .collect()
}

fn segments2coords(segs: &Vec<WireSegment>) -> (HashSet<Point>, HashMap<Point, i64>) {
    let mut x = 0;
    let mut y = 0;
    let mut coords = HashSet::new();
    let mut pathdist = HashMap::new();
    let mut dist = 0;

    for seg in segs.iter() {
        match seg.direction {
            Direction::Up => {
                for _ in 0..seg.distance {
                    y += 1;
                    dist += 1;
                    if coords.insert(Point(x, y)) {
                        pathdist.insert(Point(x, y), dist);
                    }
                }
            }
            Direction::Down => {
                for _ in 0..seg.distance {
                    y -= 1;
                    dist += 1;
                    if coords.insert(Point(x, y)) {
                        pathdist.insert(Point(x, y), dist);
                    }
                }
            }
            Direction::Left => {
                for _ in 0..seg.distance {
                    x -= 1;
                    dist += 1;
                    if coords.insert(Point(x, y)) {
                        pathdist.insert(Point(x, y), dist);
                    }
                }
            }
            Direction::Right => {
                for _ in 0..seg.distance {
                    x += 1;
                    dist += 1;
                    if coords.insert(Point(x, y)) {
                        pathdist.insert(Point(x, y), dist);
                    }
                }
            }
        }
    }

    (coords, pathdist)
}

fn part1(input: &str) -> Result<()> {
    let wire_paths = input2segments(input);
    let wire_coords: Vec<HashSet<Point>> = wire_paths
        .iter()
        .map(|wp| segments2coords(&wp))
        .map(|x| x.0)
        .collect();

    // Get intersection points
    let mut min_dist = std::i64::MAX;
    for x in wire_coords[0].intersection(&wire_coords[1]) {
        let dist = x.0.abs() + x.1.abs();
        if dist < min_dist {
            min_dist = dist;
        }
    }
    println!("part1 :: min dist: {}", min_dist);

    Ok(())
}

fn part2(input: &str) -> Result<()> {
    let wire_paths = input2segments(input);
    let tmp: Vec<(HashSet<Point>, HashMap<Point, i64>)> =
        wire_paths.iter().map(|wp| segments2coords(&wp)).collect();

    let wire_coords: Vec<&HashSet<Point>> = tmp.iter().map(|x| &x.0).collect();
    let path_dist: Vec<&HashMap<Point, i64>> = tmp.iter().map(|x| &x.1).collect();

    // Get intersection points
    let mut min_path_len = std::i64::MAX;
    for x in wire_coords[0].intersection(&wire_coords[1]) {
        let path_len = path_dist[0].get(x).unwrap() + path_dist[1].get(x).unwrap();
        if path_len < min_path_len {
            min_path_len = path_len
        }
    }
    println!("part2 :: min path length: {}", min_path_len);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
        let input = "R8,U5,L5,D3\nU7,R6,D4,L4";
        let wire_paths = input2segments(&input);

        let wire_coords: Vec<HashSet<Point>> =
            wire_paths.iter().map(|wp| segments2coords(&wp)).collect();

        // Get intersection points
        let intersections = wire_coords[0].intersection(&wire_coords[1]);
        println!("{:?}", wire_coords);
        println!("{:?}", intersections);
    }
}
