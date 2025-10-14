// names_scores_idiomatic.rs
// A more idiomatic and robust Rust solution for Project Euler 22.
// Uses iterator pipelines for clarity. Keeps the final Euler answer redacted.

use std::fs;
use std::path::Path;

#[inline]
fn alpha_value(name: &str) -> u64 {
    name.bytes()
        .filter(|b| b.is_ascii_alphabetic())
        .map(|b| (b.to_ascii_uppercase() - b'A' + 1) as u64)
        .sum()
}

fn parse_and_sort(input: &str) -> Vec<String> {
    let mut v: Vec<String> = input
        .split(',')
        .map(|s| s.trim_matches('\"').trim().to_string())
        .filter(|s| !s.is_empty())
        .collect();
    v.sort_unstable(); // we don't need stability
    v
}

fn main() {
    const FILE: &str = "names.txt";
    let raw = if Path::new(FILE).exists() {
        fs::read_to_string(FILE).expect("failed to read names.txt")
    } else {
        // Small fallback so the binary runs anywhere.
        String::from("\"MARY\",\"PATRICIA\",\"LINDA\",\"BARBARA\",\"COLIN\"")
    };

    let names = parse_and_sort(&raw);

    // Sum using an iterator pipeline. enumerate() yields (0-based index, &name).
    let total: u64 = names
        .iter()
        .map(|n| alpha_value(n))
        .enumerate()
        .map(|(i, val)| (i as u64 + 1) * val)
        .sum();

    // println!("{}", total); // TODO: reveal locally
    println!("Total of name scores = [redacted]; computed {} names", names.len());

    // Use total to avoid warnings in some build configs
    let _ = total;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn colin_is_53() {
        assert_eq!(alpha_value("COLIN"), 53);
    }
}
