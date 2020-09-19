# rcnb-rs 

![Node.js CI](https://github.com/juzi5201314/rcnb-rs/workflows/bench/badge.svg)
[![Rayon crate](https://img.shields.io/crates/v/rcnb-rs.svg)](https://crates.io/crates/rcnb-rs)
[![Rayon documentation](https://docs.rs/rcnb-rs/badge.svg)](https://docs.rs/rcnb-rs)

The world is based on RC. Thus, *everything* can be encoded into RCNB.

RCNB is available in various languages: **Rust** [JavaScript](https://github.com/rcnbapp/RCNB.js) [C](https://github.com/rcnbapp/librcnb) [PHP](https://github.com/rcnbapp/RCNB.php) [Pascal](https://github.com/rcnbapp/RCNB.pas) ([more..](https://github.com/rcnbapp/))

## Why RCNB?

### RCNB vs Base64

|           | Base64       | RCNB                                                          |
|-----------|--------------|---------------------------------------------------------------|
| Speed     | ❌ Fast       | ✔️ Slow, motivate Intel to improve their CPU                   |
| Printable | ❌ On all OS  | ✔️ Only on newer OS, motivate users to upgrade their legacy OS |
| Niubility | ❌ Not at all | ✔️ RCNB!                                                     |
| Example   | QmFzZTY0Lg== | ȐĉņþƦȻƝƃŔć                                                    |

## Install

### Use [Wapm](https://wapm.io/) (with [Wasmer](https://wasmer.io/))
package: [soeur/rcnb-rs](https://wapm.io/package/soeur/rcnb-rs)

[online playground](https://webassembly.sh/?run-command=wapm%20install%20soeur/rcnb-rs)
```bash
$ wapm install soeur/rcnb-rs
```

### Use Cargo
```bash
$ cargo install rcnb-rs
```

### From source
```bash
$ cargo build --release
// look ./target/release
```

### From source (to wasi)
```bash
$ rustup target add wasm32-wasi
$ cargo build --release --target=wasm32-wasi
// look ./target/wasm32-wasi/release
```

### From [releases](https://github.com/juzi5201314/rcnb-rs/releases)

## Usage

### In code
```rust
use rcnb_rs::{decode, encode};

fn main() {
    let content = "rcnb";
    let encoded = encode(content);
    let decoded = decode(&encoded);

    assert_eq!(content, decoded);
}
```

### Cli
```bash
$ rcnb-rs --help
Usage: rcnb-rs [<content>] [-d] [-e]

RCNB!

Options:
  -d, --decode      decode
  -e, --encode      encode
  --help            display usage information
```

#### Use wapm
```bash
$ wapm run rcnb-rs -- --help
```

#### Example
##### Encode
```bash
$ rcnb-rs rcnb!
ɌcńƁȓČņÞŔć
```
##### Decode
```bash
$ rcnb-rs -d ɌcńƁȓČņÞŔć
rcnb!
```
##### Reads from stdin
```bash
$ cat test.txt | rcnb-rs
or
$ rcnb-rs <test.txt
```

## Performance
[See here](https://github.com/juzi5201314/rcnb-rs/runs/1137919930?check_suite_focus=true#step:4:64)
```
test rcnb_decode_100kib ... bench:   8,722,523 ns/iter (+/- 645,427) = 44 MB/s
test rcnb_decode_10kib  ... bench:     862,019 ns/iter (+/- 96,099) = 44 MB/s
test rcnb_decode_1b     ... bench:         105 ns/iter (+/- 11) = 38 MB/s
test rcnb_decode_1kib   ... bench:      87,963 ns/iter (+/- 8,862) = 43 MB/s
test rcnb_decode_1mib   ... bench:  89,603,925 ns/iter (+/- 3,345,813) = 44 MB/s

test rcnb_encode_100kib ... bench:   2,514,532 ns/iter (+/- 227,809) = 40 MB/s
test rcnb_encode_10kib  ... bench:     251,022 ns/iter (+/- 25,876) = 40 MB/s
test rcnb_encode_1b     ... bench:          63 ns/iter (+/- 6) = 15 MB/s
test rcnb_encode_1kib   ... bench:      25,390 ns/iter (+/- 4,762) = 40 MB/s
test rcnb_encode_1mib   ... bench:  27,145,590 ns/iter (+/- 2,225,573) = 38 MB/s
```
