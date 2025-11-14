SHELL := /bin/zsh

run:
	set -a && source .env && env cargo run

deploy:
	/bin/bash deploy.sh
