# GitVerse Rustic

[![made-with-rust](https://img.shields.io/badge/Made%20with-Rust-black?style=for-the-badge&logo=Rust)][rust]

[![build](https://github.com/thevickypedia/gitverse-rustic/actions/workflows/rust.yml/badge.svg)][build]

Rustic version of [GitVerse][3]

#### Summary
[`gitverse-rustic`][1] is an application written in Rust to create organized release notes.

#### Description
- Uses `git tags` to get all tags for a repository
- Makes GET requests to the repository using GitHub API to fetch release notes
- Organize the release notes in either straight or reverse order
  - Requires tag names and release titles to be the same

## Usage

#### Download Executable
```shell
curl -o asset -LH "Accept: application/octet-stream" "https://github.com/thevickypedia/gitverse-rustic/releases/latest/download/asset_gitverse"
```

#### Arguments
- `debug` - Enable on screen logging
- `reverse` - Generate release notes in reverse mode _(last release first)_

#### Flags
- `--filename` / `-f` - Filename for release notes to be stored in
- `--title` / `-t` - Title for the release notes

## Crate
https://crates.io/crates/gitverse

## Linting
### Requirement
```shell
rustup component add clippy
```
### Usage
```shell
cargo clippy --no-deps --fix --allow-dirty
```

## License & copyright

&copy; Vignesh Rao

Licensed under the [MIT License][2]

[1]: https://github.com/thevickypedia/gitverse-rustic
[2]: https://github.com/thevickypedia/gitverse-rustic/blob/main/LICENSE
[3]: https://github.com/thevickypedia/gitverse
[build]: https://github.com/thevickypedia/gitverse-rustic/actions/workflows/rust.yml
[rust]: https://www.rust-lang.org/
