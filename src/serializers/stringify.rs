/// This module provides a generic serializer that serializes any type T:
/// Display as a string and can be used with `#[serde(with =
/// "simplicity::serializers::stringify")` over a struct field that implements
/// `std::fmt::Display`.
///
///```rust
/// use simplicity::Value;
///
/// #[derive(serde::Serialize)]
/// struct Foo {
///    #[serde(with = "simplicity::serializers::stringify")]
///     v: Value,
/// }
///
/// assert_eq!(
///     serde_json::to_value(&Foo { v: Value::u1(0) }).unwrap(),
///     serde_json::json!( { "v": "0b0" } )
/// );
///```
use std::fmt::Display;

use serde::Serializer;

pub fn serialize<T, S>(value: &T, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
    T: Display,
{
    serializer.serialize_str(&value.to_string())
}

#[cfg(test)]
mod tests {
    use crate::Value;
    use serde::Serialize;

    #[test]
    fn can_stringify_value() {
        #[derive(Serialize)]
        struct Foo {
            #[serde(with = "crate::serializers::stringify")]
            v: Value,
        }

        // values taken from Value's `value_display` test
        let value_string_pairs = [
            (Value::u1(0), "0b0"),
            (Value::u1(1), "0b1"),
            (Value::u4(6), "0x6"),
        ];
        for (v, want) in value_string_pairs {
            assert_eq!(
                serde_json::to_value(&Foo { v }).unwrap(),
                serde_json::json!({ "v": want }),
            );
        }
    }
}
