# rAssetFinder

Finds potential hosts & subdomains related to your target domain. (Passive via 3rd party sources)

This project is based on [tomnomnom's assetfinder](https://github.com/tomnomnom/assetfinder)

# Install

rAssetfinder is written in [Rust](https://www.rust-lang.org/). Make sure it is configured on your system
to build it yourself.

```
cargo build --release
```

# Usage
```
Usage: rassetfinder [OPTIONS] <DOMAIN>

Arguments:
  <DOMAIN>  Domain to lookup

Options:
  -o, --output <FILENAME>  Save domains to this file as simple list
  -h, --help               Print help
  -V, --version            Print version
```

By default it will create a file with all findings in the same folder you run `rassetfinder` in. The name of the file is `<DOMAIN_NAME>.txt`.

# Sources

See the following list of sources that are implemented, including in which module you can find the code.
```
 Implemented:
 engine::urlscan             https://urlscan.io/
 engine::crt                 https://crt.sh
 engine::hackertarget        https://api.hackertarget.com
 engine::certspotter         https://api.certspotter.com
 engine::wayback             https://web.archive.org
 engine::virustotal          https://www.virustotal.com     Requires API Key via environment var:VT_API_KEY
```

You can signup on virustotal.com for a free API key. Simply put your key in the environment variable `VT_API_KEY`. If it is not present, the tool skips checks on virustotal.com

If you want more sources to be implemented, please submit an issue or do a PR.

# License
[MIT License](LICENSE)