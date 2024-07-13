# toml-config

Export structs to toml, converting Rust doc strings to comments.

Supports one level of nesting. Fields containing structs must come
after regular fields.

This library provides a derive macro for generating Toml config files including comments.

```toml
[dependencies]
toml-config = "0.1.0"
```

The `Default` implementation is used to generate the config file.

Currently one level of nesting is allowed.

### Supported types:
Each supported type also allows `Option<type>`, `Vec<type>` and `Option<Vec<type>>`
- isize
- i8
- i16
- i32
- i64
- usize
- u8
- u16
- u32
- u64
- f32
- f64
- bool
- String
- PathBuf
- SocketAddr

# Example

### With generic `Default`
```rust
#[derive(Default, TomlConfig)]
struct Test {
    /// Some comment
    a: String,

    /// An option without a value
    b: Option<String>,
}
```

`Test::default_to_string()` will than generate:
```toml
# Some comment
a = ""
# An option without a value
#b = 
```

### With custom `Default`
```rust
#[derive(TomlConfig)]
struct Test {
    /// Some comment
    a: String,

    /// An option without a value
    b: Option<String>,
}

impl Default for Test {
    fn default() -> Self {
        Self {
            a: "Hello World".into(),
            b: "Some value".into(),
        }
    }
}
```

`Test::default_to_string()` will than generate:
```toml
# Some comment
a = "Hello World"
# An option without a value
b = "Some value" 
```

# License
Licensed under either of [Apache License, Version 2.0](LICENSE-APACHE) or [MIT license](LICENSE-MIT) at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
