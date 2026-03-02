.PHONY: all build assets test lint coverage clean env fuzz fuzz-all fuzz-stress-all stress fuzz-lint report $(FUZZ_TARGETS) $(addprefix fuzz-,$(FUZZ_TARGETS)) $(addprefix fuzz-stress-,$(FUZZ_TARGETS)) $(addprefix fuzz-cmin-,$(FUZZ_TARGETS))

FUZZ_TARGETS := token_payload verify_request submissions
FUZZ_FLAGS ?= -runs=10000
HOURS ?= 2

all: lint test build fuzz

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

fuzz-lint:
	@cd fuzz && cargo clippy --all-features --all-targets -- -D warnings -D clippy::pedantic -D clippy::nursery -D clippy::all

report: fuzz-cmin
	@cd fuzz && ./sync_report.sh

lint: fuzz-lint
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

$(FUZZ_TARGETS):
	cd fuzz && RUSTFLAGS="-C lto=off -C strip=none" cargo +nightly fuzz run $@ -- $(FUZZ_FLAGS)

fuzz: fuzz-all

fuzz-all: $(FUZZ_TARGETS)

$(addprefix fuzz-,$(FUZZ_TARGETS)): fuzz-%:
	cd fuzz && RUSTFLAGS="-C lto=off -C strip=none" cargo +nightly fuzz run $* -- $(FUZZ_FLAGS)

fuzz-stress-all:
	@STRESS_SEC=$$(($(HOURS) * 3600)); \
	$(MAKE) fuzz-all FUZZ_FLAGS="-max_total_time=$$STRESS_SEC"

stress: fuzz-stress-all

fuzz-cmin-all: $(addprefix fuzz-cmin-,$(FUZZ_TARGETS))

fuzz-cmin: fuzz-cmin-all

$(addprefix fuzz-cmin-,$(FUZZ_TARGETS)): fuzz-cmin-%:
	cd fuzz && RUSTFLAGS="-C lto=off -C strip=none" cargo +nightly fuzz cmin $*

$(addprefix fuzz-stress-,$(FUZZ_TARGETS)): fuzz-stress-%:
	@STRESS_SEC=$$(($(HOURS) * 3600)); \
	cd fuzz && RUSTFLAGS="-C lto=off -C strip=none" cargo +nightly fuzz run $* -- -max_total_time=$$STRESS_SEC
