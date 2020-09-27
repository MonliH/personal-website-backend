install:
	cd www && \
	npm i

build_frontend:
	cd www && \
	npm run build

build_backend:
	cd server && \
	cargo build --release --target-dir ../server_target

run:
	./server_target/release/server