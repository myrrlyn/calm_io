################################################################################
#                                   Justfile                                   #
#                                                                              #
# Set of routines to execute for development work.                             #
################################################################################

# Run the benchmarks. Currently, this requires the nightly compiler series.
bench:
	cargo +nightly bench

# Build the project, after checking that it is valid.
build: check
	cargo build

# Runs the checker and linter.
check:
	cargo check
	cargo clippy

# Destroys build artifacts.
clean:
	cargo clean

# Documents the project, after checking that it is valid.
doc: check
	cargo doc

examples: test
	cargo run --example good_yes | head > /dev/null

# Runs a Justfile recipe on every change to the workspace.
loop action:
	cargo watch -s "just {{action}}"

# Runs the project under the Miri interpreter. This is currently nightly-only.
miri:
	cargo +nightly miri test

publish: test doc
	cargo package # no --allow-dirty this time
	cargo publish -p calmio_filters
	cargo publish

# Runs the test suite.
test: build
	cargo test
