#!make
PROJECTS := Auto-spell-checker Auto-spell-checker-installer Auto-spell-checker-patcher

all: ${PROJECTS}

.PHONY: ${PROJECTS}
${PROJECTS}:
	@echo "Building Auto-spell-checker in release mode..."
	@cd Auto-spell-checker && cargo build --release && cd ..