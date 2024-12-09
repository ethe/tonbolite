.DEFAULT_GOAL := wasm

mode ?= debug

rs:
ifeq ($(mode), release)
	cargo +nightly build --release --target wasm32-unknown-emscripten -p sqlite-extension
else
	cargo +nightly build --target wasm32-unknown-emscripten -p sqlite-extension
endif

sqlite3.c:
	cd sqlite && ./configure --enable-all && make sqlite3.c

wasm:
ifeq ($(mode), release)
	cd sqlite/ext/wasm && make clean && make oz
else
	cd sqlite/ext/wasm && make clean && make
endif

clean:
	cargo clean
	cd sqlite && make clean
	cd sqlite/ext/wasm && make clean

all: clean rs sqlite3.c wasm

web:
	cd sqlite/ext/wasm && python ../../../http-debugger.py --bind localhost
