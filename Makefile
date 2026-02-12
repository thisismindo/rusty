#!make
project-name = rusty

up:
	@echo Start $(project-name) API Service: && \
	docker compose up -d
down:
	@echo Shut down $(project-name) API Service: && \
	docker compose down
docker-build:
	@echo Build $(project-name) API Service docker image: && \
	docker build --no-cache -t rusty .
local-build:
	@echo Build $(project-name) API Service local image: && \
	cargo clean && cargo build
reset:
	@echo [WARNING] Reset $(project-name) project: && \
	docker compose down --rmi all --remove-orphans
	docker volume prune
	docker image prune
	sudo rm -rf mysql-db-primary
setup-db:
	@echo Setup $(project-name) DB: && \
	./platform/bin/db-setup.sh

lint:
	@echo Running clippy lints on $(project-name): && \
	cargo clippy --all-targets --all-features -- -D warnings

lint-fix:
	@echo Fixing clippy warnings on $(project-name): && \
	cargo clippy --all-targets --all-features --fix --allow-dirty

fmt:
	@echo Formatting $(project-name) code: && \
	cargo fmt

fmt-check:
	@echo Checking $(project-name) formatting: && \
	cargo fmt -- --check

check:
	@echo Running all checks on $(project-name): && \
	cargo fmt -- --check && \
	cargo clippy --all-targets --all-features -- -D warnings && \
	cargo test

test:
	@echo Running tests on $(project-name): && \
	cargo test

test-verbose:
	@echo Running tests on $(project-name) with verbose output: && \
	cargo test -- --nocapture

logs:
	@docker compose logs -f

logs-nginx:
	@docker compose logs -f nginx

logs-api:
	@docker compose logs -f rusty-1 rusty-2 rusty-3

status:
	@docker compose ps

build-all:
	@echo Building all $(project-name) images: && \
	docker compose build --no-cache
