# Ligen
Ligen (Language Interface Generator) is an extensible macro-based multi-language binding
generator.

We officially support `ligen-c`, a binding generator for the Programming Language C.

You can add ligen to your codebase by adding `#[ligen]` attributes to the items you want to
export. It is as simple as this:
```rust
use ligen::ligen;
use ligen_c::ligen_c;
use ligen_cpp::ligen_cpp;

pub struct Counter {
  count: u32
}

#[ligen(c, cpp)]
impl Counter {
  pub fn new() -> Self { Self { count: 0 } }

  pub fn count(&mut self) { self.count += 1; }

  pub fn get_count(&self) -> u32 { self.count }
}
```
which generates the following file:

### Getting started

Here are a few links to get started:
* [List of officially supported languages](https://github.com/search?q=org%3Asensorial-systems+ligen).
* [Usage example](https://github.com/sensorial-systems/ligen-c/tree/main/examples/counter/README.md).