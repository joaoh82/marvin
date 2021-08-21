SHELL:=/bin/bash

# COLORS
GREEN  := $(shell tput -Txterm setaf 2)
YELLOW := $(shell tput -Txterm setaf 3)
WHITE  := $(shell tput -Txterm setaf 7)
RESET  := $(shell tput -Txterm sgr0)

LOCAL_KUBECTX ?= "docker-for-desktop"
DEV_KUBECTX="TO_BE_DEFINED"
STAGING_KUBECTX="TO_BE_DEFINED"

VERSION=dev
COMMIT=$(shell git rev-parse HEAD)
GITDIRTY=$(shell git diff --quiet || echo 'dirty')
REPOSITORY ?= "joaoh82"
PID := $(shell ps -A | grep marvin | awk '{print $$1}' | head -1)

GIT_BRANCH := $(shell git rev-parse --abbrev-ref HEAD)

TARGET_MAX_CHAR_NUM=25
## Show help
help:
	@echo ''
	@echo 'Usage:'
	@echo '  ${YELLOW}make${RESET} ${GREEN}<target>${RESET}'
	@echo ''
	@echo 'Targets:'
	@awk '/^[a-zA-Z\-\_0-9]+:/ { \
		helpMessage = match(lastLine, /^## (.*)/); \
		if (helpMessage) { \
			helpCommand = substr($$1, 0, index($$1, ":")-1); \
			helpMessage = substr(lastLine, RSTART + 3, RLENGTH); \
			printf "  ${YELLOW}%-$(TARGET_MAX_CHAR_NUM)s${RESET} ${GREEN}%s${RESET}\n", helpCommand, helpMessage; \
		} \
	} \
	{ lastLine = $$0 }' $(MAKEFILE_LIST)

.PHONY: kill-marvin
## Kills marvin daemon (needs to be run as root)
kill-marvin:
	@echo '${GREEN}Killing${RESET} ${YELLOW}Marvin${RESET} Application'
	@echo "Killing PID: ${PID}"
	@kill -9 ${PID}

.PHONY: build
## Builds the application
build:
	@echo '${GREEN}Building${RESET} ${YELLOW}Marvin${RESET} Application'
	@cargo build --release
	@mkdir -p artifacts
	@cp target/release/marvin artifacts/marvin

.PHONY: start-daemon
## Starts the daemon (needs to be run as root)
start-daemon:
	@echo '${GREEN}Starting${RESET} ${YELLOW}Marvin${RESET} Application'
	@ ./artifacts/marvin

.PHONY: run-tests
## Runs unit tests
run-tests:
	@echo '${GREEN}Running${RESET} ${YELLOW}Unit Tests${RESET} for Service'
	@cargo test
