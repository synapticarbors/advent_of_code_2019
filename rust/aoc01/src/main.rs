use std::io::{self, Read, Write};

type Result<T> = ::std::result::Result<T, Box<dyn::std::error::Error>>;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    part1(&input)?;
    part2(&input)?;

    Ok(())
}

fn fuel_req(mass: i64) -> i64 {
    ((mass as f64 / 3.0).floor() - 2.0) as i64
}

fn part1(input: &str) -> Result<()> {
    let mut fuel_total = 0;
    for line in input.lines() {
        let mass: i64 = line.parse()?;
        fuel_total += fuel_req(mass);
    }

    writeln!(io::stdout(), "part1: {}", fuel_total)?;
    Ok(())
}

fn part2(input: &str) -> Result<()> {
    let mut fuel_total = 0;
    for line in input.lines() {
        let mass: i64 = line.parse()?;

        let mut mod_fuel = fuel_req(mass);
        fuel_total += mod_fuel;
        loop {
            mod_fuel = fuel_req(mod_fuel);
            if mod_fuel > 0 {
                fuel_total += mod_fuel;
            } else {
                break;
            }
        }
    }

    writeln!(io::stdout(), "part2: {}", fuel_total)?;
    Ok(())
}
