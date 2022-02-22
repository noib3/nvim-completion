build:
	@cargo build
	@rm -rf ./lua/compleet.so
	@cp ./target/debug/libcompleet.so ./lua/compleet.so
	@mkdir -p ./lua/deps
	@cp ./target/debug/deps/*.rlib ./lua/deps
