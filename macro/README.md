# ligen
Ligen (Language Interface Generator) is a macro-based extensible multi-language binding generator.

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
impl Struct {
    pub fn new() -> Self { ... }
    pub fn print(&self) { ... }
    pub fn type() -> String { ... }
}
```