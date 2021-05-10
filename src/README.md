# ligen
Ligen (Language Interface Generator) is an easy-to-use extensible multi-language binding generator.

### How to use it

```rust
use ligen::ligen;
use ligen_c::ligen_c;
use ligen_cpp::ligen_cpp;

#[ligen(c, cpp)]
pub struct Struct {
    ...
}

#[ligen(c, cpp)]
pub impl Struct {
    fn new() -> Self { ... }
    fn print(&self) { ... }
    fn type() -> String { ... }
}
```