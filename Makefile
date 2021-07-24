default: dist
COMPILERS:=$(shell pwd)/.compilers
PATH:=$(COMPILERS)/rust/bin/:$(PATH)

help: # with thanks to Ben Rady
	@grep -E '^[0-9a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-20s\033[0m %s\n", $$1, $$2}'

export XZ_OPT=-1 -T 0

.PHONY: clean run haskell-support d-support rust-support compilers
.PHONY: dist demanglers gh-dist
demanglers: haskell-support d-support rust-support
d-support: compilers
	$(MAKE) -C d GDC=$(COMPILERS)/gdc/x86_64-pc-linux-gnu/bin/gdc

compilers:
	./get_compilers.sh

haskell-support: compilers
	$(MAKE) -C haskell GHC=$(COMPILERS)/ghc/bin/ghc

CARGO=$(COMPILERS)/rust/bin/cargo
rust/bin/rustfilt: rust/src/main.rs rust/Cargo.lock rust/Cargo.toml compilers
	cd rust && $(CARGO) build --release
rust-support: rust/bin/rustfilt

clean:  ## Cleans up everything
	rm -rf out
	[ -x $(CARGO) ] && cd rust && $(CARGO) clean
	$(MAKE) -C d clean
	$(MAKE) -C haskell clean

HASH := $(shell git rev-parse HEAD)
dist: demanglers  ## Creates a distribution
	rm -rf out/demanglers
	mkdir -p out/demanglers
	mkdir -p out/demanglers/d
	cp d/demangle out/demanglers/d/
	mkdir -p out/demanglers/haskell
	cp haskell/demangle out/demanglers/haskell
	ldd out/demanglers/haskell/demangle \
		| sed -n -e 's/.*=> \([^ ]*\) .*/\1/p' \
		| egrep -v '^/lib' \
		| xargs cp -t out/demanglers/haskell/
	cd rust && $(CARGO) install --path . --root $(shell pwd)/out/demanglers/rust --force
	echo ${HASH} > out/demanglers/git_hash

gh-dist: dist  ## Creates a distribution as if we were running on github CI
	echo "::set-output name=branch::$${GITHUB_REF#refs/heads/}"
	tar -Jcf /tmp/ce-build.tar.xz -C out demanglers
	rm -rf out/dist-bin
	mkdir -p out/dist-bin
	mv /tmp/ce-build.tar.xz out/dist-bin/gh-${GITHUB_RUN_NUMBER}.tar.xz
	echo ${HASH} > out/dist-bin/gh-${GITHUB_RUN_NUMBER}.txt
