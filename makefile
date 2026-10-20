sound-test:
	cargo build
	./target/debug/chirp -e assets/test/7-beep.ch8
