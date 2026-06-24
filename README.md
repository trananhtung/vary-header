# vary-header

[![All Contributors](https://img.shields.io/badge/all_contributors-1-orange.svg?style=flat-square)](#contributors-)

[![Crates.io](https://img.shields.io/crates/v/vary-header.svg)](https://crates.io/crates/vary-header)
[![Documentation](https://docs.rs/vary-header/badge.svg)](https://docs.rs/vary-header)
[![CI](https://github.com/trananhtung/vary-header/actions/workflows/ci.yml/badge.svg)](https://github.com/trananhtung/vary-header/actions/workflows/ci.yml)
[![License](https://img.shields.io/crates/l/vary-header.svg)](#license)

**Append field names to an HTTP `Vary` header — correctly.** De-duplicates
case-insensitively, preserves existing casing, validates field-name tokens, and
collapses to `*` when the header (or a field) is `*`. A faithful Rust port of the
[`vary`](https://www.npmjs.com/package/vary) npm package (used by `cors`,
`compression`, …). Zero dependencies and `#![no_std]`.

```rust
use vary_header::append;

assert_eq!(append("", "Accept").unwrap(), "Accept");
assert_eq!(append("Accept", "Accept-Encoding").unwrap(), "Accept, Accept-Encoding");
assert_eq!(append("Accept", "accept").unwrap(), "Accept"); // already present (case-insensitive)
assert_eq!(append("Accept", "*").unwrap(), "*");           // varies on everything
```

## Why vary-header?

Setting `Vary` by hand — `res.headers["vary"] += ", Origin"` — easily produces
duplicates, wrong casing, or a value that should have been `*`. This is the canonical
algorithm (the one behind Express's `cors` and `compression` middleware), ported
faithfully so you get the same result.

> The crate is named `vary-header` because the `vary` name on crates.io is taken by an
> unrelated package.

```toml
[dependencies]
vary-header = "0.1"
```

## API

| Item | Purpose |
| --- | --- |
| `append(header, field)` | Append a field (or comma-separated list) to a `Vary` value |
| `append_fields(header, &[field, …])` | Append already-separated literal field names |
| `VaryError` | `FieldRequired` / `InvalidFieldName` |

## Contributors ✨

This project follows the [all-contributors](https://github.com/all-contributors/all-contributors) specification. Contributions of any kind are welcome — code, docs, bug reports, ideas, reviews! See the [emoji key](https://allcontributors.org/docs/en/emoji-key) for how each contribution is recognized, and open a PR or issue to get involved.

Thanks goes to these wonderful people:

<!-- ALL-CONTRIBUTORS-LIST:START - Do not remove or modify this section -->
<!-- prettier-ignore-start -->
<!-- markdownlint-disable -->
<table>
  <tbody>
    <tr>
      <td align="center" valign="top" width="14.28%"><a href="https://github.com/trananhtung"><img src="https://avatars.githubusercontent.com/u/30992229?v=4?s=100" width="100px;" alt="Tung Tran"/><br /><sub><b>Tung Tran</b></sub></a><br /><a href="https://github.com/trananhtung/vary-header/commits?author=trananhtung" title="Code">💻</a> <a href="#maintenance-trananhtung" title="Maintenance">🚧</a></td>
    </tr>
  </tbody>
</table>

<!-- markdownlint-restore -->
<!-- prettier-ignore-end -->

<!-- ALL-CONTRIBUTORS-LIST:END -->

## License

Licensed under either of [Apache-2.0](LICENSE-APACHE) or [MIT](LICENSE-MIT) at
your option.
