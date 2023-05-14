
# Parameter Replacer

This program replace url parameters with new value.

## Install
```bash
$ cargo build --release
```

## Usage/Examples

```bash
Parameter
         Replace Values

Usage: paramrep.exe [OPTIONS] --file <FILE>

Options:
  -f, --file <FILE>            File contain urls
  -n, --new_value <NEW_VALUE>  New value to replace [default: 1337]
  -p, --parameter <PARAMETER>  Parameter to replace; display urls with this parameter, default all parameters
  -r, --report_errors          Report errors and exit the program, default skip url parsing errors
  -h, --help                   Print help
  -V, --version                Print version
```

