use std::io::{self, Read};

type Result<T> = ::std::result::Result<T, Box<dyn::std::error::Error>>;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    part1(&input)?;
    part2(&input)?;

    Ok(())
}

fn part1(input: &str) -> Result<()> {
    let w = 25;
    let h = 6;

    let parsed = input.chars().collect::<Vec<char>>();
    let layers: Vec<_> = parsed.chunks(w * h).collect();
    let mut cnts: Vec<(i64, i64, i64)> = vec![];

    for layer in layers.iter() {
        let mut c = (0, 0, 0);
        for elem in layer.iter() {
            match elem {
                '0' => c.0 += 1,
                '1' => c.1 += 1,
                '2' => c.2 += 1,
                _ => (),
            }
        }
        cnts.push(c);
    }

    let target = cnts.iter().min_by_key(|x| x.0).unwrap();
    println!("part1: {}", target.1 * target.2);

    Ok(())
}
fn part2(input: &str) -> Result<()> {
    let w = 25;
    let h = 6;

    let parsed = input.chars().collect::<Vec<char>>();
    let layers: Vec<_> = parsed.chunks(w * h).collect();

    let mut img = vec![];

    for i in 0..(w * h) {
        for layer in layers.iter() {
            match layer[i] {
                '0' => {
                    img.push("#".to_owned());
                    break;
                }
                '1' => {
                    img.push(" ".to_owned());
                    break;
                }
                _ => (),
            }
        }
    }

    for line in img.chunks(w) {
        println!("{}", line.join(""));
    }

    Ok(())
}
