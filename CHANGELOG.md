# Changelog

All notable changes to this project are documented here. The format is based on
[Keep a Changelog](https://keepachangelog.com/en/1.1.0/), and this project
adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] - 2026-06-22

### Added

- Initial release.
- `append` — append a field (or comma-separated list) to a `Vary` header value, with
  case-insensitive de-duplication, casing preservation, token validation, and `*`
  handling.
- `append_fields` — append already-separated literal field names.
- `VaryError` (`FieldRequired`, `InvalidFieldName`).
- Faithful to the `vary` npm package v1.1.2. Zero dependencies; `#![no_std]`.

[0.1.0]: https://github.com/trananhtung/vary-header/releases/tag/v0.1.0
