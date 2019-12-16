use std::io::{self, Read};

type Result<T> = ::std::result::Result<T, Box<dyn::std::error::Error>>;

#[derive(Debug)]
struct IntComputer {
    pub program: Vec<i64>,
    pub cursor: usize,
}

impl IntComputer {
    fn from_str(input: &str) -> IntComputer {
        let prog = input.split(",").map(|s| s.parse().unwrap()).collect();
        IntComputer {
            program: prog,
            cursor: 0,
        }
    }

    fn get_op(&self) -> i64 {
        self.program[self.cursor]
    }

    fn execute_instr(&mut self) -> Result<()> {
        let op = self.get_op();
        match op {
            1 => {
                let p1 = self.program[self.cursor + 1] as usize;
                let p2 = self.program[self.cursor + 2] as usize;
                let p3 = self.program[self.cursor + 3] as usize;
                self.program[p3] = self.program[p1] + self.program[p2];
            }
            2 => {
                let p1 = self.program[self.cursor + 1] as usize;
                let p2 = self.program[self.cursor + 2] as usize;
                let p3 = self.program[self.cursor + 3] as usize;
                self.program[p3] = self.program[p1] * self.program[p2];
            }
            99 => (),
            _ => {
                println!("Error: unexpected op code {}", op);
                return Err("unexpected op code")?;
            }
        }
        Ok(())
    }

    fn run(&mut self) -> Result<()> {
        let mut op = self.program[self.cursor];
        while op != 99 {
            self.execute_instr()?;
            self.cursor += 4;
            op = self.get_op();
        }

        Ok(())
    }
}

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    part1(&input)?;
    part2(&input)?;

    Ok(())
}

fn part1(input: &str) -> Result<()> {
    let mut intcomp = IntComputer::from_str(input);

    // reset state
    intcomp.program[1] = 12;
    intcomp.program[2] = 2;

    intcomp.run()?;
    println!("part 1: {}", intcomp.program[0]);

    Ok(())
}

fn part2(input: &str) -> Result<()> {
    let target = 19690720;

    for noun in 0..100 {
        for verb in 0..100 {
            let mut intcomp = IntComputer::from_str(input);

            // reset state
            intcomp.program[1] = noun;
            intcomp.program[2] = verb;

            intcomp.run()?;

            let result = intcomp.program[0];
            if result == target {
                println!("Found: {}", 100 * noun + verb);
                return Ok(());
            }
        }
    }
    Err("No solution found")?
}
