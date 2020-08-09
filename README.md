# Rust-template

![Build Status](https://github.com/zaszi/rust-template/workflows/Rust/badge.svg)

A project template for Rust, helping to structure your projects according to best practices and avoid doing the same setup time and again.

## Features

* Suitable for both a binary or library project
* Structured according to both Git and Rust best practices
    * Includes readme, license, gitignore and gitattributes
    * Extensive Cargo manifest
    * [Separation of concerns for binary projects](https://doc.rust-lang.org/stable/book/ch12-03-improving-error-handling-and-modularity.html?highlight=separation,concerns#separation-of-concerns-for-binary-projects)
* Multiplatform support - The template is compatible with Linux, MacOS and Windows
* WebAssembly support
    * Conditionally includes `console_error_panic_hook` improving browser error messages.
    * Conditionally includes `wee_alloc` for a smaller memory footprint.
    * Dedicated `README_WEB.md` for WebAssembly usage and building instructions

## Usage

To tailor the template to your specific needs, simply go over the following
checklist:

1. The template assumes a binary project by default. If you require a library project:
    1. Comment out Cargo.lock in `.gitignore`.
    1. Remove `src/main.rs`.
1. Update `Cargo.toml` with the correct information for your project.
1. Change the name inside the `LICENSE.md` file, or replace with a license of your choice.
1. Update this `README.md` and `README_WEB.md` (do not forget to replace URLs).


## Building

If desired, you can build Rust-template yourself. You will need a working `Rust` and `Cargo` setup. [Rustup](https://rustup.rs/) is the simplest way to set this up on either Windows, Mac or Linux.

Once the prerequisites have been installed, compilation on your native platform is as simple as running the following in a terminal:

```
cargo build --release
```

## WebAssembly

Rust-template is supports running as WebAssembly in the browser. See the [web readme](https://github.com/zaszi/rust-template/blob/master/README_WEB.md) for instructions.

## Contribution

Found a problem or have a suggestion? Feel free to open an issue.

## License

Rust-template itself is licensed under the [MIT license](https://github.com/zaszi/rust-template/blob/master/LICENSE.md) and includes this as the default project license.
