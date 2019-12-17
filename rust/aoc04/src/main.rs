type Result<T> = ::std::result::Result<T, Box<dyn::std::error::Error>>;

fn main() -> Result<()> {
    let rmin = 256310;
    let rmax = 732736;

    part1(rmin, rmax)?;
    part2(rmin, rmax)?;

    Ok(())
}

fn int2digits(x: i64) -> Vec<i64> {
    x.to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as i64)
        .collect()
}

fn testnum_part1(d: &Vec<i64>) -> bool {
    let mut last_num = -1;
    let mut found_adj = false;
    for &v in d.iter() {
        if v < last_num {
            return false;
        }

        if last_num == v {
            found_adj = true;
        }

        last_num = v;
    }

    if found_adj {
        true
    } else {
        false
    }
}

fn part1(rmin: i64, rmax: i64) -> Result<()> {
    let mut cnt = 0;
    for t in rmin..=rmax {
        let d = int2digits(t);
        if testnum_part1(&d) {
            cnt += 1;
        }
    }

    println!("Part 1: {}", cnt);

    Ok(())
}

fn testnum_part2(d: &Vec<i64>) -> bool {
    let mut last_num = -1;
    let mut num_consecutive = 1;
    let mut found_adj = false;
    for &v in d.iter() {
        if v < last_num {
            return false;
        }

        if last_num == v {
            num_consecutive += 1;
        } else {
            if num_consecutive == 2 {
                found_adj = true;
            }
            num_consecutive = 1;
        }

        last_num = v;
    }

    // Account for last two digits in number
    if num_consecutive == 2 {
        found_adj = true;
    }

    if found_adj {
        true
    } else {
        false
    }
}

fn part2(rmin: i64, rmax: i64) -> Result<()> {
    let mut cnt = 0;
    for t in rmin..=rmax {
        let d = int2digits(t);
        //println!("{:?}: {}", &d, testnum_part2(&d));
        if testnum_part2(&d) {
            cnt += 1;
        }
    }

    println!("Part 2: {}", cnt);

    Ok(())
}
