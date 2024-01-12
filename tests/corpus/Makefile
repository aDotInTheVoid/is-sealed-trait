jsons := $(patsubst %.rs,%.json,$(wildcard *.rs))

all: $(jsons)

%.json: %.rs
	rustdoc +nightly --edition 2021 --crate-type lib $< --output-format json -Z unstable-options --out-dir . --document-private-items --document-hidden-items

.PHONY: clean
clean:
	rm -f *.json