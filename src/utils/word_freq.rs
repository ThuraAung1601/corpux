use std::collections::HashMap;

pub fn word_frequency(lines: Vec<String>) -> HashMap<String, usize> {
    // Word frequency analysis and HashMap construction
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

#[test]
fn test_word_frequency() {
    // Define the input data, a vector of strings.
    let input_lines = vec![
        "This is a test sentence.".to_string(),
        "This is another test sentence.".to_string(),
        "Test, test, and test again!".to_string(),
    ];

    // Call the word_frequency function to compute word frequencies.
    let word_frequencies = word_frequency(input_lines.clone());

    // Define the expected word frequencies.
    let mut expected_frequencies: HashMap<String, usize> = HashMap::new();
    expected_frequencies.insert("this".to_string(), 2);
    expected_frequencies.insert("is".to_string(), 2);
    expected_frequencies.insert("a".to_string(), 1);
    expected_frequencies.insert("test".to_string(), 5);
    expected_frequencies.insert("sentence".to_string(), 2);
    expected_frequencies.insert("another".to_string(), 1);
    expected_frequencies.insert("and".to_string(), 1);
    expected_frequencies.insert("again".to_string(), 1);

    // Compare the actual and expected results.
    assert_eq!(word_frequencies, expected_frequencies);
}

#[test]
fn test_word_frequency_empty_input() {
    // Test with an empty input vector.
    let input_lines: Vec<String> = Vec::new();
    let word_frequencies = word_frequency(input_lines);

    // The result should also be an empty HashMap.
    let expected_frequencies: HashMap<String, usize> = HashMap::new();
    assert_eq!(word_frequencies, expected_frequencies);
}
