use std::fs::{self, File};
use std::io::{self, BufRead, Write};
use clap::{App, Arg};
use std::path::Path;
use std::collections::HashMap;

mod utils {
    pub mod preprocessor;
    pub mod counter;
    pub mod word_freq;
    pub mod lang_detect;
    pub mod ngram;
    pub mod generators;
}

use crate::lang_detect::LangInfo;
use crate::counter::Counts;
use crate::utils::{preprocessor, counter, word_freq, lang_detect, ngram, generators};

fn report_generator(counts: Vec<Counts>, lang_infos: Vec<Vec<LangInfo>>, file_names: Vec<String>) -> String {
    let mut report = String::new();

    // generate count table
    let count_table = generators::generate_html_table(&counts, &file_names);
    report.push_str(&count_table);

    generators::generate_many_svgs(lang_infos, file_names.clone());
    for file_name in file_names {
        let svg_header = format!("<h3>Languages included in the {}</h3>", file_name);
        report.push_str(&svg_header);
        let svg_file = format!("<img src=\"{}.svg\" alt=\"SVG Image\">", file_name);
        report.push_str(&svg_file);
    }
    
    report
}

fn replace_invalid_utf8(input: &str) -> String {
    let mut encoded = Vec::new();
    
    for c in input.chars() {
        encoded.extend(c.encode_utf8(&mut [0; 4]).bytes());
    }
    
    String::from_utf8_lossy(&encoded).to_string()
}

fn save_word_frequencies_to_csv(word_frequencies: &HashMap<String, usize>) {
    let mut file = File::create("word_frequencies.csv").expect("Failed to create the output file");

    // Write the header to the CSV file
    writeln!(file, "Word,Frequency").expect("Failed to write to output file");

    // Write word frequencies to the CSV file
    for (word, frequency) in word_frequencies {
        writeln!(file, "{},{}", word, frequency).expect("Failed to write to output file");
    }

    println!("Word frequencies saved to word_frequencies.csv");
}

fn process_file(file_path: &str, mode: &str, n_value: Option<usize>) -> Option<(Counts, Vec<LangInfo>)> {
    let file = File::open(file_path).expect("Cannot open this file.");
    let reader = io::BufReader::new(file);

    let lines: Vec<String> = reader
        .lines()
        .filter_map(|l| {
            match l {
                Ok(line) => {
                    // Replace invalid UTF-8 sequences
                    let cleaned_line = replace_invalid_utf8(&line);
                    Some(preprocessor::clean_text(&cleaned_line))
                }
                Err(_) => None, // Skip invalid lines
            }
        })
        .collect();

    match mode {
        "generate" => {
            let counts = counter::counter(lines.clone());
            let lang_info = lang_detect::lang_detect(lines.clone());
            Some((counts, lang_info))
        }
        "frequency" => {
            let freq = word_freq::word_frequency(lines.clone());
            save_word_frequencies_to_csv(&freq);
            None
        }
        "ngram" => {
            let n_value = n_value.expect("'n' value is required for 'ngram' mode.");
            let ngrams = ngram::generate_ngrams(lines.clone(), n_value);
            let file = format!("{}-gram_file.txt", n_value);
            let mut output_file = File::create(file).expect("Failed to create output file");
            write!(output_file, "{}", ngrams).expect("Failed to write to output file");
            None
        }
        _ => {
            eprintln!("Invalid mode provided. Use --help for usage information.");
            None
        }
    }
}

fn is_text_file(file_path: &Path) -> bool {
    if let Some(extension) = file_path.extension() {
        if let Some(ext_str) = extension.to_str() {
            // Add more text file extensions as needed
            return ext_str == "txt" || ext_str == "text" || ext_str == "md";
        }
    }
    false
}

fn process_folder(folder_path: &str, mode: &str, n_value: Option<usize>) -> (Vec<Counts>, Vec<Vec<LangInfo>>, Vec<String>) {
    let mut counters = Vec::new();
    let mut lang_infos = Vec::new();
    let mut file_names = Vec::new();

    if let Ok(entries) = fs::read_dir(folder_path) {
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();
                if path.is_file() && is_text_file(&path) {
                    if let Some((counts, lang_info)) = process_file(&path.to_string_lossy(), mode, n_value) {
                        counters.push(counts);
                        lang_infos.push(lang_info);
                        file_names.push(path.to_string_lossy().into_owned());
                    }
                }
            }
        }
    }

    (counters, lang_infos, file_names)
}

fn main() {
    let matches = App::new("CorpuX: Text corpus analysis tool")
        .version("1.0")
        .author("Thura Aung <66011606@kmitl.ac.th>")
        .about("Perform various text analysis and generate the report")
        .arg(
            Arg::with_name("input_path")
                .help("The input folder or file to analyze")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::with_name("mode")
                .help("Select the analysis mode")
                .short("m")
                .long("mode")
                .possible_values(&["generate", "frequency", "ngram"])
                .required(true)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("is_folder")
                .help("Specifies whether the input is a folder")
                .short("l")
                .long("folder")
                .required(false)
                .takes_value(false),
        )
        .arg(
            Arg::with_name("n_value")
                .help("Value of 'n' for n-gram analysis")
                .short("n")
                .long("nvalue")
                .required(false)
                .takes_value(true),
        )
        .get_matches();

    let input_path = matches.value_of("input_path").unwrap();
    let mode = matches.value_of("mode").unwrap();
    let is_folder = matches.is_present("is_folder");
    let n_value = matches.value_of("n_value").map(|n| n.parse::<usize>().expect("Invalid 'n' value"));

    let (counters, lang_infos, file_names) = if is_folder {
        process_folder(input_path, mode, n_value)
    } else {
        let mut counters = Vec::new();
        let mut lang_infos = Vec::new();
        let mut file_names = Vec::new();

        if let Some((counts, lang_info)) = process_file(input_path, mode, n_value) {
            counters.push(counts);
            lang_infos.push(lang_info);
            file_names.push(input_path.into());
        }

        (counters, lang_infos, file_names)
    };

    if mode == "generate" {
        // Generate the report
        let report = report_generator(counters, lang_infos, file_names);

        // Define the output HTML file name
        let output_file_name = "report.html";

        // Write the report to the HTML file
        let mut output_file = File::create(output_file_name).expect("Failed to create output file");
        output_file.write_all(report.as_bytes()).expect("Failed to write report to file");

        println!("Report saved to {}", output_file_name);
    }
}