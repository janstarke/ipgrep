# ipgrep
search for IP addresses in text files

# Usage

```
Usage: ipgrep [OPTIONS] [FILE]...

Arguments:
  [FILE]...  

Options:
  -i, --include <INCLUDE>        display only lines who match ALL of the specified criteria. Values
                                 are delimited with comma [possible values: ipv4, ipv6, public,
                                 private, loopback]
  -x, --exclude <EXCLUDE>        hide lines who match ANY of the specified criteria. Values are
                                 delimited with comma [possible values: ipv4, ipv6, public, private,
                                 loopback]
  -I, --ignore-ips <IGNORE_IPS>  ignore any of the specified IP addresses. Values are delimited with
                                 comma
  -c, --colors                   highlight interesting content using colors
  -v, --verbose...               More output per occurrence
  -q, --quiet...                 Less output per occurrence
  -h, --help                     Print help
  -V, --version                  Print version
```

License: GPL-3.0