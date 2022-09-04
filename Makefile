.PHONY: build

up:
	cargo run -p migration -- up

clean:
	rm -f ./build/*

build: clean
	docker build -t tyorka-shop .
	docker save tyorka-shop:latest | gzip > build/app.tar.gz
