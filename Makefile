TEST_FILE ?= $(shell pwd)/data/sun-rising.txt
GOPATH = $(shell pwd)/go
GO_REPO = "github.com/michaelsproul/go_trie_benchmarks"

bench: rust-bench haskell-bench go-bench

rust-bench:
	@echo "=== Rust Benchmarks ==="
	@export TEST_FILE=$(TEST_FILE)
	cd rust
	cargo bench
	@echo

haskell-bench:
	@echo "=== Haskell Benchmarks ==="
	@export TEST_FILE=$(TEST_FILE)
	cd haskell
	cabal sandbox init
	cabal install -j4 --dependencies-only
	cabal configure
	cabal run benchmark
	@echo

go-bench: $(GOPATH)
	@echo "=== Go Benchmarks ==="
	@export GOPATH=$(GOPATH)
	@export TEST_FILE=$(TEST_FILE)
	go get $(GO_REPO)
	cd go/src/$(GO_REPO)
	go get -t ./...
	go test -bench=.
	@echo

$(GOPATH):
	mkdir -p $(GOPATH)

clean: rust-clean haskell-clean go-clean

rust-clean:
	cd rust
	cargo clean
	rm Cargo.lock

haskell-clean:
	cd haskell
	cabal clean
	rm -rf cabal.sandbox.config .cabal-sandbox

go-clean:
	rm -rf go

.ONESHELL:
.PHONY: bench rust-bench haskell-bench go-bench clean rust-clean haskell-clean go-clean
