.PHONY: all clean dev

all:
	wasm-pack build --target web
dev:
	python3 -m http.server
clean:
	rm -rf pkg
