#![feature(test, io, fs)]

extern crate "test" as std_test;
extern crate radix_trie;
extern crate sequence_trie;

#[cfg(test)]
mod benchmark;
