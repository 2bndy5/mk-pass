# Contributing

First, thank you for considering a contribution to this project!

This document should help detail the project's expectations about contributions.

## Development tools

This project uses the following tools for development:

- :simple-rust: [Rust][rustup]
- :simple-uv: [uv] for managing Python dependencies.
- :simple-yarn: [yarn] for managing JavaScript dependencies.
  Actually, `yarn@4.9.2` is checked into this repository, so
  really [Node.js] needs to be installed with `corepack` enabled.
  [fnm] is recommended to install [Node.js].
- [nur] for running common development tasks
- [cargo-nextest] for running Rust tests
- [cargo-llvm-cov] for collecting Rust code coverage
- [clang-format] v14 for formatting C++ sources
- [cbindgen] for generating the C++ header file for the C++ binding

### Optional tools

- [committed] for verifying commit messages conform to [conventional commit] standards.
- [git-cliff] for generating a [Changelog](CHANGELOG.md) and release notes.

[rustup]: https://rustup.rs/
[uv]: https://docs.astral.sh/uv/
[yarn]: https://yarnpkg.com/
[cargo-llvm-cov]: https://crates.io/crates/cargo-llvm-cov
[cargo-nextest]: https://crates.io/crates/cargo-nextest
[nur]: https://crates.io/crates/nur
[committed]: https://crates.io/crates/committed
[conventional commit]: https://www.conventionalcommits.org
[git-cliff]: https://crates.io/crates/git-cliff
[Node.js]: https://nodejs.org/en
[fnm]: https://github.com/Schniz/fnm
[clang-format]: https://releases.llvm.org/download.html
[cbindgen]: https://github.com/mozilla/cbindgen/

## Code style

[pre-commit]: https://pre-commit.com

This project's CI leverages [pre-commit] to ensure

- [x] line endings all use LF (not CRLF)
- [x] lines have no trailing whitespace
- [x] files end with a blank line
- [x] valid syntax is used in all yaml and toml files
- [x] no large files (greater than 500 kB) are added
- [x] no unknown or misspelled words are present

To run [pre-commit], you can use [uv] or [nur]:

```shell
uv run pre-commit run --all-files
```

The following [nur] command uses the above [uv] command

```shell
nur pre-commit
```

Additional options are shown and documented with `nur pre-commit -h`.

> [!TIP]
> To register the pre-commit hooks, use:
>
> ```shell
> uv run pre-commit install
> ```

## Linting code

This project uses several tools to lint and format various language syntaxes:

| Tool | Description | Managed by |
|:-----|:------------|:-----------|
| `clippy` | For linting Rust sources | [rustup] |
| `rustfmt` | For formatting Rust sources | [rustup] |
| `clang-format` | For formatting C++ sources | [Independently installed][clang-format]; v14 is expected. |
| `ruff` | For formatting and linting Python sources | [uv] |
| `oxlint` | For linting Javascript/Typescript sources | [yarn] |
| `prettier` | For formatting Javascript/Typescript sources | [yarn] |

All of these tools can be invoked with a [nur] command:

```shell
nur lint
```

## Running tests

This project uses unit tests to ensure expected behavior.
Each language employs its own testing harness.

### Rust tests

Code coverage is calculated only from the `mk-pass` Rust crate/library.

First ensure that the following cargo-managed binaries are installed:

- [cargo-llvm-cov]
- [cargo-nextest]

Use [nur] to run the Rust tests:

```sh
nur test
```

Additional options are shown and documented with `nur test -h`.

> [!TIP]
> Different test profiles are still defined in .config/nextest.toml.
> The above command uses the "default" profile, but to mimic the CI, use:
>
> ```sh
> nur test --profile ci
> ```

To generate a coverage report:

```sh
nur test llvm-cov --open
```

The `--open` part is optional.
It opens the built coverage report in your default browser.

> [!TIP]
> Coverage data is uploaded to codecov using a lcov.info file.
> A lcov.info file can be generated with
>
> ```sh
> nur test lcov
> ```

### Python tests

For the Python binding, `pytest` is used, but
the binding needs to be installed (and up-to-date) first.
This can be done using the following [nur] command:

```shell
nur test py
```

Additional options are shown and documented with `nur test py -h`.

### Javascript/Typescript tests

For the Node.js binding, `ava` is used, but
the binding needs to be built (and up-to-date) first.
This can be done using the following [nur] command:

```shell
nur test js
```

Additional options are shown and documented with `nur test js -h`.

### C++ tests

For the C++ binding, `CMake` and `CTest` are used, but
the binding's generated header file needs to be up-to-date first.
This can be done using the following [nur] command:

```shell
nur test cpp
```

Additional options are shown and documented with `nur test cpp -h`.

> [!NOTE]
> This project uses [corrosion] to integrate the Rust sources with CMake.
> In this project, CMake is configured to automatically download [corrosion]
> if it is not discovered/installed on the system.

[corrosion]: https://github.com/corrosion-rs/corrosion

## Generating docs

This project's documentation is comprised of HTML output from several different documentation tools.

| Tool | Description | Managed by |
|:-----|:------------|:-----------|
| `rustdoc` | For documenting the Rust API | [rustup] |
| `doxygen` | For documenting the C++ API | [Independently installed][doxygen] (recommend v1.14.0) |
| `typedoc` | For documenting the Javascript/Typescript API | [yarn] |
| `mkdocs` | For wrapping the above tools' output (and generating the Python API documentation) into a single site | [uv] which also manages `mkdocs` plugins/theme |

[doxygen]: https://www.doxygen.nl/

To view the docs locally, use

```sh
nur docs --open
```

The `--open` part is optional. It opens the built docs in your default browser.

To simply build the docs without spawning a local server (as done in CI),
the `--built` flag can be used:

```sh
nur docs --build
```

Additional options are shown and documented with `nur docs -h`.
