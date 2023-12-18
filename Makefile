RUST_BACKTRACE ?= 0

setaf6 = $(shell tput setaf 6)
sgr0 = $(shell tput sgr0)

.PHONY: run-all test-libs benchmark-all test-and-run-day% benchmark-day% web-dev web-build
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
		--output $@ \
		"https://adventofcode.com/2023/day/$(subst day,,$(subst day0,,${day}))/input"

test-and-run-day%: day = $(subst test-and-run-,,$@)
test-and-run-day%: inputs/day%.txt
	@echo "$(setaf6)>>>>> Testing ${day} <<<<<$(sgr0)"
	@cargo nextest run --lib --bin ${day} --no-fail-fast --cargo-quiet --status-level fail

	@echo "$(setaf6)>>>>> Running ${day} <<<<<$(sgr0)"
	@cargo run --bin ${day} --release --quiet

benchmark-all:
	@cargo bench --bench main --features bench --quiet -- --save-baseline current
	@critcmp baseline current

benchmark-day%: day = $(subst benchmark-,,$@)
benchmark-day%: test-and-run-day% inputs/day%.txt
	@echo "$(setaf6)>>>>> Benchmarking ${day} <<<<<$(sgr0)"
	@cargo bench --bench main --features bench --quiet -- --only $(subst day,,${day}) --save-baseline current
	@critcmp baseline current --filter ${day}

benchmark-set-baseline-all:
	@echo "$(setaf6)>>>>> Updating benchmark baselines <<<<<$(sgr0)"
	@cargo bench --bench main --features bench --quiet -- --save-baseline baseline

benchmark-set-baseline-day%: day = $(subst benchmark-set-baseline-,,$@)
benchmark-set-baseline-day%: inputs/day%.txt
	@echo "$(setaf6)>>>>> Updating benchmark baseline for ${day} <<<<<$(sgr0)"
	@cargo bench --bench main --features bench --quiet -- --only $(subst day,,${day}) --save-baseline baseline

# Whenever this target is run this shell command will first be executed, altering the timestamp of the tracker file. If this causes the tracker file to be newer than the json file itself this will cause the it to be considered out-of-date and to be re-downloaded; otherwise it will be considered up-to-date and skipped. In effect this means the json file will be updated if it's been longer than the time passed to touch since it was last updated.
.leaderboard.json: $(shell touch -d '-1 hour' .leaderboard.json.timestamp-tracker)
.leaderboard.json: .session .leaderboard.json.timestamp-tracker
	@if [ -z "$LEADERBOARD_ID" ]; then \
		echo >&2 "Please set the LEADERBOARD_ID environment variable."; \
		exit 1; \
	fi

	@echo "$(setaf6)>>>>> Downloading leaderboard json <<<<<$(sgr0)"
	@curl \
		-H "Cookie: session=$$(cat .session)" \
		--fail \
		--output $@ \
		"https://adventofcode.com/2023/leaderboard/private/view/${LEADERBOARD_ID}.json"

leaderboard: .leaderboard.json
	@echo "$(setaf6)>>>>> Processing leaderboard json <<<<<$(sgr0)"
	@cargo run --quiet --bin leaderboard --features leaderboard -- .leaderboard.json

web-dev:
	@wasm-pack build ./wasm --target web -- --features debug
	@(cd web && npm install && npm run start)

web-build:
	@wasm-pack build ./wasm --target web -- --features debug
	@(cd web && npm install && rm -rf dist/ && npm run build)
