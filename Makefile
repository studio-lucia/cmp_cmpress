CARGO := cargo
INSTALL := install
PREFIX := /usr/local
bindir := $(PREFIX)/bin

target/release/cmp_cmpress: src/main.rs
	$(CARGO) build --release

.phony: all clean install

all: target/release/cmp_cmpress

clean:
	rm -rf target/release

install: all
	$(INSTALL) -d $(bindir)
	$(INSTALL) target/release/cmp_cmpress $(bindir)
