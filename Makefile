RUST_BACKTRACE ?= 0

setaf1 = $(shell tput setaf 1)
setaf6 = $(shell tput setaf 6)
sgr0 = $(shell tput sgr0)

.PHONY: run-all test-libs benchmark-all test-and-run-% visualize-% benchmark-% web-dev docs
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

test-libs: ignore=nix/store|puzzle_runner|puzzle_wasm
test-libs: ignore_puzzles=aoc
test-libs:
	@cargo llvm-cov clean --workspace
	@cargo llvm-cov --no-report nextest --lib --no-fail-fast --cargo-quiet
	@cargo llvm-cov --no-report --doc --no-fail-fast

	@cmd="$$( \
		cargo llvm-cov report --doctests --ignore-filename-regex '${ignore}|${ignore_puzzles}' --color always -v 2>&1 \
		| grep -oE 'Running\S* `.*`' \
		| tail -n1 \
		| cut -d '`' -f2 \
	 )" \
	 && eval "$$cmd" --show-region-summary=false --show-branch-summary=false
	@LLVM_COV_FLAGS='--show-directory-coverage' \
	 cargo llvm-cov report --doctests --ignore-filename-regex '${ignore}' --html
	@cargo llvm-cov report --doctests --ignore-filename-regex '${ignore}' --lcov --output-path target/llvm-cov/lcov

test-and-run-%: bin = $(subst test-and-run-,,$@)
test-and-run-%: inputs/%.txt
	@echo "$(setaf6)>>>>> Testing ${bin} <<<<<$(sgr0)"
	@cargo nextest run --lib --bin ${bin} --no-fail-fast --cargo-quiet --status-level fail

	@echo "$(setaf6)>>>>> Running ${bin} <<<<<$(sgr0)"
	@cargo build --bin ${bin} --release --quiet
	@cargo run --bin ${bin} --release --quiet

visualize-%: bin = $(subst visualize-,,$@)
visualize-%: inputs/%.txt
	@if ! grep -qxF '#[puzzle_runner::visual]' "aoc/src/bin/${bin}.rs"; then \
		>&2 echo "$(setaf1)${bin} doesn't have any visualizations.$(sgr0)"; \
		exit 1; \
	fi
	@echo "$(setaf6)>>>>> Visualizing ${bin} <<<<<$(sgr0)"
	@cargo run --bin ${bin} --release --quiet --features visual

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

DEP_TARGETS = $(foreach dep,$(shell cargo tree --package aoc --depth 1 -e normal --prefix none | cut -d' ' -f1),target/doc-parts/dep/$(dep))
target/doc-parts/dep/%: name = $(notdir $@)
target/doc-parts/dep/%: version = $(shell cargo tree --package aoc --depth 1 -e normal --prefix none | grep -E "^${name} " | cut -d' ' -f2 | sed 's/^v//')
target/doc-parts/dep/%: Cargo.toml Cargo.lock katex.html
	@echo "Building docs for ${name}..."
	@rm -rf target/doc
	@RUSTDOCFLAGS="--html-in-header $$PWD/katex.html -Z unstable-options --merge none --parts-out-dir $$PWD/$@" \
	 cargo -Z unstable-options doc --lib --no-deps -p "${name}@${version}"
	@rsync -r target/doc/ "$@/"
target/doc-parts/dep/aoc: $(shell find aoc -type f -print)
target/doc-parts/dep/puzzle_lib: $(shell find puzzle_lib -type f -print)
target/doc-parts/dep/puzzle_runner: $(shell find puzzle_runner puzzle_runner_derive -type f -print)

docs: ${STDLIB_TARGETS} ${DEP_TARGETS} katex.html
	@echo "Building combined docs..."
	@rsync -r $(foreach dep,${STDLIB_TARGETS} ${DEP_TARGETS},${dep}/) target/doc/
	@RUSTDOCFLAGS="--enable-index-page --html-in-header $$PWD/katex.html -Z unstable-options --merge finalize $$(printf -- "--include-parts-dir $$PWD/%s " ${STDLIB_TARGETS} ${DEP_TARGETS})" \
	 cargo doc --lib --no-deps

#
# Benchmarking & profiling.
#

benchmark-%: bin = $(subst benchmark-,,$@)
benchmark-%: test-and-run-% inputs/%.txt
	@echo "$(setaf6)>>>>> Benchmarking ${bin} <<<<<$(sgr0)"
	@cargo bench --bench main --features bench --quiet -- --only ${bin} --save-baseline current
	@critcmp baseline current --filter ${bin}

benchmark-set-baseline-%: bin = $(subst benchmark-set-baseline-,,$@)
benchmark-set-baseline-%: test-and-run-% inputs/%.txt
	@echo "$(setaf6)>>>>> Updating benchmark baseline for ${bin} <<<<<$(sgr0)"
	@cargo bench --bench main --features bench --quiet -- --only ${bin} --save-baseline baseline

profile-%: bin = $(subst profile-,,$@)
profile-%: test-and-run-% inputs/%.txt
	@echo "$(setaf6)>>>>> Profileing ${bin} <<<<<$(sgr0)"
	@cargo bench --bench main --features bench --quiet -- --only ${bin} --profile-time 15 --profile-name current
	@for f in target/criterion/${bin}_*/profile/current.pb; do \
		name="$${f%/profile/current.pb}"; \
		name="$${name##*/}"; \
		name="$${name//_/ }"; \
		pprofme upload "$$f" --description="$$name @ $$(stat --format '%y' "$$f")"; \
	done

profile-set-baseline-%: bin = $(subst profile-set-baseline-,,$@)
profile-set-baseline-%: test-and-run-% inputs/%.txt
	@echo "$(setaf6)>>>>> Updating profile baseline for ${bin} <<<<<$(sgr0)"
	@cargo bench --bench main --features bench --quiet -- --only ${bin} --profile-time 15 --profile-name baseline
	@for f in target/criterion/${bin}_*/profile/baseline.pb; do \
		name="$${f%/profile/baseline.pb}"; \
		name="$${name##*/}"; \
		name="$${name//_/ }"; \
		pprofme upload "$$f" --description="$$name @ $$(stat --format '%y' "$$f") (baseline)"; \
	done

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

target/debug/wasm-pkg: flags = --dev
target/release/wasm-pkg: flags = --release
target/%/wasm-pkg: $(shell find aoc puzzle_wasm/Cargo.toml puzzle_wasm/src web/src -type f -print)
	@rm -rf $@
	@wasm-pack build ./puzzle_wasm --target web --out-dir "$$PWD/$@" ${flags} 

web-dev: target/debug/wasm-pkg
	@ln -sfT "$$PWD/$<" web/puzzle_wasm
	@( true \
		&& cd web \
		&& npm install \
		&& npm run start \
	)

target/release/web: target/release/wasm-pkg
	@rm -rf $@
	@ln -sfT "$$PWD/$<" web/puzzle_wasm
	@( true \
		&& cd web \
		&& npm install \
		&& npm run build \
	)
