RUST_BACKTRACE ?= 0

setaf1 = $(shell tput setaf 1)
setaf6 = $(shell tput setaf 6)
sgr0 = $(shell tput sgr0)

.PHONY: test-libs web-dev docs confirm FORCE
.SECONDEXPANSION:

targets=$(foreach path,$(wildcard */src/bin/*),$(let crate bin,$(subst /src/bin/, ,${path}),$(subst -${crate},,$(subst .rs,,${crate}-${bin}))))

#
# Files downloaded from the AoC website.
#

.session:
	@echo "Please create a file named .session containing your session cookie." >&2
	@exit 1

inputs/aoc/%/input.txt: bin = $(patsubst inputs/aoc/%/input.txt,%,$@)
inputs/aoc/%/input.txt: nameparts = $(subst -, ,${bin})
inputs/aoc/%/input.txt: year = $(word 1,${nameparts})
inputs/aoc/%/input.txt: day = $(patsubst 0%,%,$(word 2,${nameparts}))
inputs/aoc/%/input.txt: .session
	@echo "$(setaf6)>>>>> Downloading input for ${bin} <<<<<$(sgr0)"
	@mkdir -p $(dir $@)
	@curl \
		-H "Cookie: session=$$(cat .session)" \
		--fail \
		--output $@ \
		"https://adventofcode.com/20${year}/day/${day}/input"

#
# Basic run/test commands.
#

run-all: run-aoc
	@true

run-%: target = $(subst run-,,$@)
run-%: crate = $(word 1,$(subst -, ,${target}))
run-%: bin = $(subst ${crate}-,,${target})
run-%: name = $(subst /${crate},,${crate}/${bin})
run-%: input = $(if $(subst ${crate},,${bin}),inputs/${crate}/${bin}/input.txt,)
run-%: FORCE $${input}
	@echo "$(setaf6)>>>>> Running ${name} <<<<<$(sgr0)"
	@./cargo-semiquiet.sh run --release --package ${crate} --bin ${bin}

test-and-run-%: target = $(subst test-and-run-,,$@)
test-and-run-%: crate = $(word 1,$(subst -, ,${target}))
test-and-run-%: bin = $(subst ${crate}-,,${target})
test-and-run-%: name = $(subst /${crate},,${crate}/${bin})
test-and-run-%: FORCE
	@echo "$(setaf6)>>>>> Testing ${name} <<<<<$(sgr0)"
	@./cargo-semiquiet.sh nextest run --workspace --exclude puzzle_wasm --lib --no-fail-fast --status-level fail
	@./cargo-semiquiet.sh nextest run --package ${crate} --lib --bin ${bin} --no-fail-fast --status-level fail --no-tests pass

	@make --no-print-directory run-${target}

test-libs: ignore=nix/store|puzzle_runner|puzzle_wasm
test-libs: ignore_puzzles=aoc
test-libs:
	@cargo llvm-cov clean --workspace
	@./cargo-semiquiet.sh llvm-cov --no-report nextest --lib --no-fail-fast
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

confirm:
	@find inputs -type f -name '*.pending' | while read -r file; do \
		target="$${file%.pending}"; \
		echo "Confirming $$target."; \
		mv "$$file" "$$target"; \
	done

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

__setup-benchmark-%: target = $(subst __setup-benchmark-,,$@)
__setup-benchmark-%: crate = $(word 1,$(subst -, ,${target}))
__setup-benchmark-%: bin = $(subst ${crate}-,,${target})
__setup-benchmark-%: source = ${crate}/src/bin/${bin}.rs
__setup-benchmark-%: ${source}
	@printf '%s.rs' "${bin}" > "${crate}/src/bin/bench-target"

benchmark-%: target = $(subst benchmark-,,$@)
benchmark-%: crate = $(word 1,$(subst -, ,${target}))
benchmark-%: bin = $(subst ${crate}-,,${target})
benchmark-%: test-and-run-% __setup-benchmark-%
	@echo "$(setaf6)>>>>> Benchmarking ${crate}/${bin} <<<<<$(sgr0)"
	@./cargo-semiquiet.sh bench --package ${crate} --bin bench --features bench -- --save-baseline current
	@critcmp baseline current --filter ${crate}/${bin}

benchmark-set-baseline-%: target = $(subst benchmark-set-baseline-,,$@)
benchmark-set-baseline-%: crate = $(word 1,$(subst -, ,${target}))
benchmark-set-baseline-%: bin = $(subst ${crate}-,,${target})
benchmark-set-baseline-%:
	@echo "$(setaf6)>>>>> Setting last benchmark for ${crate}/${bin} as baseline <<<<<$(sgr0)"
	@for d in target/criterion/${crate}_${bin}_*/current; do \
		rm -rf "$${d%/current}/baseline"; \
		mv "$$d" "$${d%/current}/baseline"; \
	done

profile-%: target = $(subst profile-,,$@)
profile-%: crate = $(word 1,$(subst -, ,${target}))
profile-%: bin = $(subst ${crate}-,,${target})
profile-%: test-and-run-% __setup-benchmark-%
	@echo "$(setaf6)>>>>> Profileing ${crate}/${bin} <<<<<$(sgr0)"
	@./cargo-semiquiet.sh bench --package ${crate} --bin bench --features bench -- --profile-time 15 --profile-name current
	@for f in target/criterion/${crate}_${bin}_*/profile/current.pb; do \
		name="$${f%/profile/current.pb}"; \
		name="$${name##*/}"; \
		name="$${name//_/ }"; \
		pprofme upload "$$f" --description="$$name @ $$(stat --format '%y' "$$f")"; \
	done

profile-set-baseline-%: target = $(subst profile-set-baseline-,,$@)
profile-set-baseline-%: crate = $(word 1,$(subst -, ,${target}))
profile-set-baseline-%: bin = $(subst ${crate}-,,${target})
profile-set-baseline-%:
	@echo "$(setaf6)>>>>> Setting last profile for ${bin} as baseline <<<<<$(sgr0)"
	@for f in target/criterion/${crate}_${bin}_*/profile/current.pb; do \
		mv "$$f" "$${f%/current.pb}/baseline.pb"; \
	done

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
