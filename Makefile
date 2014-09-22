RUSTC = rustc
RUST_FLAGS = -L build

all: indigestion

libindigestion = build/libindigestion.timestamp

indigestion: src/indigestion.rs $(libindigestion)
	$(RUSTC) $(RUST_FLAGS) -o $@ $<

$(libindigestion): Makefile $(wildcard src/*.rs)
	mkdir -p build
	$(RUSTC) $(RUST_FLAGS) src/lib.rs --out-dir=build
	@touch $@

clean:
	rm -f indigestion
	rm -rf build
.PHONY: clean
