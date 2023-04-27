install: build
	if [ -d "/usr/local/bin/" ]; then \
	 	cp target/release/ctp_check $(HOME)/.local/bin; \
	else \
		mkdir /usr/local/bin; \
		cp target/release/ctp_check $(HOME)/.local/bin/; \
	fi

build:
	cargo build --release

clean:
	rm -rvf target/release;

uninstall:
	rm $(HOME)/.local/bin/ctp_check;