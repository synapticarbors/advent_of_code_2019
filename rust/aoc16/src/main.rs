use std::fmt::Write;
use std::io::{self, Read};

use std::iter;

type Result<T> = ::std::result::Result<T, Box<dyn::std::error::Error>>;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    part1(&input)?;
    part2(&input)?;

    Ok(())
}

struct Data {
    state: Vec<i32>,
    patterns: Vec<Vec<i32>>,
}

impl Data {
    fn from_str(input: &str, base_pattern: &[i32]) -> Data {
        let state = str2data(input);
        let mut patterns = vec![];
        for i in 0..state.len() {
            let mut p: Vec<_> = base_pattern
                .iter()
                .flat_map(|&e| iter::repeat(e).take(i + 1))
                .cycle()
                .take(state.len() + 1)
                .collect();

            p.drain(..1);
            patterns.push(p);
        }
        Data {
            state: state,
            patterns: patterns,
        }
    }

    fn apply_fft(&mut self) {
        for i in 0..self.state.len() {
            self.state[i] = self.apply_pattern(i);
        }
    }

    fn apply_pattern(&mut self, i: usize) -> i32 {
        let x: i32 = self
            .state
            .iter()
            .zip(&self.patterns[i])
            .map(|e| (e.0 * e.1))
            .sum();
        x.abs() % 10
    }

    fn get_output(&self) -> String {
        let mut out = String::new();
        for n in self.state.iter().take(8) {
            let _ = write!(&mut out, "{}", n);
        }

        out
    }
}

fn str2data(input: &str) -> Vec<i32> {
    input
        .chars()
        .map(|c| c.to_digit(10).unwrap() as i32)
        .collect()
}

fn part1(input: &str) -> Result<()> {
    let pattern = vec![0, 1, 0, -1];
    let mut data = Data::from_str(input, &pattern);

    for _ in 0..100 {
        data.apply_fft();
    }
    println!("part1: {}", data.get_output());
    Ok(())
}

//####################################
struct DataP2 {
    state: Vec<i32>,
}

impl DataP2 {
    fn from_str(input: &str, base_pattern: &[i32]) -> DataP2 {
        let state = str2data(input);

        DataP2 { state: state }
    }

    fn apply_fft(&mut self) {
        // Assumption based on inspection of the pattern
        // is that the offset of the solution
        // makes it such that we do not have to calculate the
        // first N / 2 elements, so the solution just involves
        // patterns containing 1s and 0s so we just have to sum
        // the appropriate elements of the data.

        let start = self.state.len() / 2;
        let end = self.state.len();

        let mut s = 0;
        for i in (start..end).rev() {
            s += self.state[i];
            self.state[i] = s.abs() % 10;
        }
    }

    fn get_offset(&self) -> usize {
        let offset_str: String = self.state.iter().take(7).map(ToString::to_string).collect();
        let offset: usize = offset_str.parse().unwrap();

        offset
    }

    fn get_output(&self) -> String {
        let mut out = String::new();
        let offset = self.get_offset();
        for n in self.state[offset..offset + 8].iter() {
            let _ = write!(&mut out, "{}", n);
        }

        out
    }

    fn is_valid_assumption(&self) -> bool {
        let offset = self.get_offset();
        offset > self.state.len() / 2
    }
}

fn part2(input: &str) -> Result<()> {
    let new_input = input.repeat(10000);

    let pattern = vec![0, 1, 0, -1];
    let mut data = DataP2::from_str(&new_input, &pattern);

    // Check the assumption is valid
    assert!(data.is_valid_assumption());

    for _ in 0..100 {
        data.apply_fft();
    }
    println!("part1: {}", data.get_output());
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
        let input = "80871224585914546619083218645595";
        let pattern = vec![0, 1, 0, -1];
        let mut data = Data::from_str(input, &pattern);
        //for p in &data.patterns {
        //println!("{:?}", &p);
        //}

        for _ in 0..100 {
            data.apply_fft();
        }

        assert_eq!("24176176", data.get_output());
    }

    #[test]
    fn part2_test() {
        let input = "03036732577212944063491565474664";
        let new_input = input.repeat(10000);

        let pattern = vec![0, 1, 0, -1];
        let mut data = DataP2::from_str(&new_input, &pattern);

        for _ in 0..100 {
            data.apply_fft();
        }

        assert_eq!("84462026", data.get_output());
    }
}
