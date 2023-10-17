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