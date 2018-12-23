SHELL=/bin/zsh
build:
	cargo build --release

install: build
	install target/release/*(.x) $(HOME)/local/lib/sys/info
	
