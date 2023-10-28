# CorpuX: Text corpus analysis tool
Thura Aung, email: 66011606@kmitl.ac.th
```
$ cargo run -- --help
  CorpuX: Text corpus analysis tool 1.0
  Thura Aung <66011606@kmitl.ac.th>
  Perform various text analysis and generate the report
  
  USAGE:
      corpux [FLAGS] [OPTIONS] <input_path> --mode <mode>
  
  FLAGS:
      -h, --help       Prints help information
      -l, --folder     Specifies whether the input is a folder
      -V, --version    Prints version information
  
  OPTIONS:
      -m, --mode <mode>         Select the analysis mode [possible values: generate, frequency, ngram]
      -n, --nvalue <n_value>    Value of 'n' for n-gram analysis
  
  ARGS:
      <input_path>    The input folder or file to analyze
```

## Contents
- [Introduction](#Introduction)
- [Motivation](#Motivation)
- [Features](#Features)
  - [ONE corpus file analysis](#for-ONE-corpus-file-analysis)
  - [FOLDER analysis](#FOLDER-analysis)
- [Usage](#Usage)
- [Demonstration](#Demonstration)
  - [Report Generation](#Report-Generation)
  - [Corpus statistics for a specific text file](#Corpus-statistics-for-a-specific-text-file)
    - [Word frequency table](#Word-frequency-table)
    - [N-gram generation](#N-gram-generation)
- [Implementation](./implementation.md)
- [Test](#Usage)
- [Reference](#Reference)
- [Conclusion](#Conclusion)

## Introduction
CorpuX is a tool for analyzing text and determining which languages are being used and how often specific words appear. This tool would be particularly useful for tasks like understanding the diversity of languages in a text or identifying the most common words used in a document.

## Motivation
The reason this project has been started is to create a tool for understanding and learning from text files. By doing this, I aimed to gain knowledge about the languages used in the text and the frequency of words in the given information.

## Features

#### for ONE corpus file analysis
- **Basic text analysis** for each .txt file and text cleaning before it
- **Read and Count** the number of words, lines, and unique words
- **Language Identification** by percentages included in the txt file based on Unicode character count
- **Report Generation**: generate an HTML report for each .txt file - with a Count table and language percentage bar graph
- **Corpus Statistics**
  - **Ngram**: generate an n-gram of the file
  - **Word Frequency**: make a frequency table and give output as .csv file

#### for FOLDER analysis
- Loop all of the .txt files under the folder and generate a report with file information

## Usage
To generate a report for a text file with word count, line count, unique word count and percentage of languages included
```
cargo run -- <input-file> --mode generate
```
```
cargo run -- <input-file> -m generate
```

To generate a report for a folder of text files with word count, line count, unique word count and percentage of languages included
```
cargo run -- --folder <folder-name> --mode generate
```
```
cargo run -- -l <folder-name> -m generate
```

To get word frequency for a specific text file
```
cargo run -- <input-file> --mode frequency
```
```
cargo run -- <input-file> -m frequency
```

To get n-gram for a specific text file
```
cargo run -- <input-file> --mode ngram --nvalue <n-value>
```
```
cargo run -- <input-file> -m ngram -n <n-value>
```

## Demonstration 

#### Report generation
```
$ cargo run -- test.txt -m generate   
    Finished dev [unoptimized + debuginfo] target(s) in 0.05s
     Running `target/debug/corpux test.txt -m generate`
Report saved to report.html
```
The following is the screenshot of the generated report.html

![screenshot file](./images/report_ss1.png "Screenshot of the generated report.html for a corpus")

```
$ cargo run -- -l txt_files -m generate
    Finished dev [unoptimized + debuginfo] target(s) in 0.04s
     Running `target/debug/corpux -l txt_files -m generate`
Report saved to report_folder.html
```

The following is the screenshot of the generated report_folder.html

![screenshot file](./images/report_ss2.png "Screenshot of the generated report.html for the folder")

***Note***: replace_invalid_utf8 function and preprocessor::clean_text function will replace invalid utf8 and clean non-unicode characters so that there could be some differences in data statistics if we use **wc** command to check the contents. But manual checking has already done for those steps.

#### Corpus statistics for a specific text file
##### Word frequency table
```
cargo run -- test2.txt -m frequency
   Compiling corpux v0.1.0 (/Users/thuraaung/Desktop/KMITL_courseworks/project/corpux)
    Finished dev [unoptimized + debuginfo] target(s) in 0.78s
     Running `target/debug/corpux test2.txt -m frequency`
Word frequencies saved to word_frequencies.csv
```
The following is the result 
```
$ head -5 word_frequencies.csv
  Word,Frequency
  companies,1
  stable,1
  level,2
  minimize,1
```
##### N-gram generation
Sample corpus
```
$ cat test2.txt 
  Corporate bonds offer a slightly higher yield compared to government bonds. 
  However, they come with a moderate level of risk associated with the financial stability of the issuing company. 
  In your case, as you are nearing retirement, you can consider investing in highly-rated corporate bonds to 
  potentially earn a bit more than government bonds while maintaining a reasonable level of safety. Ensure you select bonds from reputable, financially stable companies to minimize credit risk.                                          
```

###### Bi-gram
```
$ cargo run -- test2.txt -m ngram -n 2
```
The following is the bi-gram file
```
$ head -5 2-gram_file.txt 
  Corporate bonds
  bonds offer
  offer a
  a slightly
  slightly higher
```

###### Tri-gram
```
$ cargo run -- test2.txt -m ngram -n 3
```
The following is the tri-gram file
```
$ head -5 3-gram_file.txt
  Corporate bonds offer
  bonds offer a
  offer a slightly
  a slightly higher
  slightly higher yield
```

We can generate to N-gram as long as we have enough sentence length.

## Test
- Unit tests for each utility function
```
$ cargo test 
   Compiling corpux v0.1.0 (/Users/thuraaung/Desktop/KMITL_courseworks/project/corpux)
    Finished test [unoptimized + debuginfo] target(s) in 0.24s
     Running unittests src/main.rs (target/debug/deps/corpux-25e72c67c737876b)

running 6 tests
test utils::ngram::test_generate_ngrams_insufficient_length ... ok
test utils::counter::test_counter ... ok
test utils::ngram::test_generate_ngrams ... ok
test utils::word_freq::test_word_frequency_empty_input ... ok
test utils::word_freq::test_word_frequency ... ok
test utils::lang_detect::test_lang_detect ... ok

test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

## Future Work
Looking forward, there's a lot of potential to expand this mini-project. It could be developed into a more comprehensive text corpus analysis library, by adding more functions for linguistic analysis, such as Part-Of-Speech Tagging. This expansion could have a significant impact on the field of text analysis and provide exciting opportunities for research and practical applications in linguistics and related areas.

## Conclusion
To wrap up this individual project, I want to highlight that I handled all aspects of it by myself. This includes writing all the code and creating test cases. I didn't rely on any external tools or libraries except for "clap," which is used for the Command-Line Interface (CLI). 
I also want to stress that I took great care to ensure the accuracy of the information used in this project. For instance, the Unicode ranges and language data were thoroughly researched and verified. This attention to detail is crucial for the reliability and precision of the language detection and text analysis tool.

## References
- Unicode Character Ranges, Linguistics 538, Computational Methods in Linguistic Research, Link: https://www.ling.upenn.edu/courses/Spring_2003/ling538/UnicodeRanges.html
- Dave MacLeod, "Easy Rust", Github Pages, Link: https://dhghomon.github.io/easy_rust
