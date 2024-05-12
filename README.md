# Rust_study
study rust

## Primitives
strings

floats

integrs

booleans


variables can not change type


### integres
Integer Sizes

i8 8bits(1B)

i16 16bits(2B)

i32 32bits(4B)

i64 64bits(8B)

i128 128bits(16B)

u8 

u16

u32

u63

u128

char a u32 that's been unicode validated

### converting Numbers with as
```rust
fn multiply(x: i64, y: u8) -> i64 {
    return x * (y as i64)
}

fn divide(x: i32, y: u16) -> f64 {
    return x as f64 / y as f64;
}
```
