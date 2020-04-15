# Literally

[![docs](https://docs.rs/literally/badge.svg)](https://docs.rs/literally)
[![crates.io](https://img.shields.io/crates/v/literally.svg)](https://crates.io/crates/literally)
[![License: MIT OR Apache-2.0](https://img.shields.io/crates/l/clippy.svg)](#license)
![CI](https://github.com/estk/literally/workflows/CI/badge.svg)
[![Minimum rustc version](https://img.shields.io/badge/rustc-1.38+-green.svg)](https://github.com/estk/literally#rust-version-requirements)

I found the methods and ergonomics of [`maplit`](#credit) aggravating so I made this. It's basically the same code except that the names are different and the values are  `.into()`'d.

Enjoy.

## Example

```rust
use std::collections::{HashMap, HashSet};
use literally::{hmap, hset};
let m: HashMap<String, HashSet<String>> = hmap!{ 
    "key" => hset!{
        "value"
    }
};
assert_eq!(m.get("key").unwrap().get("value"), Some(&"value".to_string()))
 ```


## Rust Version Requirements

1.38+

## License

Licensed under either of the following at your option.

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

### Credit

Like I said this *is* `maplit` with ergonomics changes, all credit should be directed there. Again, I literally copied and pasted the code from `maplit` and changed the ergonomics ever so slightly.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you shall be dual licensed as above, without any
additional terms or conditions.
