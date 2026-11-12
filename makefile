ALL: build

build:
	cargo build

sound-test: build
	./target/debug/chirp -e -f assets/test/7-beep.ch8

benchmark: build
	./target/debug/chirp -e -f assets/test/benchmark.ch8

tetris: build
	./target/debug/chirp -e -f assets/test/tetris.ch8

keypad-test: build
	./target/debug/chirp -e -f assets/test/6-keypad.ch8

quirks-test: build
	./target/debug/chirp -e -f assets/test/5-quirks.ch8
