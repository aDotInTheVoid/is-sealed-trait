all: simple_public.json # TODO: glob

%.json: %.rs
	rustdoc +nightly --edition 2021 --crate-type lib $< --output-format json -Z unstable-options --out-dir .

.PHONY: clean
clean:
	rm -f *.json