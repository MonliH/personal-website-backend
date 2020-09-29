install:
	cd www && \
	npm i
	cd blog_processing && \
	npm i

build_frontend:
	cd www && \
	npm run build

build_backend:
	node blog_processing
	cd server && \
	cargo build --release --target-dir ../server_target

run:
	./server_target/release/server
