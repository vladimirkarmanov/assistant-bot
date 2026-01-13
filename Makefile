SHELL := /bin/zsh

run:
	set -a && source .env && env cargo run

redis:
	set -a && source .env && docker compose -f docker-compose.dev.yml up --build redis

deploy:
	/bin/bash deploy.sh

clippy:
	cargo clippy

db-migrate:
	set -a && source .env && sqlx migrate run

test:
	set -a && source .env && cargo test
