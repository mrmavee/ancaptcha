.PHONY: all build assets test lint coverage clean env

all: lint test build

env:
	@if [ ! -f .env ]; then \
		echo "ANCAPTCHA_SECRET=$$(openssl rand -hex 32)" > .env; \
	fi

assets:
	@chmod +x compress.sh
	@./compress.sh

build: assets
	@cargo build --release --workspace

test: env
	@cargo nextest run --all-features --workspace

lint:
	@cargo fmt --all -- --check
	@cargo clippy --all-features --all-targets -- -D warnings -D clippy::pedantic -D clippy::nursery

coverage: env
	@mkdir -p assets/badges
	@cargo llvm-cov nextest --all-features --workspace --lcov --output-path lcov.info > /dev/null 2>&1
	@GEN_COV=$$(cargo llvm-cov --all-features --workspace --json | jq -r '.data[0].totals.lines.percent | floor'); \
	if [ "$$GEN_COV" -ge 90 ]; then COLOR="brightgreen"; \
	elif [ "$$GEN_COV" -ge 75 ]; then COLOR="yellow"; \
	else COLOR="red"; fi; \
	curl -s "https://img.shields.io/badge/coverage-$$GEN_COV%25-$$COLOR" > assets/badges/coverage.svg; \
	rsvg-convert assets/badges/coverage.svg > assets/badges/coverage.png; \
	rm assets/badges/coverage.svg
	@rm -f lcov.info

clean:
	@cargo clean
	@rm -f lcov.info
