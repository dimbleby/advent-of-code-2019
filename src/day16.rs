use std::iter::repeat;
static BASE: [i32; 4] = [0, 1, 0, -1];

pub(crate) fn day16() {
    let line = std::fs::read_to_string("data/day16.txt").expect("Failed to open input");
    let digits: Vec<_> = line
        .trim()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect();

    // Part one.
    let part_one = digits.clone();
    let output = (0..100).fold(part_one, |acc, _| fft(&acc));
    let answer: String = output[0..8]
        .iter()
        .map(|&d| std::char::from_digit(d, 10).unwrap())
        .collect();
    println!("Part one answer is: {}", answer);

    // Part two.  First seven digits of input are 5971509.
    //
    // The trick is that this at this point in the message the patterns are just: a lot of zeros
    // followed by a lot of ones.
    let offset: usize = line[..7].parse().unwrap();
    let part_two = digits.repeat(10000)[offset..].to_owned();
    let output = (0..100).fold(part_two, |acc, _| fast_partial_fft(&acc));
    let answer: String = output[0..8]
        .iter()
        .map(|&d| std::char::from_digit(d, 10).unwrap())
        .collect();
    println!("Part two answer is: {}", answer);
}

fn get_pattern(multiplier: usize) -> impl Iterator<Item = i32> {
    BASE.iter()
        .flat_map(move |&n| repeat(n).take(multiplier))
        .cycle()
        .skip(1)
}

fn fft_at(digits: &[u32], position: usize) -> u32 {
    let pattern = get_pattern(position + 1);
    let value = digits
        .iter()
        .zip(pattern)
        .fold(0i32, |acc, (&d, p)| (acc + (d as i32) * p))
        .abs() as u32;
    value % 10
}

fn fft(digits: &[u32]) -> Vec<u32> {
    let length = digits.len();
    (0..length).map(|p| fft_at(digits, p)).collect()
}

fn fast_partial_fft(digits: &[u32]) -> Vec<u32> {
    let mut output: Vec<u32> = digits
        .iter()
        .rev()
        .scan(0, |state, &d| {
            *state = (*state + d) % 10;
            Some(*state)
        })
        .collect();
    output.reverse();
    output
}
