//! [![Crate](https://img.shields.io/crates/v/sized_integer.svg)](https://crates.io/crates/sized_integer)
//!
//! A simple library for reading differently-sized integers and floats.
//!
//! While writing h2gb, I needed a way to dynamically read integers from a
//! Vec of u8 bytes. Libs like `byteorder` and `io::Cursor` nearly has the
//! right functionality, but weren't quite flexible enough.
//!
//! This library wraps / uses those modules to simplify reading arbitrary values
//! from a cursor, and storing / displaying them with user-controlled settings.
//!
//! # Example
//!
//! TODO
#![allow(dead_code)] // TODO: Disable this

// use serde::{Serialize, Deserialize};

pub type Context<'a> = std::io::Cursor<&'a Vec<u8>>;

pub mod display_options;

pub mod sized_integer;
// use sized_integer::SizedInteger;

pub mod sized_float;
