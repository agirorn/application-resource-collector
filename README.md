# Application resource collector

```
arc --help
Usage: appmon [OPTIONS] [NAME]

Arguments:
  [NAME]

Options:
  -o, --out <FILE>     Save collected samples to file
  -d, --delay <DELAY>  The delay between samples in millisecones [default: 1000]
  -h, --help           Print help
  -V, --version        Print version
```

## Usage

```
cargo run -- '.*process-name-or-pattern.*'
```
