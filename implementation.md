# Implementation Overview

## Table of contents

- [Configuration](#Configuration-(```main.rs```))
- [Preprocessor Module](#Preprocessor-Module-(```preprocessor.rs```))
- [Counter-Module](#Counter-Module)

---

## Configuration (```main.rs```)
The ```main.rs``` file is responsible for handling command-line arguments and configuring the application. 
It defines an AppConfig struct to store the user's input parameters, such as input path, analysis mode, and n-value for n-gram analysis.

### Modules
The project is divided into several modules, each responsible for a specific aspect of text analysis. 
These modules are located in the utils folder and include:
- **preprocessor**: Handles text preprocessing, cleaning, and character encoding.
- **counter**: Performs text analysis, counting characters, words, and sentences.
- **word_freq**: Calculates word frequencies in the input text.
- **lang_detect**: Detects the languages present in the text corpus.
- **ngram**: Generates n-grams for text analysis.
- **generators**: Provides functions to generate HTML tables and SVG visualizations.

#### Functions
Here's the documentation for the functions based on the provided code:

##### `report_generator`
- **Description**: Combines the analysis results to generate an HTML report with count tables and SVG images displaying language information.
- **Parameters**:
  - `analysis_results` (type: `&AnalysisResults`): A reference to the analysis results, which include character, word, and sentence counts, as well as language information.
  - `output_path` (type: `&str`): A reference to the output path where the HTML report file will be saved.
- **Returns**: None

##### `replace_invalid_utf8`
- **Description**: Replaces invalid UTF-8 sequences with the Unicode replacement character.
- **Parameters**:
  - `input` (type: `&str`): A reference to the input string that may contain invalid UTF-8 sequences.
- **Returns**:
  - A `String` containing the input string with invalid UTF-8 sequences replaced by the Unicode replacement character.

##### `save_word_frequencies_to_csv`
- **Description**: Saves word frequencies to a CSV file.
- **Parameters**:
  - `word_frequencies` (type: `&HashMap<String, usize>`): A reference to a HashMap containing word frequencies (word-to-count mapping).
  - `output_path` (type: `&str`): A reference to the output path where the CSV file will be saved.
- **Returns**: None

##### `process_file`
- **Description**: Processes a single file based on the selected analysis mode. The analysis results are updated based on the content of the file.
- **Parameters**:
  - `file_path` (type: `&str`): A reference to the path of the file to be processed.
  - `analysis_mode` (type: `AnalysisMode`): An enum value indicating the analysis mode (Character, Word, or Sentence).
  - `analysis_results` (type: `&mut AnalysisResults`): A mutable reference to the analysis results to be updated.
- **Returns**: None

##### `is_text_file`
- **Description**: Checks if a file has a text file extension.
- **Parameters**:
  - `filename` (type: `&str`): A reference to the name of the file to be checked.
- **Returns**:
  - A `bool` indicating whether the file has a text file extension (`true` if it is a text file; `false` otherwise).

##### `process_folder`
- **Description**: Processes all text files in a folder based on the selected analysis mode. It scans the folder for text files, processes each file, and updates the analysis results.
- **Parameters**:
  - `folder_path` (type: `&str`): A reference to the path of the folder to be processed.
  - `analysis_mode` (type: `AnalysisMode`): An enum value indicating the analysis mode (Character, Word, or Sentence).
  - `analysis_results` (type: `&mut AnalysisResults`): A mutable reference to the analysis results to be updated.
- **Returns**: None

##### `configure_app`
- **Description**: Configures the application and parses command-line arguments. It sets up the application based on the user's input.
- **Parameters**:
  - `args` (type: `clap::ArgMatches`): A structure containing the parsed command-line arguments.
- **Returns**:
  - An `AppConfig` structure containing the configuration options based on the command-line arguments.

--- 

## Preprocessor Module (```preprocessor.rs```)
The `preprocessor` module plays a crucial role in text preprocessing, cleaning, and character encoding. It ensures that the input text data is properly formatted and prepared for analysis. This module contains functions that handle various aspects of text data preparation. The notable functions within this module are `clean_text` and `is_text_file`.

#### Functions
##### `clean_text(input_text: &str) -> String`

This function takes raw text as input and performs the following text preprocessing tasks:
1. **Unicode Character Selection:** It employs regular expressions to filter out characters that are not Unicode letters, numbers, or white spaces. This ensures that only valid characters are retained for analysis.
2. **Whitespace Handling:** After filtering out unwanted characters, the function trims leading and trailing whitespace to eliminate any extraneous spaces.
3. **Multiple Whitespace Reduction:** It replaces multiple consecutive whitespace characters with a single space, ensuring that words are separated appropriately.
The result is a cleaned and normalized text string ready for further analysis.

**Parameters:**

- `input_text` (type: `&str`): The raw text to be cleaned and preprocessed.

**Returns:**

- A `String` containing the cleaned and preprocessed text.

The `clean_text` function is employed to prepare text data for analysis by eliminating non-Unicode characters and normalizing whitespace, which is an essential step in ensuring that the text analysis results are accurate and consistent.

--- 
## Counter Module (```counter.rs```)
### `counter` Function
- **Description**: This function counts the number of words, lines, and unique words in a collection of text lines. It is used for basic text analysis.
- **Parameters**:
  - `lines` (type: `Vec<String>`): A vector of strings representing the text lines to be analyzed.
- **Returns**:
  - A `Counts` structure containing the word count, line count, and unique word count.

### `Counts` Structure
- **Description**: This structure holds the results of the text analysis, including the word count, line count, and unique word count.
- **Fields**:
  - `word_count` (type: `usize`): The total count of words in the input text lines.
  - `line_count` (type: `usize`): The total count of lines in the input text.
  - `unique_word_count` (type: `usize`): The count of unique words in the input text.

### Example Usage
```rust
fn main() {
    let input_lines = vec![
        "This is a sample sentence.".to_string(),
        "Another sentence for testing.".to_string(),
        "This is a sample sentence.".to_string(),
    ];

    let counts = counter(input_lines);

    println!("Word Count: {}", counts.word_count);
    println!("Line Count: {}", counts.line_count);
    println!("Unique Word Count: {}", counts.unique_word_count);
}
```

### `test_counter` Test Function
- **Description**: This test function validates the `counter` function by providing a set of input lines and checking whether it produces the expected counts.
- **Input**:
  - `input_lines` (type: `Vec<String>`): A set of sample text lines for testing.
- **Assertions**:
  - Checks that the word count, line count, and unique word count produced by the `counter` function match the expected values.
