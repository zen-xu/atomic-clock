SHELL := /bin/bash

.PHONY: help
help: ## This help message
	@echo -e "$$(grep -hE '^\S+:.*##' $(MAKEFILE_LIST) | sed -e 's/:.*##\s*/:/' -e 's/^\(.\+\):\(.*\)/\\x1b[36m\1\\x1b[m:\2/' | column -c2 -t -s :)"

.PHONY: build
build: dev-packages ## Builds Rust code and atomic_clock Python modules
	poetry run maturin build

.PHONY: build-release
build-release: dev-packages ## Build atomic_clock module in release mode
	poetry run maturin build --release

.PHONY: install
install: dev-packages ## Install atomic_clock module into current virtualenv
	poetry run maturin develop --release

.PHONY: publish
publish: ## Publish crate on Pypi
	poetry run maturin publish

.PHONY: clean
clean: ## Clean up build artifacts
	cargo clean

.PHONY: dev-packages
dev-packages: ## Install Python development packages for project
	poetry install

.PHONY: cargo-test
cargo-test: ## Run cargo tests only
	cargo test

.PHONY: test
test: cargo-test dev-packages install quicktest ## Intall atomic_clock module and run tests

.PHONY: quicktest
quicktest: ## Run tests on already installed atomic_clock module
	poetry run pytest tests
