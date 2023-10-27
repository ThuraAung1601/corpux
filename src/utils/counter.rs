use std::collections::HashSet;

#[allow(dead_code)]
#[derive(Debug)]
pub struct Counts {
    pub word_count: usize,
    pub line_count: usize,
    pub unique_word_count: usize,
}

pub fn counter(lines: Vec<String>) -> Counts {
    let mut wc = 0;
    let mut lc = 0;
    let mut unique_words = HashSet::new();

    for line in &lines {
        if line != "" {
            lc += 1;
            // Count words in the line
            wc += line.split_whitespace().count(); 

            // Count unique words
            for word in line.split_whitespace() {
                unique_words.insert(word);
            }
        }
    }

    Counts {
        word_count: wc,
        line_count: lc,
        unique_word_count: unique_words.len(),
    }
}

#[test]
fn test_counter() {
    let input_lines = vec![
        "This is a sample sentence.".to_string(),
        "Another sentence for testing.".to_string(),
        "This is a sample sentence.".to_string(),
    ];

    let counts = counter(input_lines);

    assert_eq!(counts.word_count, 14);
    assert_eq!(counts.line_count, 3);
    assert_eq!(counts.unique_word_count, 9);
}
