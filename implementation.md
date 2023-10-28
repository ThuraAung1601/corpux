# Implementation Overview

## Table of contents

- [Configuration](#Configuration)
- [Preprocessor Module](#Preprocessor-Module)
- [Counter Module](#Counter-Module)
- [Preprocessor Module](#Preprocessor-Module)
- [Counter Module](#Counter-Module)
- [Generators Module](#Generators-Module)
- [Language Detection Module](#Language-Detection-Module)
- [Ngram Generation Module](#Ngram-Generation-Module)
- [Word Frequency Table](#Word-Frequency-Table)

---

The following is the program file structure for the CorpuX project
```
.
├── src
│   ├── main.rs
│   ├── txt_files
│   │   ├── test1.txt
│   │   ├── test2.txt
│   │   ├── test3.txt
│   │   └── test4.txt
│   └── utils
│       ├── counter.rs
│       ├── generators.rs
│       ├── lang_detect.rs
│       ├── ngram.rs
│       ├── preprocessor.rs
│       ├── txt_files
│       │   ├── test1.txt
│       │   ├── test2.txt
│       │   ├── test3.txt
│       │   └── test4.txt
│       └── word_freq.rs
```

---

## Configuration 
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

## Preprocessor Module 
The preprocessor (```preprocessor.rs```) module plays a crucial role in text preprocessing, cleaning, and character encoding. It ensures that the input text data is properly formatted and prepared for analysis. This module contains functions that handle various aspects of text data preparation. The functions within this module are `clean_text` and `is_text_file`.

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
## Counter Module 
```counter.rs```
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

### `test_counter` Test Function
- **Description**: This test function validates the `counter` function by providing a set of input lines and checking whether it produces the expected counts.
- **Input**:
  - `input_lines` (type: `Vec<String>`): A set of sample text lines for testing.
- **Assertions**:
  - Checks that the word count, line count, and unique word count produced by the `counter` function match the expected values.
---

## Generators Module
```generators.rs```
### `generate_html_table` Function
- **Description**: This function generates an HTML table displaying data statistics of text files. It shows counts like word count, unique words, and line count for multiple text files.
- **Parameters**:
  - `count` (type: `&[Counts]`): A slice of `Counts` structures containing the statistics for each text file.
  - `file_names` (type: `&[String]`): A slice of file names corresponding to the text files.
- **Returns**:
  - An HTML table as a string containing the data statistics.

### `generate_svg` Function
- **Description**: This function generates an SVG bar chart displaying language information such as character counts and percentages.
- **Parameters**:
  - `lang` (type: `&[String]`): A slice of language names.
  - `char_vec` (type: `&[usize]`): A slice of character counts corresponding to each language.
  - `percent_vec` (type: `&[f64]`): A slice of percentage values corresponding to each language.
- **Returns**:
  - An SVG bar chart as a string.

### `generate_many_svgs` Function
- **Description**: This function generates multiple SVG files, each containing a bar chart for different language data.
- **Parameters**:
  - `lang_infos` (type: `Vec<Vec<LangInfo>>`): A vector of language information for each file, where `LangInfo` contains language name, character count, and percentage.
  - `file_names` (type: `Vec<String>`): A vector of file names to save the SVG charts.
- **Output**:
  - Multiple SVG files named after the provided `file_names` with `.svg` extension.

These functions are responsible for generating HTML tables and SVG visualizations to represent the data statistics and language information.

---

## Language Detection Module
```lang_detect.rs```

### `Lang` Enum
- **Description**: This is an enumeration representing different languages. It includes various languages, each associated with a language code.

### `LangType` Struct
- **Description**: A structure that associates a language from the `Lang` enum with Unicode character range boundaries and the language's name.
- **Fields**:
  - `lang` (type: `Lang`): The language represented by this entry.
  - `uniup` (type: `u64`): The upper boundary of the Unicode character range associated with the language.
  - `unidown` (type: `u64`): The lower boundary of the Unicode character range associated with the language.
  - `text` (type: `String`): The name of the language.

### `LangInfo` Struct
- **Description**: A structure that contains information about a detected language. It includes the language name, total character count, and percentage.
- **Fields**:
  - `lang` (type: `String`): The name of the detected language.
  - `total_character` (type: `usize`): The total character count for the language.
  - `percentage` (type: `f64`): The percentage of characters in the text that belong to this language.

### `find_unicode` Function
- **Description**: This function takes a character and identifies its associated language using the provided `LangType` information and Unicode character range boundaries.
- **Parameters**:
  - `langtype` (type: `[LangType; 59]`): An array of `LangType` structures that define the Unicode character ranges for different languages.
  - `c` (type: `char`): The character to be identified.
- **Returns**:
  - The detected language from the `Lang` enum.

### `return_lang` Function
- **Description**: This function converts a `Lang` enum value into a human-readable language name as a string.
- **Parameters**:
  - `lang` (type: `Lang`): The language to convert.
- **Returns**:
  - The name of the language as a string.

### `lang_detect` Function
- **Description**: This function detects languages in a list of input lines. It calculates the character count and percentage for each language.
- **Parameters**:
  - `lines` (type: `Vec<String>`): A vector of input lines to analyze.
- **Returns**:
  - A vector of `LangInfo` structures containing information about detected languages, including language name, character count, and percentage.

### `test_lang_detect` Test Function
- **Description**: This is a test function for the `lang_detect` function. It provides a sample input and asserts the output to validate the language detection logic.

These functions and structures are used to detect the language of text data and provide information about the detected languages, such as character counts and percentages.

--- 

## Ngram Generation Module
```ngram.rs```

### `generate_ngrams` Function
- **Description**: This function generates n-grams from a list of input lines, where an n-gram is a contiguous sequence of n words in the text.
- **Parameters**:
  - `lines` (type: `Vec<String>`): A vector of input lines from which n-grams will be generated.
  - `n` (type: `usize`): The size of n-grams to generate.
- **Returns**:
  - A string containing the generated n-grams separated by newlines.

### `test_generate_ngrams` Test Function
- **Description**: This test function validates the behavior of the `generate_ngrams` function when provided with a sample input. It checks if the function correctly generates n-grams and joins them with newlines.

### `test_generate_ngrams_insufficient_length` Test Function
- **Description**: This test function checks how the `generate_ngrams` function handles cases where the input lines are not long enough to form n-grams of the specified size (n).

These functions are used for generating n-grams from input text, and the test functions verify their correctness and robustness in handling different input scenarios.

---

## Word Frequency Table
```word_freq.rs```

### `word_frequency` Function
- **Description**: This function calculates word frequencies from a list of input lines. Word frequencies represent how many times each unique word appears in the text. The function processes words by removing non-alphabetic characters and converting them to lowercase.
- **Parameters**:
  - `lines` (type: `Vec<String>`): A vector of input lines containing text from which word frequencies will be calculated.
- **Returns**:
  - A `HashMap` where keys are unique words and values are the frequencies (counts) of each word in the input text.

### `test_word_frequency` Test Function
**Description**: This test function validates the behavior of the `word_frequency` function when provided with a sample input. It checks if the function correctly calculates word frequencies for the input text.

### `test_word_frequency_empty_input` Test Function
- **Description**: This test function checks how the `word_frequency` function handles cases where the input vector is empty.

These functions are used for counting word frequencies in input text, and the test functions verify their correctness and their ability to handle empty input gracefully.
