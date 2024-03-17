use rand::{rngs::ThreadRng, Rng};
use std::env;

#[derive(Debug, PartialEq)]
enum TokenType {
    BadToken,
    DiceCode { number: usize, sides: usize },
}

fn main() -> Result<(), String> {
    let args: Vec<String> = env::args().skip(1).collect();
    let mut rng = rand::thread_rng();
    let tokens = tokenize(&args);
    let mut sum: usize = 0;
    for token in tokens {
        match token {
            TokenType::BadToken => continue,
            TokenType::DiceCode { number, sides } => {
                let rolls = roll(number, sides, &mut rng);
                print_roll(number, sides, &rolls);
                sum += rolls.iter().fold(0, |acc, x| acc + x);
            }
        }
    }

    println!("Total roll:{sum}");
    Ok(())
}

fn tokenize(strings: &Vec<String>) -> Vec<TokenType> {
    strings
        .iter()
        .map(|e| {
            if e.chars()
                .fold(0, |acc, c| acc + if c == 'd' { 1 } else { 0 })
                == 1
                && e.chars().all(|c| c == 'd' || c.is_numeric())
            {
                let (fore, aft) = e.split_once('d').unwrap();
                let fore = if fore.is_empty() {
                    1usize
                } else {
                    fore.parse::<usize>().unwrap()
                };
                let aft = aft.parse::<usize>().unwrap();
                TokenType::DiceCode {
                    number: fore,
                    sides: aft,
                }
            } else {
                TokenType::BadToken
            }
        })
        .collect()
}

fn roll(number: usize, sides: usize, rng: &mut ThreadRng) -> Vec<usize> {
    (0..number).map(|_| rng.gen_range(1..=sides)).collect()
}

fn print_roll(number: usize, sides: usize, rolls: &Vec<usize>) {
    println!("{}d{}:", number, sides);
    println!(
        "\t{}",
        rolls.iter().map(|e| format!("{}, ", e)).collect::<String>()
    );
    println!("Total: {}", rolls.iter().fold(0, |acc, x| acc + x));
    println!("---")
}

#[cfg(test)]

mod tests {
    use crate::{tokenize, TokenType};
    #[test]
    fn tokenizer_test() {
        assert_eq!(
            vec![TokenType::DiceCode {
                number: 3,
                sides: 6
            }],
            tokenize(&vec!["3d6".to_string()])
        );
    }
}
