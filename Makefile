build:
	cargo build

install:
	# debug path for now. Should be configured in future
	mkdir -p .rocks/lib/tarantool
	yes | cp -rf target/debug/libbrodrust.so .rocks/lib/tarantool/brod.so