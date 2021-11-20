test: ## Run tests with Cargo (default features)
	cargo test

test-all: ## Run all tests
	cargo test --all-features --release
	./tests/seed.sh
