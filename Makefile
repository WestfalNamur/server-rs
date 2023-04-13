# Queit watch, clear and recompile on changes in src/ and execute them
dev:
	cargo watch -q -c -w src/ -x run

# Similar to dev but for quick_dev
quick-test:
	clear && cargo test -q quick_dev -- --nocapture