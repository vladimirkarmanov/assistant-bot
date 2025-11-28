SHELL := /bin/zsh

run:
	set -a && source .env && env cargo run

deploy:
	/bin/bash deploy.sh

clippy:
	cargo clippy

db-migrate:
	set -a && source .env && sqlx migrate run
