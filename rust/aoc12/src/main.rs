use std::cmp::Ordering;
use std::io::{self, Read};

use regex::Regex;

type Result<T> = ::std::result::Result<T, Box<dyn::std::error::Error>>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Moon {
    pos: [i64; 3],
    vel: [i64; 3],
}

fn sys_from_input(input: &str) -> Vec<Moon> {
    let re = Regex::new(r"<x=(-?\d+), y=(-?\d+), z=(-?\d+)>").unwrap();

    let mut moons = vec![];
    for line in input.lines() {
        let c = re.captures(line).unwrap();
        let moon = Moon {
            pos: [
                c[1].parse().unwrap(),
                c[2].parse().unwrap(),
                c[3].parse().unwrap(),
            ],
            vel: [0, 0, 0],
        };
        moons.push(moon);
    }

    moons
}

fn step(moons: &mut [Moon]) {
    for i in 0..moons.len() {
        for j in i + 1..moons.len() {
            for ai in 0..3 {
                let dv = match moons[i].pos[ai].cmp(&moons[j].pos[ai]) {
                    Ordering::Equal => 0,
                    Ordering::Greater => -1,
                    Ordering::Less => 1,
                };

                moons[i].vel[ai] += dv;
                moons[j].vel[ai] -= dv;
            }
        }
    }

    for moon in moons.iter_mut() {
        for ai in 0..3 {
            moon.pos[ai] += moon.vel[ai];
        }
    }
}

fn calc_total_energy(moons: &[Moon]) -> i64 {
    let mut total_energy = 0;
    for m in moons {
        let pot_energy: i64 = m.pos.iter().map(|&p| p.abs()).sum();
        let kin_energy: i64 = m.vel.iter().map(|&v| v.abs()).sum();
        total_energy += pot_energy * kin_energy;
    }

    total_energy
}

fn get_axis_state(moons: &[Moon], dim: usize) -> Vec<i64> {
    let mut x = moons.iter().map(|m| m.pos[dim]).collect::<Vec<i64>>();
    x.extend(moons.iter().map(|m| m.vel[dim]).collect::<Vec<i64>>());

    x
}

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    part1(&input)?;
    part2(&input)?;

    Ok(())
}

fn part1(input: &str) -> Result<()> {
    let mut moons = sys_from_input(input);

    for _ in 0..1000 {
        step(&mut moons);
    }
    println!("part1: {}", calc_total_energy(&moons));
    Ok(())
}

fn gcd(x: usize, y: usize) -> usize {
    let mut x = x;
    let mut y = y;
    while y != 0 {
        let t = y;
        y = x % y;
        x = t;
    }
    x
}

fn lcm(x: usize, y: usize) -> usize {
    (x * y) / gcd(x, y)
}

fn find_cycle_len(moons: &mut [Moon]) -> usize {
    let init_state = [
        get_axis_state(&moons, 0),
        get_axis_state(&moons, 1),
        get_axis_state(&moons, 2),
    ];

    let mut i: usize = 0;
    let mut cycle_len = [0, 0, 0];

    while cycle_len[0] == 0 || cycle_len[1] == 0 || cycle_len[2] == 0 {
        step(moons);
        i += 1;

        for ai in 0..3 {
            if cycle_len[ai] == 0 && get_axis_state(&moons, ai) == init_state[ai] {
                cycle_len[ai] = i;
            }
        }
    }

    lcm(cycle_len[0], lcm(cycle_len[1], cycle_len[2]))
}

fn part2(input: &str) -> Result<()> {
    let mut moons = sys_from_input(input);
    let l = find_cycle_len(&mut moons);
    println!("part2: {}", l);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
        let input = "<x=-1, y=0, z=2>\n<x=2, y=-10, z=-7>\n<x=4, y=-8, z=8>\n<x=3, y=5, z=-1>";
        let mut moons = sys_from_input(input);
        for i in 0..10 {
            step(&mut moons);
        }
        assert_eq!(calc_total_energy(&moons), 179);
    }

    #[test]
    fn part2_test() {
        let input = "<x=-1, y=0, z=2>\n<x=2, y=-10, z=-7>\n<x=4, y=-8, z=8>\n<x=3, y=5, z=-1>";
        let mut moons = sys_from_input(input);
        let l = find_cycle_len(&mut moons);
        assert_eq!(l, 2772);

        let input = "<x=-8, y=-10, z=0>\n<x=5, y=5, z=10>\n<x=2, y=-7, z=3>\n<x=9, y=-8, z=-3>";
        let mut moons = sys_from_input(input);
        let l = find_cycle_len(&mut moons);
        assert_eq!(l, 4686774924);
    }
}
