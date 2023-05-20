# Keedump

A POC [KeePass](https://keepass.info/) master password dumper using [CVE-2023-32784](https://cve.mitre.org/cgi-bin/cvename.cgi?name=CVE-2023-32784),
find further instructions and the original POC [here](https://github.com/vdohney/keepass-password-dumper).

![showcase](https://github.com/ynuwenhof/keedump/assets/100025337/a5553312-27a5-4118-84aa-2bf10f5a6ad8)

## Installation

### Cargo

Make sure the current stable release of [Rust](https://rust-lang.org/tools/install) is installed.

#### Registry

```bash
cargo install keedump
```

#### Manual

```bash
git clone https://github.com/ynuwenhof/keedump.git
cd keedump
cargo install --path .
```

After installing, you can run the application with:

```bash
keedump -i path/to/dump
```

this will print the recovered password into the terminal.

## License

This project is licensed under either of the following licenses, at your option:

* [Apache License, Version 2.0](https://apache.org/licenses/LICENSE-2.0)
  ([LICENSE-APACHE](https://github.com/ynuwenhof/keedump/blob/main/LICENSE-APACHE))
* [MIT License](https://opensource.org/licenses/MIT)
  ([LICENSE-MIT](https://github.com/ynuwenhof/keedump/blob/main/LICENSE-MIT))
