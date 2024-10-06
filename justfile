set dotenv-load

default:
	cargo-watch -x check  -s 'cargo loco start'

clear:
	clear

up: clear
	docker compose up -d
down: clear
	docker compose down

build: clear
	docker buildx build --platform linux/amd64 -t dailynotes:latest .
