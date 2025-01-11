test: FORCE
	cargo test

watch-tests: FORCE
	cargo watch -w src -x test


FORCE:
