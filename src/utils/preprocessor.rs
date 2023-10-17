use regex::Regex;

pub fn clean_text(input_text: &str) -> String {
    // take only unicode characters and numbers and white-spaces
    let re = Regex::new(r"[^\p{L}\p{N}\s]+").unwrap();
    let cleaned_text = re.replace_all(input_text, "");

    // Trim leading and trailing whitespace
    let trimmed_text = cleaned_text.trim();

    // Replace multiple whitespace with a single space
    let normalized_text = trimmed_text.split_whitespace().collect::<Vec<&str>>().join(" ");

    normalized_text.to_string()
}