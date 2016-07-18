all: bin/rusty_kilo
	
run: all
	clear
	bin/rusty_kilo kilo.c 2> bin/log.txt
	clear
	
bin/kilo.o: kilo.c
	mkdir -p bin
	clang -o bin/kilo.o -c kilo.c -Wall -W -pedantic -std=c99

target/debug/librusty_kilo.a: src/lib.rs Cargo.toml src/append_buffer.rs
	cargo build

bin/rusty_kilo: bin/kilo.o target/debug/librusty_kilo.a
	clang -o bin/rusty_kilo bin/kilo.o -Ltarget/debug/ -lrusty_kilo -lSystem -lc -lm

clean:
	rm -rf bin
	cargo clean
