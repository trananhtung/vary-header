//! Behavioral spec for `vary-header`, cross-checked against the npm `vary` package.

use vary_header::{append, append_fields, VaryError};

#[test]
fn basic() {
    assert_eq!(append("", "Accept").unwrap(), "Accept");
    assert_eq!(
        append("Accept", "Accept-Encoding").unwrap(),
        "Accept, Accept-Encoding"
    );
    assert_eq!(append("User-Agent", "User-Agent").unwrap(), "User-Agent");
}

#[test]
fn case_insensitive_dedup_preserves_existing_case() {
    assert_eq!(append("Accept", "accept").unwrap(), "Accept");
    assert_eq!(
        append("Accept-Encoding", "accept-encoding").unwrap(),
        "Accept-Encoding"
    );
    assert_eq!(append("accept", "ACCEPT").unwrap(), "accept");
}

#[test]
fn comma_separated_field_string() {
    assert_eq!(
        append("", "Accept, Accept-Encoding").unwrap(),
        "Accept, Accept-Encoding"
    );
    assert_eq!(
        append("Accept", "Accept-Encoding, Accept").unwrap(),
        "Accept, Accept-Encoding"
    );
    assert_eq!(
        append("Origin", "Accept, Origin, Accept-Encoding").unwrap(),
        "Origin, Accept, Accept-Encoding"
    );
    assert_eq!(
        append("Accept", "has,comma,but,fine").unwrap(),
        "Accept, has, comma, but, fine"
    );
}

#[test]
fn star() {
    assert_eq!(append("*", "Accept").unwrap(), "*");
    assert_eq!(append("Accept", "*").unwrap(), "*");
    assert_eq!(append("Accept, *", "Origin").unwrap(), "*");
    assert_eq!(append("", "*").unwrap(), "*");
}

#[test]
fn errors() {
    assert_eq!(append("Accept", ""), Err(VaryError::FieldRequired));
    assert_eq!(
        append("Accept", "bad name"),
        Err(VaryError::InvalidFieldName("bad name".into()))
    );
    assert_eq!(
        append("Accept", " "),
        Err(VaryError::InvalidFieldName(String::new()))
    );
}

#[test]
fn fields_array_form() {
    assert_eq!(
        append_fields("Accept", &["Accept-Encoding", "Origin"]).unwrap(),
        "Accept, Accept-Encoding, Origin"
    );
    assert_eq!(append_fields("", &[]).unwrap(), "");
    assert_eq!(append_fields("Accept", &["*"]).unwrap(), "*");
    assert_eq!(append_fields("", &["X-Foo"]).unwrap(), "X-Foo");
    // an array entry is a literal name — not split on commas — so a space is invalid
    assert_eq!(
        append_fields("Accept", &["bad name"]),
        Err(VaryError::InvalidFieldName("bad name".into()))
    );
}
