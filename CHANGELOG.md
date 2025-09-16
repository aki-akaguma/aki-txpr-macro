# Changelog: aki-txpr-macro

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased] *

## [0.2.0] (2025-09-16)
### Changed
* updated: runnel(0.4.0)

### Added
* `Makefile` and `README.tpl`
* `.github/workflows/test-ubuntu.yml`
* `.github/workflows/test-macos.yml`
* `.github/workflows/test-windows.yml`
* test status badges into `README.md`

### Changed
* rust-version = "1.80.0"
* refactored `Makefile`

### Removed
* `COPYING`

### Fixed
* clippy: `bool\_assert\_comparison`, `uninlined\_format\_args`
* `LICENSE-APACHE`, `LICENSE-MIT`

## [0.1.5] (2023-01-11)
### Added
* badges into `README.md`
* rust-version = "1.56.0" into Cargo.toml

### Changed
* reformat `CHANGELOG.md`
* update depends: aki-gsub(0.1.37), aki-mcolor(0.1.31), aki-mcycle(0.1.28)
* update depends: aki-mline(0.1.31), aki-resort(0.1.23), aki-stats(0.1.17)
* update depends: aki-unbody(0.1.18), aki-xcat(0.1.35), aki-xtee(0.1.24)

## [0.1.4] (2022-06-25)
### Changed
* changes to edition 2021

## [0.1.3] (2022-05-21)
### Fixed
* bug: testtest::test_2

## [0.1.2] (2022-02-07)
### Fixed
* bug: compile error at `write_error()`.

## [0.1.1] (2021-09-10)
### Changed
* update crates: anyhow(1.0.43), runnel(0.3.8)

## [0.1.0] (2021-04-27)
* first commit

[Unreleased]: https://github.com/aki-akaguma/aki-txpr-macro/compare/v0.2.0..HEAD
[0.2.0]: https://github.com/aki-akaguma/aki-txpr-macro/compare/v0.1.5..v0.2.0
[0.1.5]: https://github.com/aki-akaguma/aki-txpr-macro/compare/v0.1.4..v0.1.5
[0.1.4]: https://github.com/aki-akaguma/aki-txpr-macro/compare/v0.1.3..v0.1.4
[0.1.3]: https://github.com/aki-akaguma/aki-txpr-macro/compare/v0.1.2..v0.1.3
[0.1.2]: https://github.com/aki-akaguma/aki-txpr-macro/compare/v0.1.1..v0.1.2
[0.1.1]: https://github.com/aki-akaguma/aki-txpr-macro/compare/v0.1.0..v0.1.1
[0.1.0]: https://github.com/aki-akaguma/aki-txpr-macro/releases/tag/v0.1.0
