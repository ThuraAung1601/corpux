pub fn generate_ngrams(lines: Vec<String>, n: usize) -> String {
    let mut ngram_vec: Vec<String> = Vec::new();
    for line in lines {
        let words: Vec<String> = line.split_whitespace().map(|s| s.to_string()).collect();
        // println!("{:?}", words);
        if words.len() < n {
            return "length of sentences are not enough to form an n-gram.".to_string();
        }
        for i in 0..(words.len() - n + 1) {
            let ngram = &words[i..i + n];
            ngram_vec.push(ngram.join(" ")); // Join the words to form an n-gram
        }
    }
    ngram_vec.join("\n") // Join the n-grams with newlines
}

#[test]
fn test_generate_ngrams() {
    // Define the input data, a vector of strings.
    let input_lines = vec![
        "This is a test sentence.".to_string(),
        "Another sentence for testing.".to_string(),
    ];

    // Call the generate_ngrams function to compute n-grams with n = 2.
    let ngrams = generate_ngrams(input_lines.clone(), 2);

    // Define the expected n-grams as a single string.
    let expected_ngrams = "This is\nis a\na test\ntest sentence.\nAnother sentence\nsentence for\nfor testing.";

    // Compare the actual and expected results.
    assert_eq!(ngrams, expected_ngrams);
}

#[test]
fn test_generate_ngrams_insufficient_length() {
    // Test with insufficient length to form n-grams.
    let input_lines = vec!["Hello world!!!".to_string()];
    let ngrams = generate_ngrams(input_lines.clone(), 3);

    // The result should be an error message.
    let expected_result = "length of sentences are not enough to form an n-gram.".to_string();
    assert_eq!(ngrams, expected_result);
}
