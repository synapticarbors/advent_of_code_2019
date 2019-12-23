use std::io::{self, Read};

type Result<T> = ::std::result::Result<T, Box<dyn::std::error::Error>>;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    part1(&input)?;
    part2(&input)?;

    Ok(())
}

#[derive(Debug)]
struct Deck {
    cards: Vec<usize>,
    ncards: usize,
}

enum ShuffleOp {
    Cut(i64),
    DealNew,
    DealIncr(i64),
}

impl Deck {
    fn new(n: usize) -> Deck {
        Deck {
            cards: (0..n).collect(),
            ncards: n,
        }
    }

    fn apply_shuffle(&mut self, op: &ShuffleOp) {
        match op {
            ShuffleOp::Cut(n) => self._shuffle_cut(*n),
            ShuffleOp::DealNew => self._shuffle_deal_new(),
            ShuffleOp::DealIncr(n) => self._shuffle_deal_incr(*n),
        }
    }

    fn _shuffle_cut(&mut self, n: i64) {
        let ix = match n >= 0 {
            true => n as usize,
            false => ((self.ncards as i64) + n) as usize,
        };

        let mut tmp = self.cards.split_off(ix as usize);
        tmp.append(&mut self.cards);
        self.cards = tmp;
    }

    fn _shuffle_deal_new(&mut self) {
        self.cards.reverse();
    }

    fn _shuffle_deal_incr(&mut self, n: i64) {
        let mut tmp: Vec<usize> = vec![0; self.ncards];
        for (i, j) in (0..self.ncards)
            .cycle()
            .step_by(n as usize)
            .take(self.ncards)
            .enumerate()
        {
            tmp[j] = self.cards[i];
        }

        self.cards = tmp;
    }
}

fn input2ops(input: &str) -> Vec<ShuffleOp> {
    let mut ops = Vec::new();
    for line in input.lines() {
        if line.starts_with("deal into new stack") {
            ops.push(ShuffleOp::DealNew);
        } else if line.starts_with("deal with increment") {
            let n = line.rsplit(' ').next().unwrap().parse().unwrap();
            ops.push(ShuffleOp::DealIncr(n));
        } else if line.starts_with("cut") {
            let n = line.rsplit(' ').next().unwrap().parse().unwrap();
            ops.push(ShuffleOp::Cut(n));
        }
    }

    ops
}

fn part1(input: &str) -> Result<()> {
    let ops = input2ops(input);
    let mut deck = Deck::new(10007);

    for op in ops {
        deck.apply_shuffle(&op);
    }

    let ans = deck.cards.iter().position(|&x| x == 2019).unwrap();
    println!("part1: {}", ans);
    Ok(())
}

fn part2(input: &str) -> Result<()> {
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test_deal_new() {
        let mut deck = Deck::new(10);
        deck.apply_shuffle(ShuffleOp::DealNew);

        assert_eq!(deck.cards, vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 0]);
    }

    #[test]
    fn part1_test_cut() {
        let mut deck = Deck::new(10);
        deck.apply_shuffle(ShuffleOp::Cut(3));

        assert_eq!(deck.cards, vec![3, 4, 5, 6, 7, 8, 9, 0, 1, 2]);
    }

    #[test]
    fn part1_test_cut_neg() {
        let mut deck = Deck::new(10);
        deck.apply_shuffle(ShuffleOp::Cut(-4));

        assert_eq!(deck.cards, vec![6, 7, 8, 9, 0, 1, 2, 3, 4, 5]);
    }

    #[test]
    fn part1_test_deal_incr() {
        let mut deck = Deck::new(10);
        deck.apply_shuffle(ShuffleOp::DealIncr(3));

        assert_eq!(deck.cards, vec![0, 7, 4, 1, 8, 5, 2, 9, 6, 3]);
    }

    #[test]
    fn part1_test() {
        let mut deck = Deck::new(10);
        deck.apply_shuffle(ShuffleOp::DealIncr(7));
        deck.apply_shuffle(ShuffleOp::DealNew);
        deck.apply_shuffle(ShuffleOp::DealNew);
        assert_eq!(deck.cards, vec![0, 3, 6, 9, 2, 5, 8, 1, 4, 7]);
    }
}
