ALL: build

build:
	cargo build

sound-test: build
	./target/debug/chirp -e assets/test/7-beep.ch8

benchmark: build
	./target/debug/chirp -e assets/test/benchmark.ch8

tetris: build
	./target/debug/chirp -e assets/test/tetris.ch8
