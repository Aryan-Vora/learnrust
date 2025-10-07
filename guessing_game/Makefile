All: clean format

help:
	@echo -n "Valid commands are: "; tput setaf 2; echo -n "make clean, make format, make exec"; tput sgr0; echo

clean:
	@echo "Cleaning build artifacts..."
	@cargo clean

format:
	@echo "Formatting Rust code..."
	@cargo fmt

exec:
	@echo "Building and running Rust project..."
	@cargo run -- $(ARGS)
