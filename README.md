# Parrots - ParrotLib for Rust

Parrots is a port of the ParrotLib library that provides high-level tools to lower-level languages.

## What is ParrotLib?

ParrotLib is an intermediate library that is used when compiling ***Parrot*** source code.

The Parrot compiler ([parrotc](https://crates.io/crates/parrotc)) converts `.p` source files into a chosen lower-level language (in this case [Rust](https://rust-lang.org)).

While used as a tool to convert other languages into Rust, it is also useful as a simple library to help get stuff done with Rust.

## Installation

To install Parrots, simply add this line to your `[dependencies]` list (in Cargo.toml):

``` toml
parrots = "VERSION"
```

Where `VERSION` is the current Parrots version.

## Parrot compiler

To get started writing Parrot code, see [parrotc](https://crates.io/crates/parrotc)
