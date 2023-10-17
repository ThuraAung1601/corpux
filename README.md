# CorpuX: Text corpus analysis tool

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

### Usage
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
