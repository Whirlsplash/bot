# -------------
# | Variables |
# -------------
FLAGS = -d -t # These are just generally useful for development.

# ------------
# | Wrappers |
# ------------
fmt:
	cargo fmt

check:
	cargo check

# -------------
# | Executors |
# -------------
checkf: fmt check

run: checkf
	cargo run
