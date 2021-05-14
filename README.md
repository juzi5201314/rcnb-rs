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
[See here](https://github.com/juzi5201314/rcnb-rs/runs/2585150207/)
```
test rcnb_decode_100kib ... bench:   2,966,972 ns/iter (+/- 12,700) = 129 MB/s
test rcnb_decode_10kib  ... bench:     287,475 ns/iter (+/- 2,267) = 134 MB/s
test rcnb_decode_1b     ... bench:          22 ns/iter (+/- 0) = 181 MB/s
test rcnb_decode_1kib   ... bench:      13,393 ns/iter (+/- 171) = 287 MB/s
test rcnb_decode_1mib   ... bench:  31,528,558 ns/iter (+/- 1,043,871) = 125 MB/s

test rcnb_encode_100kib ... bench:   1,324,899 ns/iter (+/- 10,162) = 77 MB/s
test rcnb_encode_10kib  ... bench:     130,809 ns/iter (+/- 418) = 78 MB/s
test rcnb_encode_1b     ... bench:          16 ns/iter (+/- 0) = 62 MB/s
test rcnb_encode_1kib   ... bench:      12,705 ns/iter (+/- 73) = 80 MB/s
test rcnb_encode_1mib   ... bench:  13,942,205 ns/iter (+/- 436,349) = 75 MB/s
```
