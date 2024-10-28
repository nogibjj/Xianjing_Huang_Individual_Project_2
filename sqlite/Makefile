# Makefile for Rust Project


all: check build format lint test release

check:
	cargo check

build:
	cargo build

format:
	cargo fmt

lint:
	cargo clippy

test:
	cargo test

release:
	cargo build --release

# Custom tasks

extract:
	cargo run -- -e

create:
	cargo run -- -c table1

load:
	cargo run -- -l table1 data/customer_new.csv

query:
	cargo run -- -q table1

insert:
	cargo run -- -i table1 11 Remi female Durham

update:
	cargo run -- -u table1 11 Remi female 'Los Angeles'

delete:
	cargo run -- -x table1 11

drop:
	cargo run -- -d table1