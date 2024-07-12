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

    /// Comment for option value c
    c: Option<String>,
}

impl Default for TestConfigInnerA {
    fn default() -> Self {
        Self {
            a: "Inner hello world".into(),
            b: 100,
            c: Some("Hello".into()),
        }
    }
}

gen_serialize_deserialize_test!(TestConfig);

#[test]
fn test_with_options() {
    #[derive(TomlConfig)]
    struct Test {
        /// Some comment
        a: String,

        /// An option without a value
        b: Option<String>,

        /// An option with a value
        c: Option<usize>,

        /// The last comment
        d: usize,
    }

    impl Default for Test {
        fn default() -> Self {
            Self {
                a: "Hello World".into(),
                b: None,
                c: Some(42),
                d: 420,
            }
        }
    }

    assert_eq!(
        "# Some comment\na = \"Hello World\"\n# An option without a value\n#b = \n\n# An option with a value\nc = 42\n\n# The last comment\nd = 420\n",
        Test::default_to_string(),
    )
}

#[test]
fn nested_test() {
    #[derive(TomlConfig)]
    struct NestedConfig {
        /// Yes, nesting is important
        nested_value: String,
    }

    impl Default for NestedConfig {
        fn default() -> Self {
            Self {
                nested_value: "This value is nested".to_string(),
            }
        }
    }

    #[derive(TomlConfig)]
    struct MyConfig {
        /// Some self-explaining comment
        some_value: String,

        /// Option value with default value
        some_optional_value: Option<usize>,

        /// Option value defaulting to `None`
        some_optional_value_filled: Option<u8>,

        nested_config: NestedConfig,
    }

    impl Default for MyConfig {
        fn default() -> Self {
            Self {
                some_value: "Hello World".to_string(),
                some_optional_value: None,
                some_optional_value_filled: Some(42),
                nested_config: NestedConfig::default(),
            }
        }
    }

    // Get the config file
    let result = MyConfig::default_to_string();

    assert_eq!(
        "# Some self-explaining comment\nsome_value = \"Hello World\"\n# Option value with default value\n#some_optional_value = \n\n# Option value defaulting to `None`\nsome_optional_value_filled = 42\n\n\n[nested_config]\n# Yes, nesting is important\nnested_value = \"This value is nested\"\n",
        result
    );
}
