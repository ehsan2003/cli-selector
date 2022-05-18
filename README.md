a simple cli tool for extracting html tags based on a css selector from html text

### help

```
cli-selector 0.1.0
a simple cli tool for extracting html tags based on a css selector from html text

USAGE:
    cli-selector [OPTIONS] <SELECTOR>

ARGS:
    <SELECTOR>

OPTIONS:
    -h, --help                     Print help information
    -i, --input <FILE>             input file ( specify "-" for stdin ) [default: -]
    -n, --inner
    -s, --separator <SEPARATOR>    separator between matching elements [default: "\n"]
    -V, --version                  Print version information
```

### sample usage

```sh
curl https://test.com | cli-selector "div>a>p"
```

```html
<p>
    some content
    <span><span>
</p>
```
