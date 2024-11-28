RUST_BACKTRACE ?= 0

setaf6 = $(shell tput setaf 6)
sgr0 = $(shell tput sgr0)

.PHONY: run-all test-libs benchmark-all test-and-run-% benchmark-% web-dev
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
		"https://adventofcode.com/$(date +'%Y')/leaderboard/private/view/${LEADERBOARD_ID}.json"

#
# Basic run/test commands.
#

run-all:
	@cargo run --release --bin aoc --quiet

run-%:
	@echo cargo run --release --bin aoc --quiet --only $(subst run-,,$@)

test-libs:
	@cargo nextest run --lib --no-fail-fast --cargo-quiet

test-and-run-%: bin = $(subst test-and-run-,,$@)
test-and-run-%: inputs/%.txt
	@echo "$(setaf6)>>>>> Testing ${bin} <<<<<$(sgr0)"
	@cargo nextest run --lib --bin ${bin} --no-fail-fast --cargo-quiet --status-level fail

	@echo "$(setaf6)>>>>> Running ${bin} <<<<<$(sgr0)"
	@cargo run --bin ${bin} --release --quiet

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

leaderboard: .leaderboard.json
	@echo "$(setaf6)>>>>> Processing leaderboard json <<<<<$(sgr0)"
	@cargo run --quiet --bin leaderboard --features leaderboard -- .leaderboard.json

#
# Web version.
#

wasm/pkg: $(wildcard wasm/src/*)
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
