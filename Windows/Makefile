#!make
PROJECTS := Auto-spell-checker Auto-spell-checker-installer Auto-spell-checker-patcher

all: ${PROJECTS}

.PHONY: ${PROJECTS}
${PROJECTS}:
	@echo "Building $@ in release mode..."
	@cd $@ && cargo build --release && cd ..