sound-test:
	cargo build
	./target/debug/chirp -e assets/test/7-beep.ch8

benchmark:
	cargo build
	./target/debug/chirp -e assets/test/benchmark.ch8
