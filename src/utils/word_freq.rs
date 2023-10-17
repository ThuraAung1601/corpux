use std::collections::HashMap;

pub fn word_frequency(lines: Vec<String>) -> HashMap<String, usize> {
    let mut word_frequencies: HashMap<String, usize> = HashMap::new();
    for line in &lines {
        for word in line.split_whitespace() {
            if !word.is_empty() {
                let processed = word.trim_matches(|c: char| !c.is_alphabetic()).to_lowercase();
                *word_frequencies.entry(processed).or_insert(0) += 1;
            }
        }
    }
    word_frequencies
}