# Web Console Logger

A logger for the wasm32-unknown-unknown target that prints all messages to the
web console

## Usage

```rust
#[macro_use]
extern crate log;
extern crate web_console_logger;
extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn main() {
    web_console_logger::init().unwrap();
    // OR
    // web_console_logger::init_with_level(log::Level::Warn).unwrap();

    warn!("This example message will be printed to the web browser's javascript console.");
}
```

## License

Web Console Logger is licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
   http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in Web Console Logger by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms or
conditions.