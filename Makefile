RUST_BACKTRACE ?= 0

setaf6 = $(shell tput setaf 6)
sgr0 = $(shell tput sgr0)

.PHONY: run-all test-libs benchmark-all test-and-run-% benchmark-% web-dev docs
.SECONDARY:

#
# Files downloaded from the AoC website.
#

.session:
	@echo "Please create a file named .session containing your session cookie." >&2
	@exit 1

inputs/%.txt: bin = $(subst inputs/,,$(subst .txt,,$@))
inputs/%.txt: nameparts = $(subst -, ,${bin})
inputs/%.txt: year = $(word 1,${nameparts})
inputs/%.txt: day = $(patsubst 0%,%,$(word 2,${nameparts}))
inputs/%.txt: .session
	@echo "$(setaf6)>>>>> Downloading input for ${bin} <<<<<$(sgr0)"
	@mkdir -p inputs
	@curl \
		-H "Cookie: session=$$(cat .session)" \
		--fail \
		--output $@ \
		"https://adventofcode.com/20${year}/day/${day}/input"

# Whenever this target is run this shell command will first be executed, altering the timestamp of the tracker file. If this causes the tracker file to be newer than the json file itself this will cause the it to be considered out-of-date and to be re-downloaded; otherwise it will be considered up-to-date and skipped. In effect this means the json file will be updated if it's been longer than the time passed to touch since it was last updated.
.leaderboard.json: $(shell touch -d '-1 hour' .leaderboard.json.timestamp-tracker)
.leaderboard.json: year = $(shell echo $$(( $$(date +'%Y') - $$([ $$(date +'%m') -eq 12 ] && echo 0 || echo 1) )))
.leaderboard.json: .session .leaderboard.json.timestamp-tracker
	@if [ -z "$$LEADERBOARD_ID" ]; then \
		echo >&2 "Please set the LEADERBOARD_ID environment variable."; \
		exit 1; \
	fi

	@echo "$(setaf6)>>>>> Downloading leaderboard json for ${year} <<<<<$(sgr0)"
	@curl \
		-H "Cookie: session=$$(cat .session)" \
		--fail \
		--output $@ \
		"https://adventofcode.com/${year}/leaderboard/private/view/$$LEADERBOARD_ID.json"

#
# Basic run/test commands.
#

run-all:
	@cargo build --release --bin aoc
	@cargo run --release --bin aoc --quiet

run-%:
	@cargo build --release --bin aoc
	@cargo run --release --bin aoc --quiet -- --only $(subst run-,,$@)

test-libs:
	@cargo nextest run --lib --no-fail-fast --cargo-quiet
	@cargo test --doc

test-and-run-%: bin = $(subst test-and-run-,,$@)
test-and-run-%: inputs/%.txt
	@echo "$(setaf6)>>>>> Testing ${bin} <<<<<$(sgr0)"
	@cargo nextest run --lib --bin ${bin} --no-fail-fast --cargo-quiet --status-level fail

	@echo "$(setaf6)>>>>> Running ${bin} <<<<<$(sgr0)"
	@cargo build --bin ${bin} --release --quiet
	@cargo run --bin ${bin} --release --quiet

#
# Documentation.
#

STDLIB = $(shell rustc --print sysroot)/lib/rustlib/src/rust/library
target/stdlib: ${STDLIB}
	@echo "Copying standard library source from ${STDLIB}..."
	@mkdir -p target/stdlib
	@rsync --recursive --copy-links --no-owner --no-group --no-perms --chmod=+w "${STDLIB}/" "$@/"

STDLIB_TARGETS = $(foreach dep,std core,target/doc-parts/stdlib/${dep})
target/doc-parts/stdlib/%: name = $(notdir $@)
target/doc-parts/stdlib/%: target/stdlib
	@echo "Building docs for ${name}..."
	@rm -rf target/stdlib/target/doc
	@RUSTDOCFLAGS="-Z unstable-options --merge none --parts-out-dir $$PWD/$@" \
	 cargo -Z unstable-options -C target/stdlib/${name} doc
	@rsync -r target/stdlib/target/doc/ "$@/"

DEP_TARGETS = $(foreach dep,$(shell cargo tree --depth 1 -e normal --prefix none | cut -d' ' -f1),target/doc-parts/dep/$(dep))
target/doc-parts/dep/%: name = $(notdir $@)
target/doc-parts/dep/%: version = $(shell cargo tree --depth 1 -e normal --prefix none | grep -E "^${name} " | cut -d' ' -f2 | sed 's/^v//')
target/doc-parts/dep/%: Cargo.toml Cargo.lock
	@echo "Building docs for ${name}..."
	@rm -rf target/doc
	@RUSTDOCFLAGS="-Z unstable-options --merge none --parts-out-dir $$PWD/$@" \
	 cargo -Z unstable-options doc --lib --no-deps -p "${name}@${version}"
	@rsync -r target/doc/ "$@/"
target/doc-parts/dep/aoc: $(shell find src -type f -print)
target/doc-parts/dep/aoc_runner: $(shell find aoc_runner aoc_runner_derive -type f -print)

docs: ${STDLIB_TARGETS} ${DEP_TARGETS}
	@echo "Building combined docs..."
	@rsync -r $(foreach dep,${STDLIB_TARGETS} ${DEP_TARGETS},${dep}/) target/doc/
	@RUSTDOCFLAGS="-Z unstable-options --merge finalize $$(printf -- "--include-parts-dir $$PWD/%s " ${STDLIB_TARGETS} ${DEP_TARGETS})" \
	  cargo doc --lib --no-deps

#
# Benchmarking.
#

benchmark-all:
	@cargo bench --bench main --features bench --quiet -- --save-baseline current
	@critcmp baseline current

benchmark-%: bin = $(subst benchmark-,,$@)
benchmark-%: test-and-run-% inputs/%.txt
	@echo "$(setaf6)>>>>> Benchmarking ${bin} <<<<<$(sgr0)"
	@cargo bench --bench main --features bench --quiet -- --only ${bin} --save-baseline current
	@critcmp baseline current --filter ${bin}

benchmark-set-baseline-all:
	@echo "$(setaf6)>>>>> Updating benchmark baselines <<<<<$(sgr0)"
	@cargo bench --bench main --features bench --quiet -- --save-baseline baseline

benchmark-set-baseline-%: bin = $(subst benchmark-set-baseline-,,$@)
benchmark-set-baseline-%: inputs/%.txt
	@echo "$(setaf6)>>>>> Updating benchmark baseline for ${bin} <<<<<$(sgr0)"
	@cargo bench --bench main --features bench --quiet -- --only ${bin} --save-baseline baseline

#
# Leaderboard.
#

leaderboard: year = $(shell echo $$(( $$(date +'%Y') - $$([ $$(date +'%m') -eq 12 ] && echo 0 || echo 1) )))
leaderboard: .leaderboard.json
	@echo "$(setaf6)>>>>> Processing leaderboard json for ${year} <<<<<$(sgr0)"
	@cargo run --quiet --bin leaderboard --features leaderboard -- .leaderboard.json

#
# Web version.
#

wasm/pkg: $(shell find src web/src -type f -print)
	@rm -rf wasm/pkg
	@wasm-pack build ./wasm --target web

web-dev: wasm/pkg
	@( true \
		&& cd web \
		&& npm install \
		&& npm run start \
	)

web/dist: wasm/pkg
	@( true \
		cd web \
		&& npm install \
		&& rm -rf dist/ \
		&& npm run build \
	)
