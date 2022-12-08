all: build run

build:
	cargo build

run:
	sudo ./target/debug/ant-d