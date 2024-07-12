use serde::Deserialize;

use toml_config::{gen_serialize_deserialize_test, TomlConfig};

/// Comment for TestConfig
#[derive(Clone, Debug, PartialEq, Eq, TomlConfig, Deserialize)]
struct TestConfig {
    /// Comment for a that stretches over
    /// multiple lines
    a: String,
    /// Comment for b
    b: usize,
    c: bool,
    /// Comment for TestConfigInnerA
    inner_a: TestConfigInnerA,
}

impl Default for TestConfig {
    fn default() -> Self {
        Self {
            a: "Hello, world!".into(),
            b: 100,
            c: true,
            inner_a: Default::default(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, TomlConfig, Deserialize)]
struct TestConfigInnerA {
    /// Comment for a
    a: String,
    /// Comment for b
    b: usize,
}

impl Default for TestConfigInnerA {
    fn default() -> Self {
        Self {
            a: "Inner hello world".into(),
            b: 100,
        }
    }
}

gen_serialize_deserialize_test!(TestConfig);