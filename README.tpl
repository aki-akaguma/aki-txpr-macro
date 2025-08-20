# {{crate}}

[![crate][crate-image]][crate-link]
[![Docs][docs-image]][docs-link]
![Rust Version][rustc-image]
![Apache2/MIT licensed][license-image]
[![Test ubu][test-ubuntu-image]][test-ubuntu-link]
[![Test mac][test-windows-image]][test-windows-link]
[![Test win][test-macos-image]][test-macos-link]

{{readme}}

## libaki-*

| command | description |
|:--------|:------------|
| [aki-gsub]   | substitude text command, replace via regex. |
| [aki-mcolor] | mark up text with color |
| [aki-mcycle] | mark up text with cycling color |
| [aki-mline]  | match line, regex text filter like a grep of linux command. |
| [aki-resort] | sort lines of text. You can use regex to specify the KEY. |
| [aki-stats]  | output the statistics of text, like a wc of linux command. |
| [aki-unbody] | output first or last n lines, like a head and tail of linux command. |
| [aki-xcat]   | concatenate files that are plain, gzip, xz and zstd. |
| [aki-xtee]   | copy standard input to each files and standard output. |

[aki-gsub]:https://crates.io/crates/aki-gsub
[aki-mcolor]:https://crates.io/crates/aki-mcolor
[aki-mcycle]:https://crates.io/crates/aki-mcycle
[aki-mline]:https://crates.io/crates/aki-mline
[aki-resort]:https://crates.io/crates/aki-resort
[aki-stats]:https://crates.io/crates/aki-stats
[aki-unbody]:https://crates.io/crates/aki-unbody
[aki-xcat]:https://crates.io/crates/aki-xcat
[aki-xtee]:https://crates.io/crates/aki-xtee

# Changelogs

[This crate's changelog here.](https://github.com/aki-akaguma/{{crate}}/blob/main/CHANGELOG.md)

# License

This project is licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
   https://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   https://opensource.org/licenses/MIT)

at your option.

[//]: # (badges)

[crate-image]: https://img.shields.io/crates/v/{{crate}}.svg
[crate-link]: https://crates.io/crates/{{crate}}
[docs-image]: https://docs.rs/{{crate}}/badge.svg
[docs-link]: https://docs.rs/{{crate}}/
[rustc-image]: https://img.shields.io/badge/rustc-1.80+-blue.svg
[license-image]: https://img.shields.io/badge/license-Apache2.0/MIT-blue.svg
[test-ubuntu-image]: https://github.com/aki-akaguma/{{crate}}/actions/workflows/test-ubuntu.yml/badge.svg
[test-ubuntu-link]: https://github.com/aki-akaguma/{{crate}}/actions/workflows/test-ubuntu.yml
[test-macos-image]: https://github.com/aki-akaguma/{{crate}}/actions/workflows/test-macos.yml/badge.svg
[test-macos-link]: https://github.com/aki-akaguma/{{crate}}/actions/workflows/test-macos.yml
[test-windows-image]: https://github.com/aki-akaguma/{{crate}}/actions/workflows/test-windows.yml/badge.svg
[test-windows-link]: https://github.com/aki-akaguma/{{crate}}/actions/workflows/test-windows.yml
