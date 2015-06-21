Trie Benchmarks
====

Benchmarks of Trie-like data-structures in Rust, Go and Haskell.

The Makefile facilitates the running of benchmarks.

```shell
# Run all.
make bench

# Run one.
make {go,rust,haskell}-bench

# Use a different input file.
export TEST_FILE=`pwd`/data/1984.txt
make bench
```

Go benchmarks can be found [here](https://github.com/michaelsproul/go_trie_benchmarks).

# Requirements

* Nightly Rust compiler (rustc) and package manager (Cargo).
* Haskell compiler (ghc) and build tool (cabal).
* Go build tool (go).
