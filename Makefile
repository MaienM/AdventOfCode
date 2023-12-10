RUST_BACKTRACE ?= 0

setaf6 = $(shell tput setaf 6)
sgr0 = $(shell tput sgr0)

.PHONY: run-all test-libs test-and-run-day%
.SECONDARY:

run-all:
	@cargo run --release --bin aoc --quiet

test-libs:
	@cargo nextest run --lib --no-fail-fast --cargo-quiet

.session:
	@echo "Please create a file named .session containing your session cookie." >&2
	@exit 1

inputs/day%.txt: day = $(subst inputs/,,$(subst .txt,,$@))
inputs/day%.txt: .session
	@echo "$(setaf6)>>>>> Downloading input for ${day} <<<<<$(sgr0)"
	@curl \
		-H "Cookie: session=$$(cat .session)" \
		--fail \
		--output inputs/${day}.txt \
		"https://adventofcode.com/2023/day/$(subst day,,$(subst day0,,${day}))/input"

test-and-run-day%: day = $(subst test-and-run-,,$@)
test-and-run-day%: inputs/day%.txt
	@echo "$(setaf6)>>>>> Testing ${day} <<<<<$(sgr0)"
	@cargo nextest run --lib --bin ${day} --no-fail-fast --cargo-quiet --status-level fail

	@echo "$(setaf6)>>>>> Running ${day} <<<<<$(sgr0)"
	@cargo run --bin ${day} --release --quiet

	@echo "$(setaf6)>>>>> Benchmarking ${day} <<<<<$(sgr0)"
	@cargo bench --bench main --quiet -- --only $(subst day,,${day}) --save-baseline current
	@critcmp baseline current --filter ${day}

benchmark-set-baseline-all:
	@echo "$(setaf6)>>>>> Updating benchmark baselines <<<<<$(sgr0)"
	@cargo bench --bench main --quiet -- --save-baseline baseline

benchmark-set-baseline-day%: day = $(subst benchmark-set-baseline-,,$@)
benchmark-set-baseline-day%: inputs/day%.txt
	@echo "$(setaf6)>>>>> Updating benchmark baseline for ${day} <<<<<$(sgr0)"
	@cargo bench --bench main --quiet -- --only $(subst day,,${day}) --save-baseline baseline
