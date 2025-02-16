.PHONY: server client

server:
	cargo run --release --bin server

client:
	cargo run --release --bin client
