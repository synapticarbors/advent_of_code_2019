use std::collections::HashMap;
use std::io::{self, Read};

type Result<T> = ::std::result::Result<T, Box<dyn::std::error::Error>>;
type OrbitMap = HashMap<String, String>;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    part1(&input)?;
    part2(&input)?;

    Ok(())
}

fn input2map(input: &str) -> OrbitMap {
    let mut omap: HashMap<String, String> = HashMap::new();

    for line in input.lines() {
        let mut parts = line.split(")");
        let inner = parts.next().unwrap();
        let outer = parts.next().unwrap();

        omap.insert(outer.to_string(), inner.to_string());
    }

    omap
}

fn get_orbit_chain(omap: &OrbitMap, start_obj: &str) -> Vec<String> {
    let mut chain = vec![start_obj.to_owned()];
    let mut key = start_obj;

    loop {
        if let Some(next) = omap.get(key) {
            chain.push(next.to_string());
            key = next;
        } else {
            break;
        }
    }

    chain
}

fn part1(input: &str) -> Result<()> {
    let omap = input2map(input);
    let mut cnt = 0;

    for key in omap.keys() {
        let chain = get_orbit_chain(&omap, key);
        cnt += chain.len() - 1;
    }

    println!("part1: {}", cnt);
    Ok(())
}

fn part2(input: &str) -> Result<()> {
    let omap = input2map(input);
    let chain_you = get_orbit_chain(&omap, "YOU");
    let chain_san = get_orbit_chain(&omap, "SAN");

    let (common_obj, dist_you2common) =
        if let Some(pos) = chain_you.iter().position(|x| chain_san.contains(&x)) {
            let common_obj = &chain_you[pos];
            (common_obj, pos - 1)
        } else {
            unreachable!();
        };

    let dist_san2common = chain_san.iter().position(|x| x == common_obj).unwrap() - 1;

    println!("part2: total dist = {}", dist_san2common + dist_you2common);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "COM)B\nB)C\nC)D\nD)E\nE)F\nB)G\nG)H\nD)I\nE)J\nJ)K\nK)L";
        let omap = input2map(input);
        let mut cnt = 0;

        for key in omap.keys() {
            let chain = get_orbit_chain(&omap, key);
            //println!("{:?}", chain);
            cnt += chain.len() - 1;
        }

        println!("p1 -> cnt: {}", cnt);
    }

    #[test]
    fn test_part2() {
        let input = "COM)B\nB)C\nC)D\nD)E\nE)F\nB)G\nG)H\nD)I\nE)J\nJ)K\nK)L\nK)YOU\nI)SAN";
        let omap = input2map(input);
        let chain_you = get_orbit_chain(&omap, "YOU");
        let chain_san = get_orbit_chain(&omap, "SAN");

        let (common_obj, dist2common) =
            if let Some(pos) = chain_you.iter().position(|x| chain_san.contains(&x)) {
                let common_obj = &chain_you[pos];
                (common_obj, pos - 1)
            } else {
                unreachable!();
            };

        let dist_san2common = chain_san.iter().position(|x| x == common_obj).unwrap() - 1;

        println!(
            "common obj: {} dist: {} -- {}",
            common_obj, dist2common, dist_san2common
        );
    }
}
