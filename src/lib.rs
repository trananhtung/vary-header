//! # vary-header — append fields to an HTTP `Vary` header
//!
//! Correctly add field names to a `Vary` response header: de-duplicating
//! case-insensitively, preserving the existing casing, and collapsing to `*` when
//! appropriate. A faithful Rust port of the [`vary`](https://www.npmjs.com/package/vary)
//! npm package (used by `cors`, `compression`, …). Zero dependencies and `#![no_std]`.
//!
//! ```
//! use vary_header::append;
//!
//! assert_eq!(append("", "Accept").unwrap(), "Accept");
//! assert_eq!(append("Accept", "Accept-Encoding").unwrap(), "Accept, Accept-Encoding");
//! assert_eq!(append("Accept", "accept").unwrap(), "Accept"); // already present
//! assert_eq!(append("Accept", "*").unwrap(), "*");           // varies on everything
//! ```

#![no_std]
#![doc(html_root_url = "https://docs.rs/vary-header/0.1.0")]

extern crate alloc;

use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec::Vec;

// Compile-test the README's examples as part of `cargo test`.
#[cfg(doctest)]
#[doc = include_str!("../README.md")]
struct ReadmeDoctests;

/// An error returned when a field cannot be appended.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VaryError {
    /// The field argument was empty.
    FieldRequired,
    /// A field is not a valid header field-name token.
    InvalidFieldName(String),
}

impl core::fmt::Display for VaryError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            VaryError::FieldRequired => f.write_str("field argument is required"),
            VaryError::InvalidFieldName(name) => {
                write!(f, "field argument contains an invalid header name: {name}")
            }
        }
    }
}

impl core::error::Error for VaryError {}

/// Append `field` to a `Vary` header value, returning the new value.
///
/// `field` may be a single field name or a comma-separated list (e.g.
/// `"Accept, Accept-Encoding"`); each name is validated as an RFC 7230 token. Names
/// already present (case-insensitively) are skipped, and existing casing is kept. If
/// the header or any field is `*`, the result is `*`.
///
/// ```
/// assert_eq!(
///     vary_header::append("Origin", "Accept, Origin, Accept-Encoding").unwrap(),
///     "Origin, Accept, Accept-Encoding"
/// );
/// ```
///
/// # Errors
///
/// Returns [`VaryError::FieldRequired`] if `field` is empty, or
/// [`VaryError::InvalidFieldName`] if a parsed field name is not a valid token.
pub fn append(header: &str, field: &str) -> Result<String, VaryError> {
    if field.is_empty() {
        return Err(VaryError::FieldRequired);
    }
    let fields = parse(field);
    append_impl(header, &fields)
}

/// Append several already-separated field names to a `Vary` header value.
///
/// Unlike [`append`], each entry in `fields` is treated as a single literal field
/// name (it is not split on commas) and validated as a token. An empty slice returns
/// the header unchanged.
///
/// ```
/// assert_eq!(
///     vary_header::append_fields("Accept", &["Accept-Encoding", "Origin"]).unwrap(),
///     "Accept, Accept-Encoding, Origin"
/// );
/// ```
///
/// # Errors
///
/// Returns [`VaryError::InvalidFieldName`] if any entry is not a valid token.
pub fn append_fields(header: &str, fields: &[&str]) -> Result<String, VaryError> {
    let owned: Vec<String> = fields.iter().map(|s| (*s).to_string()).collect();
    append_impl(header, &owned)
}

fn append_impl(header: &str, fields: &[String]) -> Result<String, VaryError> {
    // Validate every field name first (matching the reference's ordering).
    for field in fields {
        if !is_field_name(field) {
            return Err(VaryError::InvalidFieldName(field.clone()));
        }
    }

    // An existing unspecified vary.
    if header == "*" {
        return Ok("*".to_string());
    }

    let mut val = header.to_string();
    let mut vals = parse(&lowercase(header));

    // Unspecified vary requested.
    if fields.iter().any(|f| f == "*") || vals.iter().any(|v| v == "*") {
        return Ok("*".to_string());
    }

    for field in fields {
        let fld = lowercase(field);
        if !vals.contains(&fld) {
            vals.push(fld);
            val = if val.is_empty() {
                field.clone()
            } else {
                format!("{val}, {field}")
            };
        }
    }

    Ok(val)
}

/// Tokenize a header into comma-separated fields, trimming surrounding spaces from
/// each (mirroring the reference's `parse`; note it splits only on `,` and trims only
/// ASCII space).
fn parse(header: &str) -> Vec<String> {
    let chars: Vec<char> = header.chars().collect();
    let mut list = Vec::new();
    let mut start = 0;
    let mut end = 0;

    for (i, &c) in chars.iter().enumerate() {
        match c {
            ' ' => {
                if start == end {
                    start = i + 1;
                    end = i + 1;
                }
            }
            ',' => {
                list.push(chars[start..end].iter().collect());
                start = i + 1;
                end = i + 1;
            }
            _ => end = i + 1,
        }
    }
    list.push(chars[start..end].iter().collect());

    list
}

fn lowercase(s: &str) -> String {
    s.chars().flat_map(char::to_lowercase).collect()
}

/// Whether `s` is a valid field-name token (RFC 7230 `1*tchar`).
fn is_field_name(s: &str) -> bool {
    !s.is_empty() && s.chars().all(is_tchar)
}

/// A token character per RFC 7230 (ASCII letters, digits, and the `tchar` symbols).
fn is_tchar(c: char) -> bool {
    c.is_ascii_alphanumeric()
        || matches!(
            c,
            '!' | '#'
                | '$'
                | '%'
                | '&'
                | '\''
                | '*'
                | '+'
                | '-'
                | '.'
                | '^'
                | '_'
                | '`'
                | '|'
                | '~'
        )
}
