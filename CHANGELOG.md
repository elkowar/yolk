# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.0.11](https://github.com/elkowar/yolk/compare/v0.0.10...v0.0.11) - 2024-12-13

### Added

- Implement `yolk watch` command
- add a few hex color utility functions

### Fixed

- Improve parser error message for missing end tag

### Other

- Try harder to make @druskus20 happy
- Slightly clean up parser code
- Improve error message for empty tag
- Update cargo dist to 0.26, try to use include and build-setup for man ([#9](https://github.com/elkowar/yolk/pull/9))
- Use different font for docs headings to make @druskus20 happy
- he animated now
- Try to fix theme
- Setup matching mdbook theme
- *(release)* build man page as part of release process

## [0.0.10](https://github.com/elkowar/yolk/compare/v0.0.9...v0.0.10) - 2024-12-09

### Added

- add yolk edit command
- Add to_json and from_json lua functions
- ensure template expressions are sandboxed
- add contains_key, contains_value, regex_captures functions

### Fixed

- join lines in parser where possible

### Other

- fix clippy warnings
- ensure that yolk_templates can deal with missing files
- Document inspect.lua library
- Add new utility functions to docs
- document yolk safeguard and how to clone
- simplify parser slightly
- enable tagging in release-plz
- Update references to replace function

## [0.0.9](https://github.com/elkowar/yolk/compare/v0.0.8...v0.0.9) - 2024-12-09

### Added

- [**breaking**] rename replace to replace_re (r -> rr)
- add replace_quoted, replace_value functions
- add replace_number tag function

### Fixed

- rename mktmpl to make-template
- show proper errors for yolk eval
- show source in errors in yolk.lua
- parser not preserving newline after conditional end tag

### Other

- add more tests to lua functions
- disable dependency updates in release-plz config
