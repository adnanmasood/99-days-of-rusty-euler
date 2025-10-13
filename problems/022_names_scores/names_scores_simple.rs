// names_scores_simple.rs
// A clear, loop-first solution for Project Euler 22 (Names Scores).
// Reads names from "names.txt" in the current directory if available.
// Final Euler answer intentionally not printed for the 99‑day series.

use std::fs;
use std::path::Path;

fn alpha_value(name: &str) -> u64 {
    // Sum A=1, B=2, ... Z=26; ignore non-letters and quotes
    name.chars()
        .filter(|c| c.is_ascii_alphabetic())
        .map(|c| (c.to_ascii_uppercase() as u8 - b'A' + 1) as u64)
        .sum()
}

fn parse_names(raw: &str) -> Vec<String> {
    raw.split(',')
        .map(|s| s.trim_matches('"').trim().to_string())
        .filter(|s| !s.is_empty())
        .collect()
}

fn main() {
    // Try to read Euler's names.txt; fall back to a tiny demo if missing.
    let input = if Path::new("names.txt").exists() {
        fs::read_to_string("names.txt").expect("failed to read names.txt")
    } else {
        // Small fallback so the program compiles and runs anywhere.
        // Includes the example COLIN (value 53).
        String::from("\"MARY\",\"PATRICIA\",\"LINDA\",\"BARBARA\",\"COLIN\"")
    };

    let mut names = parse_names(&input);
    names.sort(); // alphabetical order; stable not required

    // Compute the total with an explicit loop so you can see the mechanics.
    let mut total: u64 = 0;
    for (idx, name) in names.iter().enumerate() {
        let pos = (idx as u64) + 1; // positions are 1-based
        let val = alpha_value(name);
        total += pos * val;
    }

    // Keep the final answer hidden.
    // println!("{}", total); // TODO: reveal locally
    println!("Total of name scores = [redacted]; computed {} names", names.len());

    // Avoid unused warnings in some setups
    let _ = (total);
}
