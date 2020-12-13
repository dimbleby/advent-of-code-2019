use std::fs::File;
use std::io::{BufRead, BufReader};
use std::num::ParseIntError;
use std::str::FromStr;

enum Action {
    Cut(i64),
    Deal(u64),
    NewStack,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct ParseActionError;

impl From<ParseIntError> for ParseActionError {
    fn from(_err: ParseIntError) -> Self {
        Self
    }
}

impl FromStr for Action {
    type Err = ParseActionError;

    // cut 123
    // deal with increment -345
    // deal into new stack
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut words = s.split(' ');
        let verb = words.next().ok_or(ParseActionError)?;
        let last = words.last().ok_or(ParseActionError)?;

        let action = match verb {
            "cut" => {
                let count = last.parse::<i64>()?;
                Action::Cut(count)
            }
            "deal" => {
                let count = last.parse::<u64>();
                match count {
                    Ok(n) => Action::Deal(n),
                    _ => Action::NewStack,
                }
            }
            _ => return Err(ParseActionError),
        };
        Ok(action)
    }
}

impl Action {
    // Each action corresponds to a linear mapping x -> mx + a.
    fn linearize(&self, num_cards: u64) -> (u64, u64) {
        match self {
            Action::Cut(n) => {
                let m = 1;
                let a = if *n > 0 {
                    num_cards - *n as u64
                } else {
                    -(*n) as u64
                };
                (m, a)
            }
            Action::Deal(n) => (*n, 0),
            Action::NewStack => (num_cards - 1, num_cards - 1),
        }
    }
}

struct Deck {
    cards: Vec<u32>,
}

impl Deck {
    fn new(cards: Vec<u32>) -> Self {
        Self { cards }
    }

    fn apply(&mut self, action: &Action) {
        match *action {
            Action::NewStack => self.cards.reverse(),
            Action::Cut(n) => {
                if n < 0 {
                    self.cards.rotate_right(-n as usize)
                } else {
                    self.cards.rotate_left(n as usize)
                }
            }
            Action::Deal(n) => {
                let len = self.cards.len();
                let mut cards = vec![0; len];
                let position = (0usize..).step_by(n as usize);
                for (c, p) in self.cards.iter().zip(position) {
                    cards[p % len] = *c;
                }
                self.cards = cards;
            }
        }
    }
}

pub(crate) fn day22() {
    let input = File::open("data/day22.txt").expect("Failed to open input");
    let buffered = BufReader::new(input);
    let lines = buffered.lines().map(|line| line.unwrap());
    let actions: Vec<Action> = lines
        .map(|line| line.parse().expect("Parsing failed"))
        .collect();

    // Part one.
    let cards = (0u32..10007).collect();
    let mut deck = Deck::new(cards);
    for action in &actions {
        deck.apply(action);
    }
    let answer = deck.cards.iter().position(|&n| n == 2019).unwrap();
    println!("Part one answer is: {}", answer);

    // What is the combined effect of our actions, as x -> mx + a?
    let num_cards = 119_315_717_514_047;
    let (mult, add) = actions.iter().fold((1, 0), |acc, action| {
        let pair = action.linearize(num_cards);
        compose(num_cards, acc, pair)
    });

    // Calculate the inverse operation.
    let imult = modular_inverse(num_cards, mult);
    let iadd = modular_multiplication(num_cards, num_cards - add, imult);

    // Calculate the effect of performing that inverse many times.
    let repetitions = 101_741_582_076_661;
    let (m, a) = repeatedly(num_cards, (imult, iadd), repetitions);

    // Finally we can read out the answer.
    let answer = (modular_multiplication(num_cards, m, 2020) + a) % num_cards;
    println!("Part two answer is: {}", answer);
}

fn compose(num_cards: u64, (m1, a1): (u64, u64), (m2, a2): (u64, u64)) -> (u64, u64) {
    // m2 * (m1 x + a1) + a2 -> (m2 * m1)x + m2 * a1 + a2
    let m = modular_multiplication(num_cards, m2, m1);
    let a = (modular_multiplication(num_cards, m2, a1) + a2) % num_cards;
    (m, a)
}

fn repeatedly(num_cards: u64, (mult, add): (u64, u64), count: u64) -> (u64, u64) {
    // End result.
    let mut m = 1;
    let mut a = 0;

    // Working values.
    let mut xm = mult;
    let mut xa = add;
    let mut iterations = count;
    while iterations != 0 {
        if (iterations % 2) == 1 {
            let (m1, a1) = compose(num_cards, (m, a), (xm, xa));
            m = m1;
            a = a1;
        }
        let (m1, a1) = compose(num_cards, (xm, xa), (xm, xa));
        xm = m1;
        xa = a1;
        iterations /= 2;
    }
    (m, a)
}

fn modular_multiplication(modulus: u64, x: u64, y: u64) -> u64 {
    let mut result = 0;
    let mut a = x;
    let mut b = y;
    while b != 0 {
        if (b % 2) == 1 {
            result = (result + a) % modulus;
        };
        a = (a * 2) % modulus;
        b /= 2;
    }
    result
}

fn modular_inverse(modulus: u64, n: u64) -> u64 {
    let mut a = modulus;
    let mut b = n;
    let mut x0 = 0;
    let mut x1 = 1;

    // Extended Euclidean algorithm for GCD.
    while b != 0 {
        let quotient = a / b;
        let remainder = a % b;
        a = b;
        b = remainder;

        let temp = x1;
        x1 = (x0 + modular_multiplication(modulus, modulus - quotient, x1)) % modulus;
        x0 = temp;
    }
    assert_eq!(a, 1);

    x0
}
