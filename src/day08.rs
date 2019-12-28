use crate::itertools::Itertools;
use std::str;

pub(crate) fn day08() {
    let line = std::fs::read_to_string("data/day08.txt").expect("Failed to open input");
    let layers = line
        .trim()
        .as_bytes()
        .chunks(150)
        .map(str::from_utf8)
        .collect::<Result<Vec<_>, _>>()
        .unwrap();

    // Part one.
    let layer = layers
        .iter()
        .min_by_key(|layer| count_char(layer, '0'))
        .unwrap();
    let ones = count_char(&layer, '1');
    let twos = count_char(&layer, '2');
    println!("Part one answer is: {}", ones * twos);

    // Part two.
    let mut visible = layers[0].to_owned();
    for layer in &layers[1..] {
        visible = add_layers(&visible, layer);
    }
    for line in visible.chars().chunks(25).into_iter() {
        println!("{}", line.format(""));
    }
}

fn count_char(s: &str, c: char) -> usize {
    s.chars().filter(|&x| x == c).count()
}

fn add_pixels(upper: char, lower: char) -> char {
    if upper == '2' {
        lower
    } else {
        upper
    }
}

fn add_layers(upper: &str, lower: &str) -> String {
    upper
        .chars()
        .zip(lower.chars())
        .map(|(u, l)| add_pixels(u, l))
        .collect()
}
