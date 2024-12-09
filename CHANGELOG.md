# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

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

- disable dependency updates in release-plz config
