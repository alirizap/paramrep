
# Parameter Replacer

This program replace url parameters with new value.

## Install
```bash
$ cargo build --release
```

## Usage/Examples

```bash
$ paramrep -f urls.txt -n alirizap
https://example.com/path/?a=alirizap&b=alirizap

$ paramrep -f urls.txt -p a 
https://example.com/path/?a=1337&b=2

$ paramrep -h
Parameter Replace Values

Usage: paramrep.exe [OPTIONS] --file <FILE>

Options:
  -f, --file <FILE>            File contain urls
  -n, --new_value <NEW_VALUE>  New value to replace with default parameter values [default: 1337]
  -p, --parameter <PARAMETER>  Parameter to replace display urls with this parameter, default all parameters
  -r, --report_errors          Report errors and exit the program default skip url parsing errors
  -h, --help                   Print help
  -V, --version                Print version
```

